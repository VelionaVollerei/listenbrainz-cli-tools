use crate::models::cli::common::SortListensBy;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::listenbrainz::listen::MappingState;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use derive_more::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Deref, DerefMut, IntoIterator)]
pub struct ListenCollectionSpe<S>
where
    S: MappingState,
{
    pub(super) data: Vec<Arc<Listen<S>>>,
}

