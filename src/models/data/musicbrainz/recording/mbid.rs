use super::Recording;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::entity_with_mbid::EntityWithMBID;
use crate::models::data::musicbrainz::mbid::generic_mbid::IdAliasState;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::NaiveID;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::mbid::mbid_of_entity::MBIDOfEntity;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::external::RecordingExt;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use derive_more::{Deref, DerefMut, Display, From, Into};
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use musicbrainz_rs::Fetch;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
pub struct RecordingMBID(String);

impl IsMbid<Recording> for RecordingMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Recording> {
        Recording::get_cached_or_fetch(self).await
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for recording MBID: {}", &self));

        color_eyre::eyre::Ok(
            RecordingMS::fetch()
                .id(self)
                .with_artists()
                .with_releases()
                .with_work_relations()
                .with_aliases()
                .with_work_level_relations()
                .execute()
                .await
                .context("Failed to fetch recording from MusicBrainz")?
                .into_entity(),
        )
    }

    fn into_mbid(self) -> MBID {
        MBID::Recording(self)
    }
}

impl RecordingMBID {
    pub fn into_naive(self) -> MBIDSpe<Recording, NaiveID> {
        MBIDSpe::from(self.to_string())
    }
}

impl<S> MBIDSpe<Recording, S> where S: IdAliasState {}

impl<S> MBIDOfEntity<Recording, RecordingMBID> for MBIDSpe<Recording, S> where S: IdAliasState {}

impl<S> From<MBIDSpe<Recording, S>> for RecordingMBID
where
    S: IdAliasState,
{
    fn from(value: MBIDSpe<Recording, S>) -> Self {
        RecordingMBID(value.deref().to_string())
    }
}

impl EntityWithMBID for Recording {
    fn get_mbid(&self) -> MBIDSpe<Self, PrimaryID> {
        MBIDSpe::from(self.id.to_string())
    }
}
