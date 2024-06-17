use std::sync::Arc;

use crate::models::data::listenbrainz::listen::listen_mapping_state::ListenMappingState;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::listenbrainz::listen::Mapped;
use crate::models::data::listenbrainz::listen::MappingState;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::listen_collection_spe::ListenCollectionSpe;
use super::ListenCollection;

impl ListenCollection {
    #[deprecated]
    pub async fn get_listened_recordings_mbids(&self) -> color_eyre::Result<Vec<RecordingMBID>> {
        //TODO: Multithread it
        let mut recordings = Vec::new();

        for listen in self.get_mapped_listens().iter() {
            recordings.push(
                listen
                    .get_primary_recording_id()
                    .await?
                    .expect("Listen should be mapped"),
            );
        }

        Ok(recordings)
    }

    pub fn into_mapped_collection(self) -> ListenCollectionSpe<Mapped> {
        self.into_iter().filter(|listen| listen.is_mapped()).map(|listens| listens.unwrap_mapped()).collect()
    }
}


impl<S> FromIterator<Arc<Listen<S>>> for ListenCollectionSpe<S> where S: MappingState{
    fn from_iter<T: IntoIterator<Item = Arc<Listen<S>>>>(iter: T) -> Self {
        let mut data = Vec::new();
        data.extend(iter);

        Self {
            data
        }
    }
}