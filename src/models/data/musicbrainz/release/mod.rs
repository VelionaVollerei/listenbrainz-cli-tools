use chrono::NaiveDate;
use derive_getters::Getters;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::release::{ReleasePackaging, ReleaseStatus};
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::core::entity_traits::relations::has_release_group::HasReleaseGroup;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};
use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;

use self::mbid::ReleaseMBID;
use self::media::Media;

pub mod external;

pub mod caching;
pub mod converters;
pub mod get_or_fetch;
pub mod getters;
pub mod mbid;
pub mod media;
pub mod track;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Release {
    id: ReleaseMBID,
    title: String,
    status_id: Option<String>,
    status: Option<ReleaseStatus>,
    date: Option<NaiveDate>,
    country: Option<String>,
    //quality: Option<ReleaseQuality>, //TODO: Mirror renaming #[serde(rename_all(deserialize = "lowercase"))]
    barcode: Option<String>,
    disambiguation: Option<String>,
    packaging_id: Option<String>,
    packaging: Option<ReleasePackaging>,
    relations: Option<Vec<Relation>>,
    release_group: Option<ReleaseGroupMBID>,
    artist_credit: Option<ArtistCredits>,
    media: Option<Vec<Media>>,
    //label_info: Option<Vec<LabelInfo>>,
    tags: Option<Vec<Tag>>,
    aliases: Option<Vec<Alias>>,
    genres: Option<Vec<Genre>>,
    annotation: Option<String>,
}

impl IsMusicbrainzEntity for Release {
    fn as_kind(&self) -> MusicbrainzEntityKind {
        MusicbrainzEntityKind::Release
    }

    fn get_mbid(&self) -> MBIDSpe<Self, PrimaryID> {
        MBIDSpe::from(self.id.to_string())
    }
}

impl HasArtistCredits<ReleaseMBID> for Release {
    fn get_artist_credits(&self) -> &Option<ArtistCredits> {
        &self.artist_credit
    }
}

impl HasReleaseGroup<ReleaseMBID> for Release {
    fn get_release_group(&self) -> &Option<ReleaseGroupMBID> {
        &self.release_group
    }
}
