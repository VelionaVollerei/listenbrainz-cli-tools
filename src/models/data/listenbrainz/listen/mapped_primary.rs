use futures::stream;
use futures::StreamExt;
use futures::TryStream;

use crate::core::constants::MAX_CONCURRENT_FETCH;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::mbid::mbid_of_entity::MBIDOfEntity;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use crate::utils::extensions::future_ext::CTryStreamExt;

use super::listen_spe::ListenSpe;
use super::listen_spe::MappedPrimary;
use super::mapped_naive::NaiveMappedListen;
use super::Listen;

pub type MappedListen = ListenSpe<MappedPrimary>;

impl MappedListen {
    pub fn get_recording_mbid(&self) -> &MBIDSpe<Recording, PrimaryID> {
        &self.mapping_data
    }

    pub async fn get_recording(&self) -> color_eyre::Result<Recording> {
        self.mapping_data.get_or_fetch_entity().await
    }

    pub fn into_naive(self) -> NaiveMappedListen {
        NaiveMappedListen {
            user: self.user,
            listened_at: self.listened_at,
            messybrainz_data: self.messybrainz_data,
            mapping_data: MappingData {
                recording_mbid: self.mapping_data.to_string(),
                artist_credit: None,
                artist_mbid: None,
                recording_name: String::new(),
            },
        }
    }

    pub fn into_legacy(self) -> Listen {
        self.into_naive().into()
    }

    /// Return the releases of the mapped recording
    pub async fn get_releases(&self) -> color_eyre::Result<Vec<Release>> {
        let release_ids = self
            .get_recording()
            .await?
            .get_or_fetch_releases_ids()
            .await?;
        stream::iter(release_ids)
            .map(|id| async move { id.get_or_fetch_entity().await })
            .buffer_unordered(MAX_CONCURRENT_FETCH as usize)
            .try_collect_vec()
            .await
    }
}
