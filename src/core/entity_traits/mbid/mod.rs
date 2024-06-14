pub mod is_cached_mbid;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::MBID;
use extend::ext;
use futures::stream;
use futures::Stream;
use futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Display};
use std::future::Future;

use super::updatable::Updatable;

pub trait IsMbid<T>
where
    Self: Display + Clone + Serialize + DeserializeOwned,
    T: HasMBID<Self>,
{
    fn get_or_fetch_entity(&self) -> impl Future<Output = color_eyre::Result<T>> + Send;

    fn fetch(&self) -> impl Future<Output = color_eyre::Result<ExternalMusicBrainzEntity>> + Send;

    fn into_mbid(self) -> MBID;
}

#[ext]
pub impl<T, I> Vec<I>
where
    T: HasMBID<I>,
    I: IsMbid<T>,
{
    #[allow(async_fn_in_trait)]
    async fn get_or_fetch_entities(&self) -> color_eyre::Result<Vec<T>> {
        let mut result = Vec::new();

        for item in self {
            result.push(item.get_or_fetch_entity().await?);
        }

        Ok(result)
    }

    fn get_or_fetch_entities_stream(&self) -> impl Stream<Item = color_eyre::Result<T>> {
        stream::iter(self)
            .map(|id| id.get_or_fetch_entity())
            .buffered(1)
    }
}

pub trait HasMBID<K>
where
    Self: Serialize + DeserializeOwned + Updatable + Sized + Debug + Clone,
    K: IsMbid<Self>,
{
    fn get_mbid(&self) -> K;
}
