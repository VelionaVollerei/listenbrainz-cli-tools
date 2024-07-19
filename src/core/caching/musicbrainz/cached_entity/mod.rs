pub mod aliasing;
use crate::core::caching::serde_cacache;
use crate::core::caching::serde_cacache::tidy::SerdeCacacheTidy;
use crate::models::data::musicbrainz::entity::any::any_musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;
use crate::models::error::Error;

use derive_getters::Getters;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::RwLockWriteGuard;

#[derive(Debug, Getters)]
pub struct CachedEntity<V>
where
    V: MusicBrainzEntity,
{
    naive_id: NaiveMBID<V>,
    loaded: Option<Arc<V>>,

    disk_cache: Arc<SerdeCacacheTidy<PrimaryMBID<V>, V>>,
    alias_cache: Arc<SerdeCacacheTidy<NaiveMBID<V>, NaiveMBID<V>>>,
}

impl<V> CachedEntity<V>
where
    V: MusicBrainzEntity,
{
    pub fn new(
        id: NaiveMBID<V>,
        disk_cache: Arc<SerdeCacacheTidy<PrimaryMBID<V>, V>>,
        alias_cache: Arc<SerdeCacacheTidy<NaiveMBID<V>, NaiveMBID<V>>>,
    ) -> Self {
        Self {
            alias_cache,
            disk_cache,
            naive_id: id,
            loaded: None,
        }
    }

    /// Get the contained value. If there is a different primary id to this one, try getting the other's
    pub fn get(&self) -> Option<Arc<V>> {
        self.loaded.clone()
    }

    /// **Get** from the loaded value, or **load** from the cache.
    ///
    /// This version create its own read lock in case of a **get**, and create a write lock in case of **load**.
    pub async fn get_or_load(&mut self) -> Result<Option<Arc<V>>, Error> {
        match &self.loaded {
            Some(val) => Ok(Some(val.clone())),
            None => self.load().await,
        }
    }

    /// **Get** from the loaded value, or **load** from the cache, or **fetch** from the MB database
    pub async fn get_load_or_fetch(&mut self) -> Result<Arc<V>, Error> {
        match &self.loaded {
            Some(val) => Ok(val.clone()),
            None => {
                if let Some(val) = self.load().await? {
                    return Ok(val.clone());
                }

                self.fetch().await
            }
        }
    }

    /// Read from the cache to **load** the value. This rewrite the loaded value.
    pub async fn load(&mut self) -> Result<Option<Arc<V>>, Error> {
        let id = self.get_verified_id().await?;

        let cached = self
            .disk_cache
            .get_or_option(&id)
            .await?
            .map(|val| Arc::new(val));

        self.loaded = cached.clone();

        Ok(cached)
    }
    /// Send a request to Musicbrainz to **fetch** the listen, and insert into self
    pub async fn fetch(&mut self) -> Result<Arc<V>, Error> {
        let fetch_result = self.get_verified_id().await?.fetch_entity().await?;
        let converted_fetch = fetch_result.flattened_any();

        // First, process the main entity
        let main_entity = V::try_from_any(&converted_fetch.0)?;

        self.alias_cache
            .set(&self.naive_id, &main_entity.get_mbid())
            .await?;
        self.update(main_entity.clone()).await?;

        // Then, process the others
        // TODO: Use Stream
        for extra_entity in converted_fetch.1 {
            extra_entity.save_to_cache().await?;
        }

        Ok(main_entity)
    }

    // --- Insert ---

    /// Set a value in the value cache, its id in the alias cache and fill self
    pub async fn set(&mut self, value: Arc<V>) -> Result<(), serde_cacache::Error> {
        let mbid = value.get_mbid();
        let mbid_naive = mbid.clone().into_naive();

        // TODO: Add try_join! for speedup.
        self.loaded = Some(value.clone());
        self.alias_cache.set(&mbid_naive, &mbid_naive).await?;
        self.disk_cache.set(&mbid, value.as_ref()).await?;
        Ok(())
    }

    // --- Update ---

    pub async fn update(&mut self, value: Arc<V>) -> Result<(), Error> {
        let older_version = self.get_or_load().await?;

        let new_data = match older_version {
            Some(older) => Arc::new(
                older
                    .as_ref()
                    .clone()
                    .incremental_update(value.as_ref().clone()),
            ),
            None => value,
        };

        Ok(self.set(new_data).await?)
    }

    pub async fn update_from_generic_entity(
        &mut self,
        value: &AnyMusicBrainzEntity,
    ) -> Result<(), Error> {
        let converted = V::try_from_any(value)?;
        self.update(converted).await
    }
}
