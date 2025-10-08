//! Basic types used in this crate.

use fixnum::{FixedPoint, typenum};

/// Client ID
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize
)]
pub struct ClientId(u16);

/// Transaction ID
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize
)]
pub struct TxId(u32);

/// Fixed point number to keep amounts: precision — 4 digits past the decimal
/// point.
pub type Amount = FixedPoint<i64, typenum::U4>;

/// Amount that can only be positive.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PositiveAmount(
    // TODO: ensure it is indeed positive (TryFrom<Amount> + serde::Deserialize)
    Amount,
);
