use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::musicbrainz::recording::Recording;
use itertools::Itertools;
use std::sync::Arc;

use super::recording_id_with_listens::RecordingIDWithListens;
use super::RecordingWithListens;

impl RecordingWithListens {
    pub fn from_listens_iter(recording: Recording, listens: &[Arc<Listen>]) -> Self {
        let recording_id = recording.id().clone();
        Self {
            recording,
            listens: listens
                .iter()
                .filter(|listen| {
                    listen
                        .recording_mbid_unchecked()
                        .is_some_and(|id| id == recording_id)
                })
                .cloned()
                .collect_vec(),
        }
    }

    pub async fn convert_listen_iter(listens: &[Arc<Listen>]) -> color_eyre::Result<Vec<Self>> {
        let grouped_listens = listens
            .iter()
            .cloned()
            .into_group_map_by(|listen| listen.recording_mbid_unchecked());

        let mut out = Vec::new();
        for (recording_id, listens) in grouped_listens.into_iter() {
            let Some(recording_id) = recording_id else {
                continue;
            };

            let recording = recording_id.get_or_fetch_entity().await?;
            out.push(Self { recording, listens });
        }

        Ok(out)
    }

    pub async fn from_recording_id_with_listens(value: RecordingIDWithListens) -> color_eyre::Result<Self> {
        let recording = value.recording_id().get_or_fetch_entity().await?;

        Ok(Self {recording, listens: value.listens().clone()})
    }
}
