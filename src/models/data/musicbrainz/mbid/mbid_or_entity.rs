use super::generic_mbid::MBIDSpe;
use super::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::mbid::entity_with_mbid::EntityWithMBID;

/// Enum representing either a MBID or an Entity
pub enum MBIDOrEntity<T: EntityWithMBID> {
    MBID(MBIDSpe<T, PrimaryID>),
    Entity(T),
}
