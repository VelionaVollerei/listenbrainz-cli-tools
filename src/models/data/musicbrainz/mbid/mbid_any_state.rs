use serde::Deserialize;
use serde::Serialize;

use super::generic_mbid::EntityWithMBID;
use super::generic_mbid::MBIDSpe;
use super::generic_mbid::NaiveID;
use super::generic_mbid::PrimaryID;
use derive_more::*;

#[derive(Debug, From, Deserialize, Serialize, PartialEq, Eq, Clone, IsVariant, Unwrap)]
pub enum MBIDAnyState<T: EntityWithMBID> {
    Naive(MBIDSpe<T, NaiveID>),
    Primary(MBIDSpe<T, PrimaryID>),
}
