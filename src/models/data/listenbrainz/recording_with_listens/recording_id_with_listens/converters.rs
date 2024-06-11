use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::listenbrainz::recording_with_listens::RecordingWithListens;

use super::RecordingIDWithListens;

impl RecordingIDWithListens {
    pub async fn into_recording_with_listens(self) -> color_eyre::Result<RecordingWithListens>{
        RecordingWithListens::from_recording_id_with_listens(self).await
    }
}