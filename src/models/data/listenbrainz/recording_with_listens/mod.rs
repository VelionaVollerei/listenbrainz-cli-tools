use super::listen::Listen;
use crate::models::data::musicbrainz::recording::Recording;
use derive_getters::Getters;
use std::sync::Arc;

pub mod converters;
pub mod recording_id_with_listens;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct RecordingWithListens {
    recording: Recording,
    listens: Vec<Arc<Listen>>,
}

impl RecordingWithListens {
    pub fn get_first_listen(&self) -> Option<Arc<Listen>> {
        self.listens
            .iter()
            .min_by_key(|listen| listen.listened_at())
            .cloned()
    }

    pub fn get_latest_listen(&self) -> Option<Arc<Listen>> {
        self.listens
            .iter()
            .max_by_key(|listen| listen.listened_at())
            .cloned()
    }
}
