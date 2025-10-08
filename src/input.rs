//! Data types to process input: transaction and its parts.

use crate::types::{ClientId, PositiveAmount, TxId};

/// A transaction of any supported kind.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Tx {
    /// Client this transaction relates to.
    #[serde(rename = "client")]
    pub client_id: ClientId,

    /// Globally unique transaction id.
    #[serde(rename = "tx")]
    pub tx_id: TxId,

    /// See [`TxKind`].
    #[serde(flatten)]
    pub kind: TxKind,
}

/// Data specific to the transaction kind.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
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
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TxDeposit {
    /// Amount being deposited
    #[serde(rename = "amount")]
    pub amount_deposited: PositiveAmount,
}

/// Take funds from the account.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TxWithdrawal {
    /// Amount to withdraw
    #[serde(rename = "amount")]
    pub amount_withdrawn: PositiveAmount,
}

#[cfg(test)]
mod tests;
