use super::generic_mbid::EntityWithMBID;
use super::generic_mbid::MBIDSpe;
use super::generic_mbid::PrimaryID;

/// Enum representing either a MBID or an Entity
pub enum MBIDOrEntity<T: EntityWithMBID> {
    MBID(MBIDSpe<T, PrimaryID>),
    Entity(T),
}
