use std::collections::HashMap;
use std::sync::Arc;

use futures::try_join;
use tokio::sync::RwLock;

use crate::core::caching::serde_cacache::error::Error as SerdeCacacheError;
use crate::core::caching::serde_cacache::tidy::SerdeCacacheTidy;
use crate::core::caching::CACHE_LOCATION;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::error::Error;
use crate::utils::println_cli_warn;

use super::cached_entity::CachedEntity;

#[derive(Debug)]
pub struct MusicbrainzCache<V>
where
    V: MusicBrainzEntity,
{
    pub(super) cache_entities: RwLock<HashMap<NaiveMBID<V>, Arc<RwLock<CachedEntity<V>>>>>,

    pub(super) disk_cache: Arc<SerdeCacacheTidy<PrimaryMBID<V>, V>>,
    pub(super) alias_cache: Arc<SerdeCacacheTidy<NaiveMBID<V>, NaiveMBID<V>>>,
}

impl<V> MusicbrainzCache<V>
where
    V: MusicBrainzEntity + Eq,
{
    pub fn new(name: &str) -> Self {
        let mut location = CACHE_LOCATION.clone();
        location.push(name);

        let mut alias_location = CACHE_LOCATION.clone();
        alias_location.push(format!("{name}_aliases"));

        Self {
            cache_entities: RwLock::new(HashMap::new()),
            disk_cache: Arc::new(SerdeCacacheTidy::new(location)),
            alias_cache: Arc::new(SerdeCacacheTidy::new(alias_location)),
        }
    }

    pub async fn get_entity_entry(&self, id: &NaiveMBID<V>) -> Arc<RwLock<CachedEntity<V>>> {
        // Use a read to get the entity
        if let Some(entity) = self.cache_entities.read().await.get(id) {
            return entity.clone();
        }

        // The entity isn't found. Let's get into exclusive write mode
        let mut map = self.cache_entities.write().await;

        // While we waited for a write, it may have initialized the entity. Let's recheck
        if let Some(entity) = map.get(id) {
            return entity.clone();
        }

        // No entity was found so we initialize it
        let entity = Arc::new(RwLock::new(CachedEntity::new(
            id.clone(),
            self.disk_cache.clone(),
            self.alias_cache.clone(),
        )));
        map.insert(id.clone(), entity.clone());
        entity
    }

    pub async fn force_fetch_entity(&self, mbid: &NaiveMBID<V>) -> color_eyre::Result<Arc<V>> {
        self.remove(mbid).await?;
        Ok(self.get_load_or_fetch(mbid).await?)
    }

    // pub async fn set(&self, value: Arc<V>) -> Result<(), SerdeCacacheError> {
    //     self.get_entity_entry(&value.get_mbid().into_naive())
    //         .await
    //         .set(value)
    //         .await
    // }

    pub async fn update(&self, value: Arc<V>) -> Result<(), Error> {
        self.get_entity_entry(&value.get_mbid().into_naive())
            .await
            .update(value)
            .await
    }

    pub async fn invalidate_last_entries(
        &self,
        k: usize,
        keep_min: usize,
    ) -> color_eyre::Result<()> {
        self.disk_cache.delete_last_entries(k, keep_min).await?;
        Ok(())
    }



    pub async fn remove(&self, id: &NaiveMBID<V>) -> color_eyre::Result<()> {
        self.cache_entities.write().await.remove(id);
        self.alias_cache.remove(id).await?;
        self.disk_cache.remove(id).await?;
        Ok(())
    }

    // pub async fn get_or_fetch_primary_mbid_alias(
    //     &self,
    //     mbid: &NaiveMBID<V>,
    // ) -> color_eyre::Result<NaiveMBID<V>> {
    //     match self.alias_cache.get_or_option(mbid).await {
    //         Ok(Some(val)) => Ok(val),
    //         Ok(None) | Err(SerdeCacacheError::CacheDeserializationError(_)) => {
    //             Ok(self.force_fetch_entity(mbid).await?.get_mbid().into_naive())
    //         }
    //         Err(val) => Err(val.into()),
    //     }
    // }

    pub async fn clear(&self) -> cacache::Result<()> {
        let _ = try_join!(self.alias_cache.clear(), self.disk_cache.clear())?;
        self.cache_entities.write().await.clear();

        Ok(())
    }
}
