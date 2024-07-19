use std::sync::Arc;

use tokio::sync::RwLockWriteGuard;

use crate::core::caching::serde_cacache;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::error::Error;
use crate::utils::println_cli_info;
use crate::utils::println_cli_warn;

use super::cached_entity::CachedEntity;
use super::musicbrainz_cache::MusicbrainzCache;

impl<V> MusicbrainzCache<V>
where
    V: MusicBrainzEntity + Eq,
{
    pub async fn set_new_alias(
        &self,
        alias: NaiveMBID<V>,
        main: NaiveMBID<V>,
    ) -> Result<(), Error> {
        self.alias_cache.set(&alias, &main).await?;
        self.alias_entry(alias, &main).await;
        Ok(())
    }

    /// Set a new alias for an entry. When getting id_a, it will now return id_primary's entry
    pub async fn alias_entry(&self, id_a: NaiveMBID<V>, id_primary: &NaiveMBID<V>) {
        let new_entry = self.get_entity_entry(&id_primary).await;
        self.cache_entities.write().await.insert(id_a, new_entry);
    }

    /// Get the cached primary id for an entity
    pub async fn get_or_load_primary_id(
        &self,
        mbid: &NaiveMBID<V>,
    ) -> Result<Option<PrimaryMBID<V>>, serde_cacache::Error> {
        let entry = self.get_entity_entry(mbid).await;

        if let Some(id) = entry.read().await.primary_id() {
            return Ok(Some(id.clone()));
        }

        let res = entry.write().await.get_or_load_primary_id().await;
        res // Fix: Borrow doesn't live long enough
    }
}
