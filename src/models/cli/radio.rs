use std::ops::Deref;

use clap::ArgAction;
use clap::{Parser, Subcommand};

use crate::core::statistics::listen_rate::ListenRate;
use crate::core::statistics::listen_rate::ListenRateRange;
use crate::models::config::Config;
use crate::models::radio::RadioConfig;
use crate::models::radio::RadioConfigBuilder;
use crate::tools::radio::circles::create_radio_mix;
use crate::tools::radio::listen_rate::listen_rate_radio;
use crate::tools::radio::overdue::overdue_radio;
use crate::tools::radio::underrated::underrated_mix;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct RadioCommand {
    #[command(subcommand)]
    pub command: RadioSubcommands,

    /// The minimum count of tracks the radio should add to the playlist. (Default: 50, gets overidden by `--min-duration`)
    #[arg(long)]
    min_count: Option<u64>,

    /// The minimum duration the playlist should last for. This accept natural language (Ex: "1 hour 36 mins")
    #[arg(long)]
    min_duration: Option<String>,
}

impl RadioCommand {
    pub fn get_config(&self) -> RadioConfig {
        let mut config_builder = RadioConfigBuilder::default();

        if let Some(val) = self.min_count {
            config_builder.min_count(val);
        }

        if let Some(val) = self.min_duration.as_ref() {
            let dura: humantime::Duration = val
                .clone()
                .parse()
                .expect("Couldn't parse mimimum lenght for radio");
            let std_dura = dura.deref();
            let chrono_dura = chrono::Duration::from_std(*std_dura).unwrap();
            config_builder.min_duration(chrono_dura);
        }

        config_builder.build().expect("Couldn't generate config")
    }

    pub async fn run(&self) -> color_eyre::Result<()> {
        let config = self.get_config();

        self.command.run(config).await
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum RadioSubcommands {
    /// Randomly adds recordings from artists you already listened to
    Circles {
        /// Name of the user to fetch listens from
        username: String,

        /// Your user token.
        ///
        /// You can find it at <https://listenbrainz.org/settings/>.
        /// If it's set in the config file, you can ignore this argument
        token: Option<String>,

        /// Use this flag to only get unlistened recordings. This is great for exploration playlists
        #[clap(long, action=ArgAction::SetTrue)]
        unlistened: bool,
    },

    /// Generate a playlist containing your underrated listens
    ///
    /// This radio will create a playlist containing all the tracks that you listen to, but seemingly no one else does.
    ///
    ///> The mix is made by calculating a score for each listen. This score is composed of two values:
    ///> - The rank in the user's top 1000 recording of all time (First place get 100 points, second get 999.9, etc...)
    ///> - The percentage of the recording's listens being from the user (Made with this formula: (user listens / worldwide listens) *100)
    Underrated {
        /// Name of the user to fetch listens from
        username: String,

        /// Your user token.
        ///
        /// You can find it at <https://listenbrainz.org/settings/>.
        /// If it's set in the config file, you can ignore this argument
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Generate playlists depending on the listen rate of recordings
    ///
    /// This algorythm bases itself on your listen rate of recording to get more forgotten tracks.
    /// It takes the recordings with the lowest listen rates, and put them into a playlist
    Rate {
        /// Name of the user to fetch listens from
        username: String,

        /// Your user token.
        ///
        /// You can find it at <https://listenbrainz.org/settings/>.
        /// If it's set in the config file, you can ignore this argument
        #[arg(short, long)]
        token: Option<String>,

        /// Minimum listen rate
        #[arg(long)]
        min_rate: Option<u64>,

        /// Minimum listen rate time range
        #[arg(long)]
        min_per: Option<ListenRateRange>,

        /// Minimum listen count
        #[arg(long)]
        min: Option<u64>,

        /// The amount of hours needed to wait after a recording have been given before it is re-suggested
        #[arg(short, long, default_value_t = 0)]
        cooldown: u64,
    },

    /// Generate playlists based on recording that the user should have listened to by now
    ///
    /// Similar to listen rates, this algorithm calculate the average time between listens, and estimate when the next listen will happen.
    /// It then put together a playlist made out of recordings you should have listened by now.
    Overdue {
        /// Name of the user to fetch listens from
        username: String,

        /// Your user token.
        ///
        /// You can find it at <https://listenbrainz.org/settings/>.
        /// If it's set in the config file, you can ignore this argument
        #[arg(short, long)]
        token: Option<String>,

        /// Minimum listen count
        #[arg(long)]
        min: Option<u64>,

        /// The amount of hours needed to wait after a recording have been given before it is re-suggested
        #[arg(short, long, default_value_t = 0)]
        cooldown: u64,

        /// Sort the recordings by the time overdue / the average time between listens
        ///
        /// Instead of sorting by date, the listens are sorted by how many estimated listens should have happened by now (Time elapsed since last listen / Average time per listens)

        #[arg(short, long, default_value_t = false)]
        overdue_factor: bool,
    },
}

impl RadioSubcommands {
    pub async fn run(&self, config: RadioConfig) -> color_eyre::Result<()> {
        match self {
            Self::Circles {
                username,
                token,
                unlistened,
                //cooldown
            } => {
                create_radio_mix(
                    username,
                    Config::get_token_or_argument(username, token),
                    *unlistened,
                    config,
                )
                .await;
            },

            Self::Underrated { username, token } => {
                underrated_mix(
                    username.clone(),
                    Config::get_token_or_argument(username, token),
                    config,
                )
                .await?;
            },

            Self::Rate {
                username,
                token,
                min_rate,
                min_per,
                min,
                cooldown,
            } => {
                let mut rate = None;

                if let Some(min_rate) = min_rate {
                    if let Some(min_per) = min_per {
                        rate = Some(ListenRate::new(
                            "*".to_string().into(),
                            *min_rate,
                            min_per.get_duration(),
                        ));
                    }
                }

                listen_rate_radio(
                    username,
                    &Config::get_token_or_argument(username, token),
                    rate,
                    *min,
                    *cooldown,
                    config,
                )
                .await?;
            },

            Self::Overdue {
                username,
                token,
                min,
                cooldown,
                overdue_factor: delay_factor,
            } => {
                overdue_radio(
                    username,
                    &Config::get_token_or_argument(username, token),
                    *min,
                    *cooldown,
                    *delay_factor,
                    config,
                )
                .await?;
            },
        }

        Ok(())
    }
}
