use std::sync::Arc;

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz::mbid::state_id::MusicBrainzEntity;
use crate::models::error::Error;

impl<T, S> MBIDWithState<T, S>
where
    T: MusicBrainzEntity,
    S: MBIDState,
{
    pub fn get_entity_cache() -> Arc<MusicbrainzCache<T>> {
        T::get_cache()
    }

    pub async fn fetch_entity(&self) -> Result<ExternalMusicBrainzEntity, Error> {
        T::fetch(self).await
    }

    /// Turn the MBID into an untrusted state, consuming it
    pub fn into_naive(self) -> NaiveMBID<T> {
        NaiveMBID::from(self.id)
    }

    /// Get the matching entity from the cache, or fetch it
    pub async fn get_load_or_fetch(&self) -> color_eyre::Result<Arc<T>> {
        T::get_load_or_fetch(&self.clone().into_naive()).await // TODO: Remove into naive
    }

    pub async fn to_primary(&self) -> color_eyre::Result<PrimaryMBID<T>> {
        Ok(self.get_load_or_fetch().await?.get_mbid())
    }
}
