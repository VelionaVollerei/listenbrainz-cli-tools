use color_eyre::eyre::Ok;
use futures::stream;
use futures::StreamExt;

use crate::models::data::listenbrainz::listen::collection::listen_collection_spe::ListenCollectionSpe;
use crate::models::data::listenbrainz::listen::Mapped;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

impl ListenCollectionSpe<Mapped> {
    pub async fn get_listened_recordings_primary_mbids(&self) -> color_eyre::Result<Vec<RecordingMBID>> {
        let mut stream = stream::iter(self.data.iter())
            .map(|listen| listen.get_primary_recording_id())
            .buffer_unordered(10);

        let mut result = Vec::new();
        while let Some(recording_id) = stream.next().await {
            result.push(recording_id?);
        }

        Ok(result)
    }
}
