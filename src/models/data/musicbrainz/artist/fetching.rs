use color_eyre::eyre::{Context, Ok};
use musicbrainz_rs::entity::artist::Artist as ArtistMS;
use musicbrainz_rs::Fetch;

use crate::models::api::HasFetchApi;
use crate::models::cache::cached_trait::CacheFromMusicbrainzAutoId;
use crate::models::cache::global_cache::GlobalCache;
use crate::models::data::musicbrainz::artist::Artist;
use crate::utils::println_mus;

impl HasFetchApi<String, Artist> for Artist {
    fn fetch_and_insert(
        key: &String,
    ) -> impl std::future::Future<Output = color_eyre::Result<Artist>> {
        let key = key.clone();
        async move {
            println_mus(format!("Getting data for artist MBID: {}", &key));
            let msreturn = ArtistMS::fetch()
                .id(&key)
                .with_recordings()
                .execute()
                .await
                .context("Failed to fetch artist from MusicBrainz")?;

            Self::insert_ms_into_cache(msreturn)?;

            // The element have been inserted above, so it should be safe to unwrap the option
            Ok(GlobalCache::new().get_artist_cache().get(&key)?.unwrap())
        }
    }
}
