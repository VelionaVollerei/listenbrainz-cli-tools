pub mod getters;
use super::messybrainz::MessyBrainzData;
use crate::core::entity_traits::fetchable::FetchableAndCachable;
use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::musicbrainz::recording::Recording;
use chrono::{DateTime, Utc};
use color_eyre::eyre::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod collection;
pub mod convertion;
mod mapped_listen;

/// (User, msid, listened at)
pub type ListenID = (String, String, DateTime<Utc>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Listen {
    /// The username of the user who listened to it
    pub user: String,

    /// Time of when the listen happened
    pub listened_at: DateTime<Utc>,

    /// Data that have been sent to listenbrainz durring listen submition
    pub messybrainz_data: MessyBrainzData,

    /// Data of the mapping
    pub mapping_data: Option<MappingData>,
}

impl Listen {
    pub fn is_mapped(&self) -> bool {
        self.mapping_data.is_some()
    }

    pub fn get_mapping_data(&self) -> &Option<MappingData> {
        &self.mapping_data
    }

    pub fn get_listened_at(&self) -> &DateTime<Utc> {
        &self.listened_at
    }

    /// If mapped, return the recording MBID
    pub fn get_recording_mbid(&self) -> Option<&String> {
        self.mapping_data
            .as_ref()
            .map(|mapping| &mapping.recording_mbid)
    }

    /// Return true if the listen is mapped to this recording MBID
    pub fn is_mapped_to_recording(&self, mbid: &str) -> bool {
        self.mapping_data
            .as_ref()
            .is_some_and(|mapping| mapping.recording_mbid == mbid)
    }

    /// Return the recording's data from Musicbrainz from its mapping
    pub async fn get_recording_data(&self) -> color_eyre::Result<Option<Recording>> {
        match &self.mapping_data {
            Some(mapping) => Ok(Some(
                Recording::get_cached_or_fetch(mapping.recording_mbid()).await?,
            )),
            None => Ok(None),
        }
    }

    /// Send a mapping request to Listenbrainz
    pub async fn submit_mapping(&self, mbid: &str, token: &str) -> color_eyre::Result<()> {
        let client = reqwest::Client::new();

        let mut body_json = HashMap::new();
        body_json.insert("recording_msid", self.get_messybrain_data().msid.clone());
        body_json.insert("recording_mbid", mbid.to_owned());

        client
            .post("https://api.listenbrainz.org/1/metadata/submit_manual_mapping/")
            .header("Authorization", format!("Token {}", token.to_owned()))
            .json(&body_json)
            .send()
            .await
            .context("Couldn't send the mapping to Listenbrainz")?
            .error_for_status()
            .context("Listenbrainz returned an error")?;

        Ok(())
    }

    pub fn get_id(&self) -> ListenID {
        (self.user.clone(), self.messybrainz_data.msid, self.listened_at)
    }
}
