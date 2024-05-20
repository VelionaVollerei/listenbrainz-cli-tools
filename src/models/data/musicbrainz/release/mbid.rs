use color_eyre::eyre::Context;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::release::Release;
use derive_more::{Deref, DerefMut, Display, From, Into};
use serde::{Deserialize, Serialize};
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::utils::println_mus;
use musicbrainz_rs::entity::release::Release as ReleaseMS;
use musicbrainz_rs::Fetch;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::release::external::ReleaseExt;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Display,
)]
pub struct ReleaseMBID(String);

impl IsMbid<Release> for ReleaseMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Release> {
        Release::get_cache().get_or_fetch(&self.0).await
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for release MBID: {}", &self));

        Ok(ReleaseMS::fetch()
            .id(self)
            .with_recordings()
            .with_artists()
            .with_artist_credits()
            .with_release_groups()
            .with_artist_credits()
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .execute()
            .await
            .context("Failed to fetch release from MusicBrainz")?
            .into_entity())
    }

    fn into_mbid(self) -> MBID {
        MBID::Release(self)
    }
}
