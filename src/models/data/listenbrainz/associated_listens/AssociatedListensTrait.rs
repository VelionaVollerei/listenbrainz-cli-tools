use crate::models::data::listenbrainz::listen::collection::mapped_listen_collection::MappedListenCollection;

pub trait AssociatedListens {
    fn listens() -> MappedListenCollection;
}
