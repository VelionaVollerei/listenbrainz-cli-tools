use super::mbid_of_entity::MBIDOfEntity;
use super::streams::mbid_stream::MBIDStreamT;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::mbid::entity_with_mbid::EntityWithMBID;
use extend::ext;
use futures::stream;
use futures::StreamExt;
use futures::TryStream;

pub mod legacy_collection;

#[ext(name = MBIDCollectionT)]
pub impl<Id, Ent, LID> Vec<Id>
where
    Id: MBIDOfEntity<Ent, LID>,
    Ent: EntityWithMBID + MBCached<LID>,
    LID: IsMbid<Ent>,
{
    fn into_entities(self) -> impl TryStream<Ok = Ent, Error = color_eyre::Report> {
        stream::iter(self).into_entities()
    }
}
