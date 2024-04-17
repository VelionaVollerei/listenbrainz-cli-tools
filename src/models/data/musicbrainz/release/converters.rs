use itertools::Itertools;
use musicbrainz_rs::entity::release::Release as ReleaseMS;

use super::{media::Media, Release};

impl From<ReleaseMS> for Release {
    fn from(value: ReleaseMS) -> Self {
        Self {
            annotation: value.annotation,
            barcode: value.barcode,
            country: value.country,
            disambiguation: value.disambiguation,
            id: value.id,
            media: value
                .media
                .map(|medias| medias.into_iter().map(Media::from).collect_vec()),
            packaging_id: value.packaging_id,
            status_id: value.status_id,
            title: value.title,
            release_group: value.release_group.map(|rg| rg.id)
        }
    }
}
