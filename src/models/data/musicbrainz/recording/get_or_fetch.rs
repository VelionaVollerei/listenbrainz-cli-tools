use std::sync::Arc;

use color_eyre::eyre::Error;
use color_eyre::eyre::{eyre, Context, OptionExt};
use futures::stream;
use futures::StreamExt;
use futures::TryStream;
use futures::TryStreamExt;
use itertools::Itertools;

use crate::core::constants::MAX_CONCURRENT_FETCH;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::core::entity_traits::mbid::VecIExt;
use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::mbid::collections::legacy_collection::LegacyCollectionT;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::mbid::mbid_of_entity::MBIDOfEntity;
use crate::models::data::musicbrainz::mbid::streams::mbid_stream::MBIDStreamT;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::utils::extensions::future_ext::CTryStreamExt;

use super::Recording;

impl Recording {
    pub fn recording_mbid(&self) -> MBIDSpe<Self, PrimaryID> {
        self.id.to_string().into()
    }

    pub async fn get_or_fetch_releases_ids(&self) -> color_eyre::Result<Vec<ReleaseMBID>> {
        Ok(match &self.releases {
            Some(releases) => releases.clone(),
            None => {
                self.refresh()
                    .await
                    .context("Couldn't fetch data from the API")?
                    .releases
                    .ok_or_eyre(eyre!(format!("Releases is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested recording ID is: {}", &self.id)))?
            }
        })
    }

    pub async fn get_or_fetch_releases(&self) -> color_eyre::Result<Vec<Arc<Release>>> {
        stream::iter(self.get_or_fetch_releases_ids().await?)
            .map(|id| async move { id.get_or_fetch_entity().await.map(Arc::new) })
            .buffer_unordered(MAX_CONCURRENT_FETCH.try_into().unwrap())
            .try_collect_vec()
            .await
    }

    pub async fn get_or_fetch_work_ids(&self) -> color_eyre::Result<Vec<WorkMBID>> {
        Ok(match &self.relations {
            Some(releases) => releases.clone(),
            None => {
                self.refresh()
                    .await
                    .context("Couldn't fetch data from the API")?
                    .relations
                    .ok_or_eyre(eyre!(format!("Relation is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested recording ID is: {}", &self.id)))?
            }
        }.into_iter().filter(|relation| relation.content().is_work()).map(|relation| relation.content().clone().unwrap_work()).collect_vec())
    }

    pub async fn get_or_fetch_work_ids_with_parents(&self) -> color_eyre::Result<Vec<WorkMBID>> {
        let work_ids = self.get_or_fetch_work_ids().await?;

        if work_ids.is_empty() {
            return Ok(work_ids);
        }

        let mut result_ids = Vec::new();
        for parent in work_ids {
            result_ids.extend(
                parent
                    .get_or_fetch_entity()
                    .await?
                    .get_all_parent_works_ids()
                    .await?,
            );
            result_ids.push(parent);
        }

        Ok(result_ids)
    }
}

impl HasArtistCredits<RecordingMBID> for Recording {
    fn get_artist_credits(&self) -> &Option<ArtistCredits> {
        &self.artist_credit
    }
}
