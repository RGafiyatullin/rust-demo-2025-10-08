//! Processing transactions and keeping the balances.

use std::collections::{HashMap, hash_map::Entry::*};

use crate::{
    input::{Tx, TxDeposit, TxKind, TxWithdrawal},
    types::{Amount, ClientId, NonNegativeAmount, PositiveAmount, TxId},
};

pub mod errors;

use errors::*;
use fixnum::ops::{CheckedAdd, CheckedSub};

/// Engine keeps balances, and changes them according to the processed
/// transactions.
#[derive(Debug, Default)]
pub struct Engine {
    balances: HashMap<ClientId, Balance>,
    transactions: HashMap<TxId, TxState>,
}

#[derive(Debug, Default)]
struct Balance {
    deposited: NonNegativeAmount,
    withdrawn: NonNegativeAmount,

    disputed: NonNegativeAmount,
    resolved: NonNegativeAmount,
    chargedback: NonNegativeAmount,
}

#[derive(Debug, Clone, Copy)]
enum TxState {
    Deposited {
        amount_deposited: PositiveAmount,
        client_id: ClientId,
    },
    Withdrawn,
    #[allow(dead_code)] // XXX
    Disputed {
        amount_disputed: PositiveAmount,
        client_id: ClientId,
    },
}

impl Engine {
    /// Process a single transaction.
    pub fn process_tx(&mut self, tx: Tx) -> Result<(), ProcessTxError> {
        let Tx {
            client_id,
            tx_id,
            kind,
        } = tx;
        match kind {
            TxKind::Deposit(deposit) => self.process_deposit(client_id, tx_id, deposit)?,
            TxKind::Withdrawal(withdrawal) => {
                self.process_withdrawal(client_id, tx_id, withdrawal)?
            }
            TxKind::Dispute => self.process_dispute(client_id, tx_id)?,
            TxKind::Resolve => self.process_resolve(client_id, tx_id)?,
            TxKind::Chargeback => self.process_chargeback(client_id, tx_id)?,
        }

        Ok(())
    }

    fn process_deposit(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
        deposit: TxDeposit,
    ) -> Result<(), ProcessDepositError> {
        let TxDeposit { amount_deposited } = deposit;
        let Vacant(tx) = self.transactions.entry(tx_id) else {
            return Err(DuplicateTxId(tx_id).into());
        };
        let balance = self.balances.entry(client_id).or_default();

        balance.deposited = {
            let total_deposited: Amount = balance.deposited.into();
            let amount_deposited: Amount = amount_deposited.into();
            total_deposited.cadd(amount_deposited)?.try_into().expect(
                "sum of a non-negative and a positive, overflow handled; should be positive",
            )
        };
        tx.insert(TxState::Deposited {
            amount_deposited,
            client_id,
        });

        Ok(())
    }

    fn process_withdrawal(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
        withdrawal: TxWithdrawal,
    ) -> Result<(), ProcessWithdrawalError> {
        let TxWithdrawal { amount_withdrawn } = withdrawal;
        let (mut balance, tx) = match (
            self.balances.entry(client_id),
            self.transactions.entry(tx_id),
        ) {
            (_, Occupied(_)) => return Err(DuplicateTxId(tx_id).into()),
            (Occupied(balance), Vacant(_)) if balance.get().is_locked() => {
                return Err(AccountLocked(client_id).into());
            }

            (Vacant(_), _) => {
                return Err(ProcessWithdrawalError::InsufficientFunds(
                    client_id,
                    Default::default(),
                ));
            }
            (Occupied(balance), Vacant(_))
                if Amount::from(balance.get().available()) < Amount::from(amount_withdrawn) =>
            {
                return Err(ProcessWithdrawalError::InsufficientFunds(
                    client_id,
                    balance.get().available(),
                ));
            }

            (Occupied(balance), Vacant(tx)) => (balance, tx),
        };

        assert!(Amount::from(balance.get().available()) >= Amount::from(amount_withdrawn));
        assert!(!balance.get().is_locked());

        balance.get_mut().withdrawn = {
            let total_withdrawn: Amount = balance.get().withdrawn.into();
            let amount_withdrawn: Amount = amount_withdrawn.into();
            total_withdrawn.cadd(amount_withdrawn)?.try_into().expect(
                "sum of a non-negative and a positive, overflow handled; should be positive",
            )
        };
        tx.insert(TxState::Withdrawn);

        if balance.get().can_be_pruned() {
            let _ = balance.remove();
        }

        Ok(())
    }

    fn process_dispute(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
    ) -> Result<(), ProcessDisputeError> {
        let transaction = self
            .transactions
            .get_mut(&tx_id)
            .ok_or(UnknownTxId(tx_id))?;
        let TxState::Deposited {
            amount_deposited: amount_disputed,
            client_id: expected_client_id,
        } = *transaction
        else {
            return Err(UnexpectedTxState.into());
        };

        if client_id != expected_client_id {
            return Err(UnexpectedTxState.into());
        }

        let balance = self.balances.entry(client_id).or_default();
        balance.disputed = {
            let total_disputed: Amount = balance.disputed.into();
            let amount_disputed: Amount = amount_disputed.into();

            total_disputed.cadd(amount_disputed)?.try_into().expect(
                "sum of a non-negative and a positive, overflow handled; should be positive",
            )
        };
        *transaction = TxState::Disputed {
            client_id,
            amount_disputed,
        };

        Ok(())
    }

    fn process_resolve(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
    ) -> Result<(), ProcessResolveError> {
        let transaction = self
            .transactions
            .get_mut(&tx_id)
            .ok_or(UnknownTxId(tx_id))?;
        let TxState::Disputed {
            amount_disputed,
            client_id: expected_client_id,
        } = *transaction
        else {
            return Err(UnexpectedTxState.into());
        };
        if client_id != expected_client_id {
            return Err(UnexpectedTxState.into());
        }

        let Occupied(mut balance) = self.balances.entry(client_id) else {
            panic!("disputed account shouldn't have been pruned")
        };

        balance.get_mut().resolved = {
            let total_resolved: Amount = balance.get().resolved.into();
            let amount_disputed: Amount = amount_disputed.into();

            total_resolved.cadd(amount_disputed)?.try_into().expect(
                "sum of a non-negative and a positive, overflow handled; should be positive",
            )
        };
        *transaction = TxState::Deposited {
            amount_deposited: amount_disputed,
            client_id,
        };

        if balance.get().can_be_pruned() {
            let _ = balance.remove();
        }

        Ok(())
    }

    fn process_chargeback(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
    ) -> Result<(), ProcessChargebackError> {
        let Occupied(transaction) = self.transactions.entry(tx_id) else {
            return Err(UnknownTxId(tx_id).into());
        };
        let TxState::Disputed {
            amount_disputed,
            client_id: expected_client_id,
        } = *transaction.get()
        else {
            return Err(UnexpectedTxState.into());
        };
        if client_id != expected_client_id {
            return Err(UnexpectedTxState.into());
        }

        let balance = self
            .balances
            .get_mut(&client_id)
            .expect("disputed account shouldn't have been pruned");
        balance.chargedback = {
            let total_chargedback: Amount = balance.chargedback.into();
            let amount_disputed: Amount = amount_disputed.into();

            total_chargedback.cadd(amount_disputed)?.try_into().expect(
                "sum of a non-negative and a positive, overflow handled; should be positive",
            )
        };
        let _ = transaction.remove();

        Ok(())
    }
}

impl Balance {
    fn available(&self) -> Amount {
        let de: Amount = self.deposited.into();
        let wi: Amount = self.withdrawn.into();
        let di: Amount = self.disputed.into();
        let re: Amount = self.resolved.into();

        de // deposit should increase available funds
            .saturating_sub(wi) // withdrawal should decrease available funds
            .saturating_sub(di) // available funds decrease by the amount disputed
            .saturating_add(re) // available funds increase by the amount resolved
    }

    fn held(&self) -> NonNegativeAmount {
        let di: Amount = self.disputed.into();
        let re: Amount = self.resolved.into();
        let ch: Amount = self.chargedback.into();

        di // held funds increase upon dispute
            .saturating_sub(re) // held funds decrease by the amount resolved
            .saturating_sub(ch) // held funds decrease by the amount charged back
            .try_into()
            .expect("held funds must not be negative")
    }

    fn total(&self) -> Amount {
        let de: Amount = self.deposited.into();
        let wi: Amount = self.withdrawn.into();
        let ch: Amount = self.chargedback.into();

        de // deposit should increase total funds
            .saturating_sub(wi) // withdrawal should decrease total funds
            // total funds are unaffected by disputes
            // total funds are unaffected by resolves
            .saturating_sub(ch) // total funds decrease by the amount charged back
    }

    fn is_locked(&self) -> bool {
        Amount::from(self.chargedback).signum() > 0
    }

    fn can_be_pruned(&self) -> bool {
        !self.is_locked()
            && Amount::from(self.held()).signum() == 0
            && Amount::from(self.total()).signum() == 0
    }
}

#[cfg(test)]
mod tests;
