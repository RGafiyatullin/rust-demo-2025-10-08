//! Data types to process input: transaction and its parts.
//! 

use crate::types::{ClientId, PositiveAmount, TxId};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Tx {
    #[serde(rename = "client")]
    pub client_id: ClientId,

    #[serde(rename = "tx")]
    pub tx_id: TxId,

    #[serde(flatten)]
    pub kind: TxKind,
}


#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TxKind {
    Deposit(TxDeposit),
    Withdrawal(TxWithdrawal),
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TxDeposit {
    pub amount: PositiveAmount,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TxWithdrawal {
    pub amount: PositiveAmount,
}


#[cfg(test)]
mod tests;
