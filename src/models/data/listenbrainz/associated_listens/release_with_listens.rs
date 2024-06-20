use std::sync::Arc;

use crate::models::data::listenbrainz::listen::collection::common::ListenCollectionCommons;
use crate::models::data::listenbrainz::listen::collection::mapped_listen_collection::MappedListenCollection;
use crate::models::data::listenbrainz::listen::listen_with_data::collection::ListenWithDataCollection;
use crate::models::data::listenbrainz::listen::listen_with_data::collection::ListenWithDataCollectionExt;
use crate::models::data::musicbrainz::mbid::entity_with_mbid::EntityWithMBID;
use crate::models::data::musicbrainz::release::Release;
use chrono::DateTime;
use chrono::Utc;
use derive_getters::Getters;

#[derive(Debug, Getters, Clone)]
pub struct ReleaseWithListens {
    release: Arc<Release>,
    listens: ListenWithDataCollection,
}

impl ReleaseWithListens {
    pub async fn try_new(release: Arc<Release>, listens: ListenWithDataCollection) -> color_eyre::Result<Self> {
        Ok(Self {
            release: release.clone(),
            listens: listens.retain_listens_of_release(&release.get_mbid()).await?
        })
    }

    /// The date any of the recordings have been listened first
    pub fn first_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .oldest_listen()
            .map(|listen| *listen.listened_at())
    }

    /// The date any of the recordings have been listened last
    pub fn last_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .latest_listen()
            .map(|listen| *listen.listened_at())
    }

    /// Return the total listen count
    pub fn count(&self) -> usize {
        self.listens.len()
    }
}
