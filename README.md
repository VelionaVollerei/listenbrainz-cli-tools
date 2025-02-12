# Listenbrainz CLI Tools

[![Crate](https://img.shields.io/crates/v/listenbrainz-cli-tools)](https://crates.io/crates/listenbrainz-cli-tools)
[![CI builder](https://github.com/RustyNova016/listenbrainz-cli-tools/actions/workflows/rust.yml/badge.svg)](https://github.com/RustyNova016/listenbrainz-cli-tools/actions/workflows/rust.yml)
[![License](https://img.shields.io/crates/l/listenbrainz-cli-tools)](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/LICENSE)

A collection of CLI based tools for Listenbrainz.

# Installing
## Builds
The latest build can be found in the [release](https://github.com/RustyNova016/listenbrainz-cli-tools/releases) tab

## Manual build
```shell
git clone https://github.com/RustyNova016/listenbrainz-cli-tools.git
cd ./listenbrainz-cli-tools
cargo build --release
```

# Usage

Full markdown help can be found under [docs/CommandLineHelp.md](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/docs/CommandLineHelp.md), but full `--help` support exists too. 

# Tools
## Unmapped listens 

[Usage > Command Line documentation](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/docs/CommandLineHelp.md#listenbrainz-cli-tools-mapping-list-unmapped)

This will list all your unmapped listens, grouped by similarity. 
It also gives a link to quickly look up the listen in listenbrainz, and go link it

```
(1) Paul's Dream (Dune) - Caster
    -> https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553
(7) Raise Your Weapon - KLOUD
    -> https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522
Total: 8 unlinked recordings
```

> Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens 
> by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.
> 
> This also means that the same recording can be shown twice in the list. 
> For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.

## Interactive mass mapper

[Usage > Command Line documentation](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/docs/CommandLineHelp.md#listenbrainz-cli-tools-mapping-mapper)

This tool allows for easy and faster mapping of recordings. It goes through each unmapped recordings, and give a few suggested recordings for the mapping. This is the exact same as mapping recording in the web UI.

## Statistics

[Usage > Command Line documentation](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/docs/CommandLineHelp.md#listenbrainz-cli-tools-stats)

While ListenBrainz have its own statistic page, it only refreshes daily, and is limited to only some entities. Furthermore, bugs in ListenBrainz statitics lead to misleading result.

This calculator aims to be most accurate possible, and will display your top listens for you.

## Radio

A few radio algorithms have been made to generate playlists for you

### Artist Circles

[Usage > Command Line documentation](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/docs/CommandLineHelp.md#listenbrainz-cli-tools-radio-circles)

This algorithm keep your playlist close to the artists you are listening to. The way it generate is as follow:

- Get a random listen
- Get its artist
- Add a random recording made by this artist

There is the option to only get unlistened recordings, making an alternative to ListenBrainz's own discovery playlists.

### Underrated tracks

[Usage > Command Line documentation](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/docs/CommandLineHelp.md#listenbrainz-cli-tools-radio-underrated)

This radio will create a playlist containing all the tracks that you listen to, but seemingly no one else does. 

> The mix is made by calculating a score for each listen. This score is composed of two values:
>
> - The rank in the user's top 1000 recording of all time (First place get 100 points, second get 999.9, etc...)
>
> - The percentage of the recording's listens being from the user (Made with this formula: (user listens / worldwide listens) *100)

> [!IMPORTANT]  
> As of the 2nd of july 2024, a bug in ListenBrainz made all global listen counts frozen. This radio will most likely be extremely inaccurrate by now. See the official status on [LB-1590](https://tickets.metabrainz.org/projects/LB/issues/LB-1590) 

### Listen rate

[Usage > Command Line documentation](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/docs/CommandLineHelp.md#listenbrainz-cli-tools-radio-rate)

This algorithm bases itself on your listen rate of recording to get more forgotten tracks. It takes the recordings with the lowest listen rates, and put them into a playlist


### Overdue listens

[Usage > Command Line documentation](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/docs/CommandLineHelp.md#listenbrainz-cli-tools-radio-rate)

Similar to listen rates, this algorithm calculate the average time between listens, and estimate when the next listen will happen. 
It thens put together a playlist made out of recordings you should have listened by now.

Another mode is the "Overdue factor". Instead of sorting by date, the listens are sorted by how many estimated listens should have happened by now (Time elapsed since last listen / Average time per listens)

# User Compatibility

[Usage > Command Line documentation](https://github.com/RustyNova016/listenbrainz-cli-tools/blob/master/docs/CommandLineHelp.md#listenbrainz-cli-tools-compatibility)

Similarly to Listenbrainz, a compatibility calculator is available, using a new algorithm that may provide more accurate results.

 The score is calculated as follow:
 - For each user and listened recording, the percentage of total listens being of this recording is calculated (total recording listens / total number of listens of user)
 - The lowest percent between the two user's is then added to the total score

 ### Exemple

 As an exemple, let's have two users: UserA and UserB. They both only share one listened track in common, being "Exemple Track".
 UserA has 5 listens on "Exemple Track", and 28 total listens. UserB has 12 listens on "Exemple Track", and 45 total listens.
 - This means that "Exemple Track" makes 17% of UserA's total listens, and 26% of UserB's listens.
 - We take the lowest percent between the two, meaning that we add 17% to the final score. 
 
 Since "Exemple Track" is the only shared track, this means that the final score is 17% compatibility


# Other infos

This project is in beta. There's a lot of features I'd like to add, and need a lot of testing before 1.0. If you find a bug, or have a feature request, feel free to create (and spam) a new issue.

# See also
- [musicbrainz_rs_nova](https://github.com/RustyNova016/musicbrainz_rs_nova): A fork of musicbrainz-rs as the original project seems to have staled
- [listenbrainz-rs](https://github.com/InputUsername/listenbrainz-rs): API bindings for listenbrainz
