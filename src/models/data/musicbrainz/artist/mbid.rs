use std::sync::Arc;

use color_eyre::eyre::Context;
use derive_more::{Deref, DerefMut, Display, From};
use musicbrainz_rs::entity::artist::Artist as ArtistMS;
use musicbrainz_rs::Fetch;
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::artist::external::ArtistExt;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::generic_mbid::{IdAliasState, MBIDSpe};
use crate::models::data::musicbrainz::mbid::is_musicbrainz_id::IsMusicbrainzID;
use crate::models::data::musicbrainz::mbid::IsMusicbrainzID;
use crate::models::data::musicbrainz::mbid::MBIDEnum;
use crate::utils::println_mus;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Serialize, Deserialize, Display, Hash,
)]
pub struct ArtistMBID(String);

impl IsMbid<Artist> for ArtistMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Arc<Artist>> {
        Artist::get_cached_or_fetch(self).await
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for artist MBID: {}", &self));

        Ok(ArtistMS::fetch()
            .id(self)
            .with_aliases()
            .with_artist_relations()
            .with_recording_relations()
            .execute()
            .await
            .context("Failed to fetch artist from MusicBrainz")?
            .into_entity())
    }

    fn into_mbid(self) -> MBIDEnum {
        MBIDEnum::Artist(self)
    }
}

impl<S> IsMusicbrainzID<Artist> for MBIDSpe<Artist, S>
where
    S: IdAliasState,
{
    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for artist MBID: {}", &self));

        Ok(ArtistMS::fetch()
            .id(self)
            .with_aliases()
            .with_artist_relations()
            .with_recording_relations()
            .execute()
            .await
            .context("Failed to fetch artist from MusicBrainz")?
            .into_entity())
    }
}

impl<S> IsMusicbrainzID<Artist> for MBIDSpe<Artist, S>
where
    S: IdAliasState,
{
    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for artist MBID: {}", &self));

        Ok(ArtistMS::fetch()
            .id(self)
            .with_aliases()
            .with_artist_relations()
            .with_recording_relations()
            .execute()
            .await
            .context("Failed to fetch artist from MusicBrainz")?
            .into_entity())
    }
}