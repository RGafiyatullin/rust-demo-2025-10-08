use crate::types::{Amount, ClientId, NonNegativeAmount};

/// A serde-serializable account entry
#[derive(Debug, Clone, serde::Serialize)]
pub struct Account {
    /// client-id
    #[serde(rename = "client")]
    pub client_id: ClientId,
    /// total funds that are available for trading, stacking, withdrawal, etc.
    pub available: Amount,
    /// total funds that are held for dispute.
    pub held: NonNegativeAmount,
    /// total funds that are available or held.
    pub total: Amount,
    /// shows whether the account is locked.
    #[serde(rename = "locked")]
    pub is_locked: bool,
}
