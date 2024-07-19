pub mod alias_entry;
use std::collections::HashMap;
use std::sync::Arc;

use alias_entry::AliasEntry;
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

pub struct AliasCache<V>
where
    V: MusicBrainzEntity,
{
    pub(super) disk_cache: Arc<SerdeCacacheTidy<NaiveMBID<V>, AliasEntry<V>>>,
    pub(super) loaded_cache: Arc<RwLock<HashMap<NaiveMBID<>>>
}

