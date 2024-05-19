use derive_getters::Getters;
use std::sync::Arc;

use once_cell::sync::Lazy;

use super::listenbrainz::user_listens::UserListens;
use super::musicbrainz::artist::Artist;
use super::musicbrainz::recording::Recording;
use super::musicbrainz::release::Release;
use crate::core::caching::entity_cache::EntityCache;
use crate::models::data::musicbrainz::MBID;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;

pub(crate) static ENTITY_DATABASE: Lazy<Arc<EntityDatabase>> =
    Lazy::new(|| Arc::new(EntityDatabase::default()));

#[derive(Debug, Getters)]
pub struct EntityDatabase {
    artists: Arc<EntityCache<Artist>>,
    releases: Arc<EntityCache<Release>>,
    recordings: Arc<EntityCache<Recording>>,
    release_groups: Arc<EntityCache<ReleaseGroup>>,

    user_listens: Arc<EntityCache<UserListens>>,
    
    mbid_aliases: Arc<EntityCache<MBID>>,
}

impl Default for EntityDatabase {
    fn default() -> Self {
        Self {
            artists: Arc::new(EntityCache::new("artists")),
            releases: Arc::new(EntityCache::new("releases")),
            recordings: Arc::new(EntityCache::new("recordings")),
            release_groups: Arc::new(EntityCache::new("release_groups")),

            user_listens: Arc::new(EntityCache::new("user_listens")),
        }
    }
}
