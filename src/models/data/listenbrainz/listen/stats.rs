use itertools::Itertools;

use crate::models::cli::common::GroupByTarget;
use super::Listen;

impl Listen {
    pub async fn get_statistic_data(&self, stats_type: GroupByTarget) -> color_eyre::Result<Vec<String>> {
        Ok(match stats_type {
            GroupByTarget::Artist => {todo!()},
            GroupByTarget::Recording => {self.get_recording_stats_data().await?.collect_vec()},
            GroupByTarget::Release => {todo!()},
            GroupByTarget::ReleaseGroup => {todo!()},
            GroupByTarget::Work => {todo!()}
        })
    }

    async fn get_recording_stats_data(&self) -> color_eyre::Result<impl Iterator<Item = String>> {
        Ok(vec![self.get_primary_recording_id().await?.expect("The listen should be mapped").to_string()].into_iter()) //TODO: Turn into a report instead of a panic
    }
}