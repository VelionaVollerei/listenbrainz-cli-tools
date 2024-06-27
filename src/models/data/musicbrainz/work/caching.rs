use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCache;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;

use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

use super::Work;

impl HasID for Work {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl HasMBID<WorkMBID> for Work {
    fn get_mbid(&self) -> WorkMBID {
        self.id.clone()
    }
}

impl MBCached<WorkMBID> for Work {
    fn get_cache() -> Arc<MusicbrainzCache<WorkMBID, Self>> {
        MUSICBRAINZ_DATABASE.works().clone()
    }
}
