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
    total: NonNegativeAmount,
    held: NonNegativeAmount,

    is_locked: bool,
}

#[derive(Debug, Clone, Copy)]
enum TxState {
    #[allow(dead_code)] // XXX
    Deposited {
        amount_deposited: PositiveAmount,
        client_id: ClientId,
    },
    Withdrawn,
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

        let new_total: NonNegativeAmount = {
            let total: Amount = balance.total.into();
            let amount_deposited: Amount = amount_deposited.into();
            total.cadd(amount_deposited)?.try_into().expect(
                "sum of a non-negative and a positive, overflow handled; should be positive",
            )
        };

        balance.total = new_total;
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
            (Occupied(balance), Vacant(_)) if balance.get().is_locked => {
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
        assert!(!balance.get().is_locked);

        let new_total: NonNegativeAmount = {
            let total: Amount = balance.get().total.into();
            let amount_withdrawn: Amount = amount_withdrawn.into();
            total
                .csub(amount_withdrawn)?
                .try_into()
                .expect("relying on available not to be greater than total")
        };

        balance.get_mut().total = new_total;
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
        let _ = (client_id, tx_id);
        unimplemented!()
    }

    fn process_resolve(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
    ) -> Result<(), ProcessResolveError> {
        let _ = (client_id, tx_id);
        unimplemented!()
    }

    fn process_chargeback(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
    ) -> Result<(), ProcessChargebackError> {
        let _ = (client_id, tx_id);
        unimplemented!()
    }
}

impl Balance {
    fn available(&self) -> NonNegativeAmount {
        let t: Amount = self.total.into();
        let h: Amount = self.held.into();

        NonNegativeAmount::try_from(t.saturating_sub(h)).unwrap_or_default()
    }

    fn can_be_pruned(&self) -> bool {
        !self.is_locked
            && Amount::from(self.held).signum() == 0
            && Amount::from(self.total).signum() == 0
    }
}

#[cfg(test)]
mod tests;
