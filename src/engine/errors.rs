use fixnum::ArithmeticError;

use crate::types::{ClientId, NonNegativeAmount, TxId};

#[derive(Debug, thiserror::Error)]
pub enum ProcessTxError {
    #[error("{}", _0)]
    Deposit(
        #[from]
        #[source]
        ProcessDepositError,
    ),

    #[error("{}", _0)]
    Withdrawal(
        #[from]
        #[source]
        ProcessWithdrawalError,
    ),

    #[error("{}", _0)]
    Dispute(
        #[from]
        #[source]
        ProcessDisputeError,
    ),

    #[error("{}", _0)]
    Resolve(
        #[from]
        #[source]
        ProcessResolveError,
    ),

    #[error("{}", _0)]
    Chargeback(
        #[from]
        #[source]
        ProcessChargebackError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessDepositError {
    #[error("{}", _0)]
    DuplicateTxId(
        #[from]
        #[source]
        DuplicateTxId,
    ),

    #[error("Arithmetic error: {}", _0)]
    Overflow(
        #[from]
        #[source]
        ArithmeticError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessWithdrawalError {
    #[error("{}", _0)]
    DuplicateTxId(
        #[from]
        #[source]
        DuplicateTxId,
    ),

    #[error("Arithmetic error: {}", _0)]
    Overflow(
        #[from]
        #[source]
        ArithmeticError,
    ),

    #[error("{}", _0)]
    AccountLocked(
        #[from]
        #[source]
        AccountLocked,
    ),

    #[error("Insufficient funds: {} has {}", _0, _1)]
    InsufficientFunds(ClientId, NonNegativeAmount),
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessDisputeError {}

#[derive(Debug, thiserror::Error)]
pub enum ProcessResolveError {}

#[derive(Debug, thiserror::Error)]
pub enum ProcessChargebackError {}

#[derive(Debug, thiserror::Error)]
#[error("duplicate tx-id: {}", _0)]
pub struct DuplicateTxId(pub TxId);

#[derive(Debug, thiserror::Error)]
#[error("account locked: {}", _0)]
pub struct AccountLocked(pub ClientId);
