use std::collections::HashMap;

use chrono::{DateTime, Utc};
use color_eyre::eyre::Context;
use derive_getters::Getters;
use derive_more::*;
use serde::{Deserialize, Serialize};

use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::musicbrainz::recording::Recording;

use super::messybrainz::MessyBrainzData;

pub mod collection;
pub mod convertion;
pub mod listen_mapping_state;
pub mod mapped_listen;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Getters)]
pub struct Listen<S = Unmapped>
//TODO: Remove default
where
    S: MappingState,
{
    /// The username of the user who listened to it
    pub user: String,

    /// Time of when the listen happened
    pub listened_at: DateTime<Utc>,

    /// Data that have been sent to listenbrainz durring listen submition
    pub messybrainz_data: MessyBrainzData,

    /// Data of the mapping
    pub mapping_data: S,
}

// Typestate
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Getters)]
pub struct Unmapped {}
pub type Mapped = MappingData;

trait MappingState {}
impl MappingState for Unmapped {}
impl MappingState for Mapped {}

impl<S> Listen<S>
where
    S: MappingState,
{
    pub fn get_listened_at(&self) -> &DateTime<Utc> {
        &self.listened_at
    }

    pub fn get_messybrain_data(&self) -> &MessyBrainzData {
        &self.messybrainz_data
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
}

impl Listen<Unmapped> {
    pub fn new_unmapped(
        username: String,
        listened_at: DateTime<Utc>,
        messybrainz_data: MessyBrainzData,
    ) -> Self {
        Self {
            user: username,
            listened_at,
            messybrainz_data,
            mapping_data: Unmapped {},
        }
    }

    #[deprecated]
    pub fn is_mapped(&self) -> bool {
        false
    }

    #[deprecated]
    pub fn get_mapping_data(&self) -> &Option<MappingData> {
        &None
    }

    /// If mapped, return the recording MBID
    #[deprecated]
    pub fn get_recording_mbid_as_string(&self) -> Option<&String> {
        None
    }

    /// Return true if the listen is mapped to this recording MBID
    #[deprecated]
    pub fn is_mapped_to_recording(&self, _mbid: &str) -> bool {
        false
    }

    /// Return the recording's data from Musicbrainz from its mapping
    #[deprecated]
    pub async fn get_recording_data(&self) -> color_eyre::Result<Option<Recording>> {
        Ok(None)
    }
}
