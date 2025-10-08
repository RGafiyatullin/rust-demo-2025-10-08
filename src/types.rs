use fixnum::{typenum, FixedPoint};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClientId(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub struct TxId(u32);


pub type Amount = FixedPoint<i64, typenum::U4>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub struct PositiveAmount(/* TODO: ensure it is indeed positive (TryFrom<Amount> + serde::Deserialize) */Amount);
