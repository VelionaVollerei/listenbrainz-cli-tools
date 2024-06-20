use extend::ext;
use futures::stream;
use futures::Stream;
use futures::StreamExt;
use futures::TryStream;
use futures::TryStreamExt;

use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::mbid::entity_with_mbid::EntityWithMBID;
use crate::models::data::musicbrainz::mbid::mbid_of_entity::MBIDOfEntity;

#[ext(name = MBIDStreamT)]
pub impl<S, Id, Ent, LID> S
where
    S: Stream<Item = Id>,
    Id: MBIDOfEntity<Ent, LID>,
    Ent: EntityWithMBID + MBCached<LID>,
    LID: IsMbid<Ent>,
{
    fn into_entities(self) -> impl TryStream<Ok = Ent, Error = color_eyre::Report> {
        self.map(|id| async move { id.get_or_fetch_entity().await })
            .buffered(1)
    }
}

// #[ext(name = MBIDTryStreamT)]
// pub impl<S, Id, Ent, LID> S
// where
//     S: TryStream<Ok = Id, Error = color_eyre::Report> + TryStreamExt,
//     Id: MBIDOfEntity<Ent, LID>,
//     Ent: EntityWithMBID + MBCached<LID>,
//     LID: IsMbid<Ent>,
// {
//     fn into_entities(self) -> impl TryStream<Ok = Ent, Error = color_eyre::Report> {
//         self
//             .and_then(|id| async move { id.get_or_fetch_entity().await })
//             .try_buffered(1)
//     }
// }
