pub mod artist;
pub mod artist_credit;
pub mod entity;
pub mod external_musicbrainz_entity;
pub mod mbid;
pub mod musicbrainz_entity;
pub mod recording;
pub mod relation;
pub mod release;
pub mod release_group;
pub mod work;

pub trait HasId {
    fn get_id(&self) -> &str;
}

impl<T: HasMbid> HasId for T {
    fn get_id(&self) -> &str {
        self.get_mbid()
    }
}
