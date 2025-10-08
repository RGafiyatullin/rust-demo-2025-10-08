use std::collections::BTreeMap;

use test_case::test_case;

use crate::{engine::Engine, input::Tx};

#[test_case([]; "baseline")]
fn process_transactions(transactions: impl IntoIterator<Item = Tx>) {
    let case_name = std::thread::current()
        .name()
        .unwrap()
        .to_owned()
        .replace("::", "-");

    let mut engine = Engine::default();
    let mut transcript = vec![];

    for tx in transactions {
        let outcome = engine.process_tx(tx.clone());
        transcript.push((tx, outcome));
    }

    insta::with_settings!({
        snapshot_path => "cases",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_debug_snapshot!(case_name,
            (
                transcript,
                engine.balances.into_iter().collect::<BTreeMap<_,_>>(),
            ),
        );
    });
}

mod t {
    use crate::{
        input::{Tx, TxDeposit, TxKind, TxWithdrawal},
        types::{Amount, ClientId, PositiveAmount, TxId},
    };

    pub(super) fn d(client_id: ClientId, tx_id: TxId, amount_deposited: &str) -> Tx {
        let amount_deposited =
            PositiveAmount::try_from(Amount::from_str_exact(amount_deposited).unwrap()).unwrap();
        Tx {
            client_id,
            tx_id,
            kind: TxKind::Deposit(TxDeposit { amount_deposited }),
        }
    }

    pub(super) fn w(client_id: ClientId, tx_id: TxId, amount_withdrawn: &str) -> Tx {
        let amount_withdrawn =
            PositiveAmount::try_from(Amount::from_str_exact(amount_withdrawn).unwrap()).unwrap();
        Tx {
            client_id,
            tx_id,
            kind: TxKind::Withdrawal(TxWithdrawal { amount_withdrawn }),
        }
    }

    pub(crate) fn di(client_id: ClientId, tx_id: TxId) -> Tx {
        Tx {
            client_id,
            tx_id,
            kind: TxKind::Dispute,
        }
    }
    pub(crate) fn re(client_id: ClientId, tx_id: TxId) -> Tx {
        Tx {
            client_id,
            tx_id,
            kind: TxKind::Resolve,
        }
    }
    pub(crate) fn cb(client_id: ClientId, tx_id: TxId) -> Tx {
        Tx {
            client_id,
            tx_id,
            kind: TxKind::Chargeback,
        }
    }
}
