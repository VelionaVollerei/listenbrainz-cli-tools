pub mod collection;
use std::sync::Arc;
use std::sync::Arc;

use tokio::sync::OnceCell;

use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::mbid::mbid_of_entity::MBIDOfEntity;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;

use super::listen_spe::ListenSpe;
use super::mapped_primary::MappedListen;
use super::Listen;

#[derive(Debug, Clone)]
pub struct ListenWithData {
    listen: Arc<MappedListen>,
    recording: OnceCell<Arc<Recording>>,
    releases: OnceCell<Vec<Arc<Release>>>,
}

impl ListenWithData {
    pub fn new(listen: Arc<MappedListen>) -> Self {
        Self {
            listen,
            recording: OnceCell::new(),
            releases: OnceCell::new(),
        }
    }

    pub async fn get_recording(&self) -> color_eyre::Result<Arc<Recording>> {
        let id = self.listen.get_recording_mbid().clone();
        let cached = self
            .recording
            .get_or_try_init(|| async move { id.get_or_fetch_entity().await.map(Arc::new) })
            .await?
            .clone();

        Ok(cached)
    }

    pub async fn get_releases(&self) -> color_eyre::Result<Vec<Arc<Release>>> {
        let recording = self.get_recording().await?;

        let cached = self
            .releases
            .get_or_try_init(|| async move { recording.get_or_fetch_releases().await })
            .await?
            .clone();

        Ok(cached)
    }
}

impl From<MappedListen> for ListenWithData {
    fn from(value: MappedListen) -> Self {
        ListenWithData::new(Arc::new(value))
    }
}

impl From<Arc<MappedListen>> for ListenWithData {
    fn from(value: Arc<MappedListen>) -> Self {
        ListenWithData::new(value)
    }
}
