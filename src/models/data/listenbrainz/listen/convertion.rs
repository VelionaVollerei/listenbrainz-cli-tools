use chrono::{TimeZone, Utc};
use listenbrainz::raw::response::UserListensListen;

use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;

use super::listen_mapping_state::ListenMappingState;
use super::Listen;

impl From<UserListensListen> for ListenMappingState {
    fn from(value: UserListensListen) -> Self {
        let listened_at = Utc
            .timestamp_opt(value.listened_at, 0)
            .single()
            .expect("Cannot convert listened_at timestamp. This shouldn't happen since all the dates are UTC!");

        match value.track_metadata.mbid_mapping.clone() {
            Some(mapping) => Self::Mapped(Listen::new_mapped(
                value.user_name.clone(),
                listened_at,
                MessyBrainzData::from(value.clone()),
                MappingData::from(mapping),
            )),

            None => Self::Unmapped(Listen::new_unmapped(
                value.user_name.clone(),
                listened_at,
                MessyBrainzData::from(value.clone()),
            )),
        }
    }
}
