pub mod converters;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use derive_getters::Getters;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct RecordingIDWithListens {
    recording_id: RecordingMBID,
    listens: Vec<Arc<Listen>>,
}
