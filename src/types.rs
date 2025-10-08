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
    // FIXME: ensure the deserialized value is indeed positive.
    Amount,
);

/// Amount that cannot be negative.
#[derive(
    Default,
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
pub struct NonNegativeAmount(
    // FIXME: ensure the deserialized value is indeed non-negative.
    Amount,
);

mod non_negative_amount {
    use std::fmt;

    use super::*;

    #[derive(Debug, thiserror::Error)]
    #[error("expected non-negative amount; got: {}", _0)]
    pub struct NegativeAmount(Amount);

    impl TryFrom<Amount> for NonNegativeAmount {
        type Error = NegativeAmount;

        fn try_from(amount: Amount) -> Result<Self, Self::Error> {
            if amount.signum() < 0 {
                return Err(NegativeAmount(amount));
            }

            Ok(Self(amount))
        }
    }

    impl From<NonNegativeAmount> for Amount {
        fn from(non_negative: NonNegativeAmount) -> Self {
            let NonNegativeAmount(amount) = non_negative;
            amount
        }
    }

    impl fmt::Display for NonNegativeAmount {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }
}

mod positive_amount {
    use std::fmt;

    use super::*;
    #[derive(Debug, thiserror::Error)]
    #[error("expected positive amount; got: {}", _0)]
    pub struct NonPositiveAmount(Amount);

    impl TryFrom<Amount> for PositiveAmount {
        type Error = NonPositiveAmount;

        fn try_from(amount: Amount) -> Result<Self, Self::Error> {
            if amount.signum() <= 0 {
                return Err(NonPositiveAmount(amount));
            }

            Ok(Self(amount))
        }
    }

    impl From<PositiveAmount> for Amount {
        fn from(positive: PositiveAmount) -> Self {
            let PositiveAmount(amount) = positive;
            amount
        }
    }

    impl fmt::Display for PositiveAmount {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }
}

mod client_id {
    use std::fmt;

    use crate::types::ClientId;

    impl fmt::Display for ClientId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "C:{}", self.0)
        }
    }

    #[cfg(test)]
    impl From<u16> for ClientId {
        fn from(id: u16) -> Self {
            Self(id)
        }
    }
}

mod tx_id {
    use std::fmt;

    use crate::types::TxId;

    impl fmt::Display for TxId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "T:{}", self.0)
        }
    }

    #[cfg(test)]
    impl From<u32> for TxId {
        fn from(id: u32) -> Self {
            Self(id)
        }
    }
}
