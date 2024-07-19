use std::sync::Arc;

use tokio::sync::RwLockWriteGuard;

use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::error::Error;
use crate::utils::println_cli_info;

use super::cached_entity::CachedEntity;
use super::musicbrainz_cache::MusicbrainzCache;

impl<V> MusicbrainzCache<V>
where
    V: MusicBrainzEntity + Eq,
{
    /// Fetch and overwrite the provided entry.
    async fn fetch_entry_lock<'a>(
        mut entry: RwLockWriteGuard<'a, CachedEntity<V>>,
    ) -> Result<Arc<V>, Error> {
        entry.fetch().await
    }

    /// **Get** from the loaded value, or **load** from the cache, or **fetch** from the MB database
    ///
    /// This does the operation as a blocking write. If the value is potentially loaded, a [`MusicbrainzCache::get()`] should be done first to prevent blocking concurent reads
    async fn get_load_or_fetch_as_write<'a>(
        mut entry: RwLockWriteGuard<'a, CachedEntity<V>>,
    ) -> Result<Arc<V>, Error> {
        if let Some(val) = entry.get() {
            return Ok(val);
        }

        match entry.load().await? {
            Some(val) => return Ok(val),
            None => {
                // #[cfg(debug_assertions)]
                // println_cli_info("Cache miss");
            }
        }

        entry.fetch().await
    }

    /// **Get** from the loaded value, or **load** from the cache, or **fetch** from the MB database
    ///
    /// If the provided MBID isn't correct, it will retry with the correct one.
    pub async fn get_load_or_fetch(&self, mbid: &NaiveMBID<V>) -> Result<Arc<V>, Error> {
        let entry = self.get_entity_entry(mbid).await;

        // Try getting the value as a soft read
        if let Some(val) = entry.read().await.get() {
            return Ok(val);
        }

        // Not there? Do a blocking write and fetch it
        match Self::get_load_or_fetch_as_write(entry.write().await).await {
            Ok(val) => Ok(val),

            // Uh oh, wrong id. Let's retry with the new one
            Err(Error::MBIDRedirectError(new_id)) => {
                let new_mbid = NaiveMBID::from(new_id);
                self.alias_entry(mbid.clone(), &new_mbid).await;
                self.get_load_or_fetch(&new_mbid).await
            }

            Err(err) => Err(err),
        }
    }
}
