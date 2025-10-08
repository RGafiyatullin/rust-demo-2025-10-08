//! Data types to process input: transaction and its parts.

use crate::types::{ClientId, PositiveAmount, TxId};

mod impl_serde;

/// A transaction of any supported kind.
#[derive(Debug, Clone)]
pub struct Tx {
    /// Client this transaction relates to.
    pub client_id: ClientId,

    /// Globally unique transaction id.
    pub tx_id: TxId,

    /// See [`TxKind`].
    pub kind: TxKind,
}

/// Data specific to the transaction kind.
#[derive(Debug, Clone)]
pub enum TxKind {
    /// See [`TxDeposit`].
    Deposit(TxDeposit),
    /// See [`TxWithdrawal`].
    Withdrawal(TxWithdrawal),
    /// Initiate a dispute.
    Dispute,
    /// Cancel the previously raised dispute: unhold the disputed funds.
    Resolve,
    /// Withdraw the disputed funds.
    Chargeback,
}

/// Put funds into the account.
#[derive(Debug, Clone)]
pub struct TxDeposit {
    /// Amount being deposited
    pub amount_deposited: PositiveAmount,
}

/// Take funds from the account.
#[derive(Debug, Clone)]
pub struct TxWithdrawal {
    /// Amount to withdraw
    pub amount_withdrawn: PositiveAmount,
}

#[cfg(test)]
mod tests;
