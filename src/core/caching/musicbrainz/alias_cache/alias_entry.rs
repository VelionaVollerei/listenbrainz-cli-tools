use std::collections::HashMap;
use std::sync::Arc;

use futures::try_join;
use tokio::sync::RwLock;

use crate::core::caching::serde_cacache::error::Error as SerdeCacacheError;
use crate::core::caching::serde_cacache::tidy::SerdeCacacheTidy;
use crate::core::caching::CACHE_LOCATION;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::error::Error;
use crate::utils::println_cli_warn;

pub(super) enum AliasEntry<V>
where
    V: MusicBrainzEntity,
{
    /// The ID is just an alias of the other
    Alias(PrimaryMBID<V>),

    /// The ID is the primary one, and has those childrens
    Primary(Vec<PrimaryMBID<V>>),
}

impl<V> AliasEntry<V> where V: MusicBrainzEntity {}
