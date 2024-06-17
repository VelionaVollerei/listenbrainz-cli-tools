use std::sync::Arc;

use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::Listen;
use super::Mapped;
use super::Unmapped;
use chrono::DateTime;
use chrono::Utc;
use derive_more::*;
use futures::TryFutureExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Unwrap, IsVariant, Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ListenMappingState {
    Unmapped(Arc<Listen<Unmapped>>),
    Mapped(Arc<Listen<Mapped>>),
}

impl ListenMappingState {
    pub async fn get_primary_recording_id(&self) -> color_eyre::Result<Option<RecordingMBID>> {
        match self {
            Self::Mapped(val) => val.get_primary_recording_id().map_ok(|data| Some(data)).await,
            Self::Unmapped(_) => Ok(None)
        }
    }

    pub fn get_listened_at(&self) -> &DateTime<Utc> {
        match self {
            Self::Mapped(val) => val.get_listened_at(),
            Self::Unmapped(val) => val.get_listened_at()
        }
    }

    
}