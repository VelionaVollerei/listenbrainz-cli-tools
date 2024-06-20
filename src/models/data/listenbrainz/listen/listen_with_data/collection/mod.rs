use super::ListenWithData;
use crate::core::constants::MAX_CONCURRENT_FETCH;
use crate::models::data::listenbrainz::listen::collection::mapped_listen_collection::MappedListenCollection;
use crate::models::data::musicbrainz::mbid::entity_with_mbid::EntityWithMBID;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::release::Release;
use crate::utils::extensions::future_ext::cStreamExt;
use crate::utils::extensions::future_ext::CTryStreamExt;
use extend::ext;
use futures::stream;
use futures::StreamExt;
use futures::TryStreamExt;
use itertools::Itertools;
use std::sync::Arc;

pub type ListenWithDataCollection = Vec<ListenWithData>;

#[ext]
pub impl ListenWithDataCollection {
    async fn retain_listens_of_release(
        self,
        release_id: &MBIDSpe<Release, PrimaryID>,
    ) -> color_eyre::Result<ListenWithDataCollection> {
        stream::iter(self)
            .filter_map(|listen| async move {
                let releases = listen.get_releases().await;

                match releases {
                    Err(val) => Some(Err(val)),
                    Ok(val) => {
                        let is_in_release = val
                            .iter()
                            .map(|release| release.get_mbid())
                            .any(|id| &id == release_id);

                        if is_in_release {
                            Some(Ok(listen))
                        } else {
                            None
                        }
                    }
                }
            })
            .buffer_unordered_non_future(MAX_CONCURRENT_FETCH.try_into().unwrap())
            .try_collect_vec()
            .await
    }
}
