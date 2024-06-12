use std::ops::Deref;
use std::sync::Arc;

use async_stream::try_stream;
use chrono::format::Item;
use color_eyre::eyre::Error;
use futures::stream;
use futures::Stream;
use futures::StreamExt;
use futures::TryFutureExt;
use futures::TryStreamExt;
use itertools::Itertools;
use musicbrainz_rs::entity::release_group;

use crate::core::display::progress_bar::ProgressBarCli;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::VecIExt;
use crate::core::entity_traits::relations::has_release_group::HasReleaseGroup;
use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::cli::common::GroupByTarget;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::work::Work;

impl ListenCollection {
    pub async fn get_statistics_of(
        &self,
        target: GroupByTarget,
    ) -> color_eyre::Result<StatisticSorter> {
        let mapped = self.get_mapped_listens();
        let progress_bar = ProgressBarCli::new(
            mapped.len() as u64,
            Some(&format!("Calculating {} statistics", target.to_str())),
        );

        let counter = StatisticSorter::new();

        match target {
            GroupByTarget::Recording => {
                mapped
                    .get_recording_statistics_stream(&counter, &progress_bar)
                    .await?;
            }
            GroupByTarget::Artist => {
                mapped
                    .get_artist_statistics(&counter, &progress_bar)
                    .await?;
            }
            GroupByTarget::Release => {
                mapped
                    .get_release_statistics(&counter, &progress_bar)
                    .await?;
            }
            GroupByTarget::ReleaseGroup => {
                mapped
                    .get_release_group_statistics(&counter, &progress_bar)
                    .await?;
            }
            GroupByTarget::Work => {
                mapped.get_work_statistics(&counter, &progress_bar).await?;
            }
        }

        Ok(counter)
    }

    async fn stats<L, K>(listens: L, stats_type: GroupByTarget) -> color_eyre::Result<()>
    where
        L: IntoIterator<Item = Arc<Listen>> {
        let mut ids = try_stream! {
            for listen in listens {
                let data = listen.get_statistic_data(stats_type).await?;

                for ele in data {
                    yield (ele, listen.clone())
                }
            }
        };

        ids = ids.buffer_unordered(20)
        .try_collect()
        .await;

        let recording_ids: color_eyre::Result<Vec<(String, Arc<Listen>)>> = tokio_stream::iter(listens)
            .try(|listen| async move {(listen.get_statistic_data(stats_type), listen)})
            

        for (id, listen) in recording_ids? {
            counter.insert(&id, listen);
        }

        Ok(())
    }

    async fn get_recording_statistics_stream(self, counter: &StatisticSorter, progress_bar: &ProgressBarCli) -> color_eyre::Result<()> {
        let recording_ids: color_eyre::Result<Vec<(RecordingMBID, Arc<Listen>)>> = stream::iter(progress_bar.wrap_iter(self.into_iter()))
        .map(|listen| {
            async move {
                let recording_id = listen.get_primary_recording_id().await.transpose().expect("The listen should be mapped");

                recording_id.map(|id| (id, listen))
            }
        })
        .buffer_unordered(20).try_collect().await;

        for (id, listen) in recording_ids? {
            counter.insert(&id, listen);
        }

        Ok(())
    }

    async fn get_recording_statistics(
        self,
        counter: &StatisticSorter,
        progress_bar: &ProgressBarCli,
    ) -> color_eyre::Result<()> {
        for listen in self.into_iter() {
            counter.insert(
                listen
                    .get_primary_recording_id()
                    .await?
                    .expect("The listen should be mapped")
                    .to_string()
                    .as_str(),
                listen,
            );
            progress_bar.inc(1);
        }

        Ok(())
    }

    async fn get_artist_statistics(
        self,
        counter: &StatisticSorter,
        progress_bar: &ProgressBarCli,
    ) -> color_eyre::Result<()> {
        for listen in self {
            let artist_ids = listen
                .clone()
                .get_mapping_data()
                .as_ref()
                .expect("The listen should be mapped")
                .get_or_fetch_artist_mbids()
                .await?;

            for artist_id in artist_ids {
                counter.insert(artist_id.deref(), listen.clone());
            }
            progress_bar.inc(1);
        }

        Ok(())
    }

    async fn get_release_statistics(
        self,
        counter: &StatisticSorter,
        progress_bar: &ProgressBarCli,
    ) -> color_eyre::Result<()> {
        for listen in self {
            let releases_ids = listen
                .clone()
                .get_mapping_data()
                .as_ref()
                .expect("The listen should be mapped")
                .get_or_fetch_recording()
                .await?
                .get_or_fetch_releases_ids()
                .await?;

            for releases_id in releases_ids {
                counter.insert(releases_id.deref(), listen.clone());
            }
            progress_bar.inc(1);
        }

        Ok(())
    }

    async fn get_release_group_statistics(
        self,
        counter: &StatisticSorter,
        progress_bar: &ProgressBarCli,
    ) -> color_eyre::Result<()> {
        for listen in self {
            let releases = listen
                .clone()
                .get_mapping_data()
                .as_ref()
                .expect("The listen should be mapped")
                .get_or_fetch_recording()
                .await?
                .get_or_fetch_releases_ids()
                .await?
                .get_or_fetch_entities()
                .await?;

            let mut tasks = stream::iter(releases)
                .map(|release| async move { release.get_or_fetch_release_group().await })
                .buffer_unordered(5);

            let mut release_groups_ids = Vec::new();
            while let Some(release_group) = tasks.next().await {
                release_groups_ids.push(release_group?);
            }

            release_groups_ids = release_groups_ids.into_iter().unique().collect_vec();

            for release_groups_id in release_groups_ids {
                counter.insert(release_groups_id.deref(), listen.clone());
            }
            progress_bar.inc(1);
        }

        Ok(())
    }

    async fn get_work_statistics(
        self,
        counter: &StatisticSorter,
        progress_bar: &ProgressBarCli,
    ) -> color_eyre::Result<()> {
        for listen in self {
            let recording = listen
                .clone()
                .get_mapping_data()
                .as_ref()
                .expect("The listen should be mapped")
                .get_or_fetch_recording()
                .await?;

            let mut work_ids = recording.get_or_fetch_work_ids_with_parents().await?;

            // If the work is empty, this probably mean it wasn't added on musicbrainz.
            // We'll add a fake one to simulate it, altough it may not be accurate
            if work_ids.is_empty() {
                let new_work = Work::create_fake_work(
                    format!("_fake_{}", recording.title()),
                    recording.title().clone(),
                );
                work_ids.push(new_work.id().clone());
                Work::get_cache().set(&new_work).await?;
            } else {
                work_ids = work_ids.into_iter().unique().collect_vec();
            }

            for work_id in work_ids {
                counter.insert(&work_id, listen.clone());
            }

            progress_bar.inc(1);
        }

        Ok(())
    }
}

fn get_statistic_data_stream<L>(listens: L, stats_type: GroupByTarget) -> impl Stream<Item = color_eyre::Result<Vec<(String, Arc<Listen>)>>>
where L: IntoIterator<Item = Arc<Listen>> {
    use async_stream::stream;

    try_stream! {
        for listen in listens {
            let data = listen.get_statistic_data(stats_type).await?;

            yield stream! {
                for ele in data {
                    yield (ele, listen.clone());
                }
            }
        }
    }.flatten_unordered(None)
}