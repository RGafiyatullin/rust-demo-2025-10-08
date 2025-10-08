//! Error types

use fixnum::ArithmeticError;

use crate::types::{Amount, ClientId, TxId};

/// An error processing a transaction of any supported kind.
#[derive(Debug, thiserror::Error)]
pub enum ProcessTxError {
    /// See [`ProcessDepositError`]
    #[error("{}", _0)]
    Deposit(
        #[from]
        #[source]
        ProcessDepositError,
    ),

    /// See [`ProcessWithdrawalError`]
    #[error("{}", _0)]
    Withdrawal(
        #[from]
        #[source]
        ProcessWithdrawalError,
    ),

    /// See [`ProcessDisputeError`]
    #[error("{}", _0)]
    Dispute(
        #[from]
        #[source]
        ProcessDisputeError,
    ),

    /// See [`ProcessResolveError`]
    #[error("{}", _0)]
    Resolve(
        #[from]
        #[source]
        ProcessResolveError,
    ),

    /// See [`ProcessChargebackError`]
    #[error("{}", _0)]
    Chargeback(
        #[from]
        #[source]
        ProcessChargebackError,
    ),
}

/// An error processing deposit-transaction
#[derive(Debug, thiserror::Error)]
pub enum ProcessDepositError {
    /// See [`DuplicateTxId`]
    #[error("{}", _0)]
    DuplicateTxId(
        #[from]
        #[source]
        DuplicateTxId,
    ),

    /// An arithmetic error during the balance calculation.
    #[error("Arithmetic error: {}", _0)]
    Overflow(
        #[from]
        #[source]
        ArithmeticError,
    ),
}

/// An error processing withdrawal-transaction
#[derive(Debug, thiserror::Error)]
pub enum ProcessWithdrawalError {
    /// See [`DuplicateTxId`]
    #[error("{}", _0)]
    DuplicateTxId(
        #[from]
        #[source]
        DuplicateTxId,
    ),

    /// An arithmetic error during the balance calculation.
    #[error("Arithmetic error: {}", _0)]
    Overflow(
        #[from]
        #[source]
        ArithmeticError,
    ),

    /// See [`AccountLocked`]
    #[error("{}", _0)]
    AccountLocked(
        #[from]
        #[source]
        AccountLocked,
    ),

    /// The client does not have enough available funds to complete the
    /// requested withdrwal.
    #[error("Insufficient funds: {} has {}", _0, _1)]
    InsufficientFunds(ClientId, Amount),
}

/// An error processing dispute-transaction
#[derive(Debug, thiserror::Error)]
pub enum ProcessDisputeError {
    /// See [`UnknownTxId`]
    #[error("{}", _0)]
    UnknownTxId(
        #[from]
        #[source]
        UnknownTxId,
    ),

    /// See [`UnexpectedTxState`]
    #[error("{}", _0)]
    UnexpectedTxState(
        #[from]
        #[source]
        UnexpectedTxState,
    ),

    /// An arithmetic error during the balance calculation.
    #[error("Arithmetic error: {}", _0)]
    Overflow(
        #[from]
        #[source]
        ArithmeticError,
    ),
}

/// An error processing resolve-transaction
#[derive(Debug, thiserror::Error)]
pub enum ProcessResolveError {
    /// See [`UnknownTxId`]
    #[error("{}", _0)]
    UnknownTxId(
        #[from]
        #[source]
        UnknownTxId,
    ),

    /// See [`UnexpectedTxState`]
    #[error("{}", _0)]
    UnexpectedTxState(
        #[from]
        #[source]
        UnexpectedTxState,
    ),
}

/// An error processing chargeback-transaction
#[derive(Debug, thiserror::Error)]
pub enum ProcessChargebackError {
    /// See [`UnknownTxId`]
    #[error("{}", _0)]
    UnknownTxId(
        #[from]
        #[source]
        UnknownTxId,
    ),

    /// See [`UnexpectedTxState`]
    #[error("{}", _0)]
    UnexpectedTxState(
        #[from]
        #[source]
        UnexpectedTxState,
    ),
}

/// Transaction was rejected due to having a non-unique tx-id.
#[derive(Debug, thiserror::Error)]
#[error("duplicate tx-id: {}", _0)]
pub struct DuplicateTxId(pub TxId);

/// No transaction corresponds to the specified tx-id.
#[derive(Debug, thiserror::Error)]
#[error("unknown tx-id: {}", _0)]
pub struct UnknownTxId(pub TxId);

/// The refered transaction's state is incompatible with the requested
/// operation.
#[derive(Debug, thiserror::Error)]
#[error("unexpected transaction state")]
pub struct UnexpectedTxState;

/// Transaction was rejected because the account it refers is locked.
#[derive(Debug, thiserror::Error)]
#[error("account locked: {}", _0)]
pub struct AccountLocked(pub ClientId);
