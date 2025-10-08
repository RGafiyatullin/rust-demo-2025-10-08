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
pub enum ProcessDepositError {}

#[derive(Debug, thiserror::Error)]
pub enum ProcessWithdrawalError {}

#[derive(Debug, thiserror::Error)]
pub enum ProcessDisputeError {}

#[derive(Debug, thiserror::Error)]
pub enum ProcessResolveError {}

#[derive(Debug, thiserror::Error)]
pub enum ProcessChargebackError {}
