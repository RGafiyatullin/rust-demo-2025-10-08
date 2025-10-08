use std::collections::HashMap;
use std::collections::hash_map::Entry::*;

use crate::{
    input::{Tx, TxDeposit, TxKind, TxWithdrawal},
    types::{Amount, ClientId, NonNegativeAmount, PositiveAmount, TxId},
};

pub mod errors;

use errors::*;
use fixnum::ops::CheckedAdd;

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
    Deposited { amount_deposited: PositiveAmount, client_id: ClientId, },
}

impl Engine {
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
        let Vacant(tx) = self.transactions.entry(tx_id) else { return Err(DuplicateTxId(tx_id).into()) };
        let balance = self.balances.entry(client_id).or_default();

        let new_total: NonNegativeAmount = {
            let total: Amount = balance.total.into();
            let amount_deposited: Amount = amount_deposited.into();
            total.cadd(amount_deposited)?.try_into().expect("sum of a non-negative and a positive, overflow handled; should be positive")
        };

        balance.total = new_total;
        tx.insert(TxState::Deposited { amount_deposited, client_id, });

        Ok(())
    }

    fn process_withdrawal(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
        withdrawal: TxWithdrawal,
    ) -> Result<(), ProcessWithdrawalError> {
        unimplemented!()
    }

    fn process_dispute(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
    ) -> Result<(), ProcessDisputeError> {
        unimplemented!()
    }

    fn process_resolve(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
    ) -> Result<(), ProcessResolveError> {
        unimplemented!()
    }

    fn process_chargeback(
        &mut self,
        client_id: ClientId,
        tx_id: TxId,
    ) -> Result<(), ProcessChargebackError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests;
