use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::{Insertable, IsAutoInsertable};
use crate::core::entity_traits::insertable_children::InsertableWithChildren;
use crate::core::entity_traits::into_ms_entities::IntoMSEntities;
use crate::models::data::musicbrainz::entity_enum::MSEntity;
use musicbrainz_rs::entity::artist::Artist;

impl HasID for Artist {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Insertable for Artist {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        crate::models::data::musicbrainz::artist::Artist::get_cache()
            .update(&key, self.clone().into())
            .await
    }
}

impl InsertableWithChildren for Artist {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(recordings) = self.recordings.clone() {
            for recording in recordings {
                recording.insert_into_cache().await?;
            }
        }

        if let Some(releases) = self.releases.clone() {
            for release in releases {
                release.insert_into_cache().await?;
            }
        }

        Ok(())
    }
}

impl IntoMSEntities for Artist {
    fn into_ms_entities(self) -> Vec<MSEntity> {
        let mut results = vec![MSEntity::Artist(self.clone().into())];

        results.extend(self.recordings.into_ms_entities());
        results.extend(self.releases.into_ms_entities());
        results.extend(self.release_groups.into_ms_entities());

        results
    }
}
