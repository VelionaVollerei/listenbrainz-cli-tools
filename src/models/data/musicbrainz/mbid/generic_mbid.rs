use serde::Deserialize;
use serde::Serialize;

use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use std::marker::PhantomData;
use std::ops::Deref;

use super::entity_with_mbid::EntityWithMBID;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct MBIDSpe<T: EntityWithMBID, S: IdAliasState> {
    id: String,

    _entity_type: PhantomData<T>,
    _state: PhantomData<S>,
}

// Id state
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NaiveID;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrimaryID;

pub trait IdAliasState: Clone {}
impl IdAliasState for NaiveID {}
impl IdAliasState for PrimaryID {}

/// `MBIDSpe`'s Common Methods that change depending on the type and state
pub trait MBIDSpeMethods<T: EntityWithMBID, S: IdAliasState> {}

/// `MBIDSpe`'s Common Methods that change depending on the type only
pub trait MBIDSpeTypeMethods<T: EntityWithMBID> {}

/// `MBIDSpe`'s Common Methods that change depending on the state only
pub trait MBIDSpeStateMethods<S: IdAliasState> {}

impl<T: EntityWithMBID, S: IdAliasState> Deref for MBIDSpe<T, S> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T, S> From<String> for MBIDSpe<T, S>
where
    T: EntityWithMBID,
    S: IdAliasState,
{
    fn from(value: String) -> Self {
        Self {
            id: value,
            _entity_type: PhantomData,
            _state: PhantomData,
        }
    }
}
