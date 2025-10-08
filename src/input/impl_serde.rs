//! This module contains a separate set of structures do deserialize
//! transactions.
//!
//! The necessity for such seemingly redundant approach emerged due to the way
//! `serde(flatten)` works: it forces `deserialize_any`-path, thus eagerly
//! treats as floats/integers those fields, whose data looks like such. Treating
//! decimal values as floats can cause precision problems, it generally should
//! be avoided.
//!
//! IDDQD: https://chatgpt.com/share/68e52644-bea4-800f-ae3e-47cec9dbbb66

use serde::Deserialize;

use crate::{
    input::{Tx, TxDeposit, TxKind, TxWithdrawal},
    types::{ClientId, PositiveAmount, TxId},
};

#[derive(serde::Deserialize)]
struct T {
    #[serde(rename = "type")]
    kind: K,
    #[serde(rename = "client")]
    client_id: ClientId,
    #[serde(rename = "tx")]
    tx_id: TxId,
    #[serde(rename = "amount")]
    amount_opt: Option<PositiveAmount>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum K {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl<'de> Deserialize<'de> for Tx {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;

        let T {
            kind,
            client_id,
            tx_id,
            amount_opt: amount,
        } = Deserialize::deserialize(deserializer)?;

        let kind = match (kind, amount) {
            (K::Deposit, Some(amount_deposited)) => TxKind::Deposit(TxDeposit { amount_deposited }),
            (K::Withdrawal, Some(amount_withdrawn)) => {
                TxKind::Withdrawal(TxWithdrawal { amount_withdrawn })
            }
            (K::Deposit | K::Withdrawal, None) => {
                Err(D::Error::custom("field `amount` is missing"))?
            }
            (K::Dispute, _) => TxKind::Dispute,
            (K::Resolve, _) => TxKind::Resolve,
            (K::Chargeback, _) => TxKind::Chargeback,
        };

        Ok(Self {
            client_id,
            tx_id,
            kind,
        })
    }
}
