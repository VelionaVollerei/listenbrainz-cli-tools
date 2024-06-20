use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::mbid::entity_with_mbid::EntityWithMBID;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::mbid::mbid_of_entity::MBIDOfEntity;
use extend::ext;
use futures::stream;
use futures::StreamExt;
use futures::TryStream;
use futures::TryStreamExt;

#[ext(name = LegacyCollectionT)]
pub impl<Id, Ent, LID> Vec<LID>
where
    Id: MBIDOfEntity<Ent, LID>,
    Ent: EntityWithMBID + MBCached<LID>,
    LID: IsMbid<Ent>,
{
    fn into_specialised_ids(
        self,
    ) -> impl TryStream<Ok = MBIDSpe<Ent, PrimaryID>, Error = color_eyre::Report> + TryStreamExt
    {
        stream::iter(self)
            .map(|id| async move { Id::from_legacy(&id).await })
            .buffered(1)
    }
}
