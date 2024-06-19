use chrono::DateTime;
use chrono::Utc;
use derive_getters::Getters;

use crate::models::data::listenbrainz::listen::collection::common::ListenCollectionCommons;
use crate::models::data::listenbrainz::listen::collection::mapped_listen_collection::MappedListenCollection;
use crate::models::data::musicbrainz::release::Release;

#[derive(Debug, Getters, Clone)]
pub struct ReleaseWithListens {
    release: Release,
    listens: MappedListenCollection,
}

impl ReleaseWithListens {
    pub fn new(release: Release, listens: MappedListenCollection) -> Self {}

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
