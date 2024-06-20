use super::entity_with_mbid::EntityWithMBID;
use super::generic_mbid::MBIDSpe;
use super::generic_mbid::NaiveID;
use super::generic_mbid::PrimaryID;
use derive_more::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, From, Deserialize, Serialize, PartialEq, Eq, Clone, IsVariant, Unwrap)]
pub enum MBIDAnyState<T: EntityWithMBID> {
    Naive(MBIDSpe<T, NaiveID>),
    Primary(MBIDSpe<T, PrimaryID>),
}
