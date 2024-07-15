use color_eyre::eyre::Context;
use derive_more::{Deref, DerefMut, Display, From, Into};
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use musicbrainz_rs::Fetch;
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid;
use crate::models::data::musicbrainz::mbid::generic_mbid::IdAliasState;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::NaiveMBID;
use crate::models::data::musicbrainz::mbid::is_musicbrainz_id::IsMusicbrainzID;
use crate::models::data::musicbrainz::mbid::state_id::any::any_entity::AnyEntityMBID;
use crate::models::data::musicbrainz::mbid::state_id::any::any_id_to_kind::AnyIdToKind;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::external::RecordingExt;
use crate::models::error::Error;
use crate::utils::println_mus;

use super::Recording;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
#[deprecated]
pub struct RecordingMBID(String);

impl RecordingMBID {
    pub fn into_spe_naive(&self) -> NaiveMBID<Recording> {
        MBIDSpe::from(self.to_string())
    }

    pub async fn into_stateful(&self) -> color_eyre::Result<PrimaryMBID<Recording>> {
        Ok(self.get_or_fetch_entity().await?.get_mbid())
    }
}

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

impl<S> IsMusicbrainzID<Recording> for MBIDSpe<Recording, S>
where
    S: IdAliasState,
{
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
}

impl From<RecordingMBID> for mbid::state_id::state::NaiveMBID<Recording> {
    fn from(value: RecordingMBID) -> Self {
        Self::from(value.0)
    }
}

impl<S> AnyIdToKind<Recording, S> for MBIDWithState<Recording, S>
where
    S: MBIDState,
{
    fn try_from_any(value: AnyEntityMBID<S>) -> Result<Self, Error> {
        match value {
            AnyEntityMBID::Recording(val) => Ok(val),
            _ => Err(Error::InvalidKindConvertion(
                "!Recording".to_string(),
                "Recording".to_string(),
            )),
        }
    }
}
