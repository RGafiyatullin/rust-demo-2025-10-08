use std::collections::BTreeMap;

use test_case::test_case;

use crate::{engine::Engine, input::Tx};

#[test_case([]; "baseline")]
#[test_case([
    t::d(1,1,"1.0"),
]; "case-01")]
#[test_case([
    t::w(1, 1, "1.0"),
]; "case-02")]
#[test_case([
    t::d(1, 1, "1.0"),
    t::w(1, 1, "0.9"),
]; "case-03")]
#[test_case([
    t::d(1, 1, "1.0"),
    t::w(1, 2, "0.5"),
]; "case-04.a")]
#[test_case([
    t::d(1, 1, "1.0"),
    t::w(1, 2, "1.0"),
]; "case-04.b")]
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
        transcript.push((format!("{:?}", tx), outcome.map_err(|e| e.to_string())));
    }

    insta::with_settings!({
        snapshot_path => "cases",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_yaml_snapshot!(case_name,
            (
                transcript,
                engine.balances.into_iter()
                    .map(|(client_id, balance)|
                        (client_id, (balance.available(), balance.held(), balance.total(), balance.is_locked()))
                    )
                    .collect::<BTreeMap<_,_>>(),
            ),
        );
    });
}

mod t {
    use crate::{
        input::{Tx, TxDeposit, TxKind, TxWithdrawal},
        types::{Amount, PositiveAmount},
    };

    pub(super) fn d(client_id: u16, tx_id: u32, amount_deposited: &str) -> Tx {
        let client_id = client_id.into();
        let tx_id = tx_id.into();
        let amount_deposited =
            PositiveAmount::try_from(Amount::from_str_exact(amount_deposited).unwrap()).unwrap();
        Tx {
            client_id,
            tx_id,
            kind: TxKind::Deposit(TxDeposit { amount_deposited }),
        }
    }

    pub(super) fn w(client_id: u16, tx_id: u32, amount_withdrawn: &str) -> Tx {
        let client_id = client_id.into();
        let tx_id = tx_id.into();
        let amount_withdrawn =
            PositiveAmount::try_from(Amount::from_str_exact(amount_withdrawn).unwrap()).unwrap();
        Tx {
            client_id,
            tx_id,
            kind: TxKind::Withdrawal(TxWithdrawal { amount_withdrawn }),
        }
    }

    // pub(crate) fn di(client_id: u16, tx_id: u32) -> Tx {
    //     let client_id = client_id.into();
    //     let tx_id = tx_id.into();
    //     Tx {
    //         client_id,
    //         tx_id,
    //         kind: TxKind::Dispute,
    //     }
    // }
    // pub(crate) fn re(client_id: u16, tx_id: u32) -> Tx {
    //     let client_id = client_id.into();
    //     let tx_id = tx_id.into();
    //     Tx {
    //         client_id,
    //         tx_id,
    //         kind: TxKind::Resolve,
    //     }
    // }
    // pub(crate) fn cb(client_id: u16, tx_id: u32) -> Tx {
    //     let client_id = client_id.into();
    //     let tx_id = tx_id.into();
    //     Tx {
    //         client_id,
    //         tx_id,
    //         kind: TxKind::Chargeback,
    //     }
    // }
}
