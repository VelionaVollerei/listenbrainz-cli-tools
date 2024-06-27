use crate::models::data::musicbrainz::mbid::MBIDEnum;

#[derive(Debug, Clone, Copy)]
pub enum MusicbrainzEntityKind {
    Area,
    Artist,
    Event,
    Genre,
    Instrument,
    Label,
    Place,
    Recording,
    Release,
    ReleaseGroup,
    Series,
    URL,
    Work,
}

impl MusicbrainzEntityKind {
    pub fn to_mbid(&self, data: String) -> MBIDEnum {
        match self {
            Self::Artist => MBIDEnum::Artist(data.into()),
            Self::Recording => MBIDEnum::Recording(data.into()),
            Self::Release => MBIDEnum::Release(data.into()),
            Self::ReleaseGroup => MBIDEnum::ReleaseGroup(data.into()),
            Self::Work => MBIDEnum::Work(data.into()),
            _ => todo!(),
        }
    }
}

impl From<MBIDEnum> for MusicbrainzEntityKind {
    fn from(value: MBIDEnum) -> Self {
        match value {
            MBIDEnum::Artist(_) => Self::Artist,
            MBIDEnum::Recording(_) => Self::Recording,
            MBIDEnum::Release(_) => Self::Release,
            MBIDEnum::ReleaseGroup(_) => Self::ReleaseGroup,
            MBIDEnum::Work(_) => Self::Work,
        }
    }
}
