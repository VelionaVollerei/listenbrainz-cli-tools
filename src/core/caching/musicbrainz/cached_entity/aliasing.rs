use crate::core::caching::serde_cacache;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::error::Error;

use super::CachedEntity;

impl<V> CachedEntity<V>
where
    V: MusicBrainzEntity,
{
    /// Return the primary id of the contained entity
    pub async fn get_or_load_primary_id(&mut self) -> Result<Option<PrimaryMBID<V>>, Error> {
        let entity = self.get_or_load().await?;

        // Grab the mbid from the loaded entity
        Ok(entity.map(|ent| ent.get_mbid()))
    }

    /// Return the primary id of the contained entity
    pub async fn get_load_or_fetch_primary_id(&mut self) -> Result<PrimaryMBID<V>, Error> {
        let entity = self.get_load_or_fetch().await?;

        // Grab the mbid from the loaded entity
        Ok(entity.get_mbid())
    }

    /// This function return the id of the value to load/fetch.
    ///
    /// If the key of the current entity isn't the same as the id to fetch, then it will return a error.
    ///
    /// If it is unknown whether the key is the correct one, it will return it
    pub async fn get_verified_id(&mut self) -> Result<PrimaryMBID<V>, Error> {
        let id = self
            .get_or_load_primary_id()
            .await?
            .unwrap_or_else(|| PrimaryMBID::from(self.naive_id.to_string()));

        if id.to_string() == self.naive_id.to_string() {
            return Ok(id);
        }

        Err(Error::MBIDRedirectError(id.to_string()))
    }
}
