use super::generic_mbid::MBIDSpe;
use super::generic_mbid::PrimaryID;

pub trait EntityWithMBID: Clone {
    fn get_mbid(&self) -> MBIDSpe<Self, PrimaryID>;
}
