pub mod work;
use crate::models::cli::common::{GroupByTarget, SortSorterBy};
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::tools::stats::release_groups::stats_release_groups;
use crate::utils::println_cli;

use self::work::stats_works;

mod artists;
mod recordings;
mod release_groups;
mod releases;

pub async fn stats_command(username: &str, target: GroupByTarget, sort_by: SortSorterBy) {
    // Get the listens
    let user_listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens");

    println_cli(format!(
        "Total number of listens: {}",
        user_listens.get_listens().len()
    ));

    let stats = user_listens
        .get_listens()
        .get_statistics_of(target)
        .await
        .expect("Couldn't sort the listens");

    match target {
        GroupByTarget::Recording => {
            recordings::stats_recording(stats, sort_by).await;
        },
        GroupByTarget::Artist => {
            artists::stats_artist(stats, sort_by).await;
        },
        GroupByTarget::Release => {
            releases::stats_releases(stats, sort_by).await;
        },
        GroupByTarget::ReleaseGroup => {
            stats_release_groups(stats, sort_by).await;
        },
        GroupByTarget::Work => {
            stats_works(stats, sort_by).await;
        },
    }
}
