/// Types used in the driver
///
/// Generated with version 0.2.1 of ddgen

use crate::error::DeviceError;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ProtectionProfile {
    #[default]
    EepModeLevelB4 = 13,
    UepModeLevel2 = 2,
    UepModeLevel5 = 5,
    EepModeLevelB2 = 11,
    UepModeLevel4 = 4,
    EepModeLevelA1 = 6,
    UepModeLevel3 = 3,
    EepModeLevelB1 = 10,
    EepModeLevelB3 = 12,
    UepModeLevel1 = 1,
    EepModeLevelA2 = 7,
    EepModeLevelA3 = 8,
    EepModeLevelA4 = 9,
}

impl TryFrom<u8> for ProtectionProfile {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            13 => Ok(Self::EepModeLevelB4),
            2 => Ok(Self::UepModeLevel2),
            5 => Ok(Self::UepModeLevel5),
            11 => Ok(Self::EepModeLevelB2),
            4 => Ok(Self::UepModeLevel4),
            6 => Ok(Self::EepModeLevelA1),
            3 => Ok(Self::UepModeLevel3),
            10 => Ok(Self::EepModeLevelB1),
            12 => Ok(Self::EepModeLevelB3),
            1 => Ok(Self::UepModeLevel1),
            7 => Ok(Self::EepModeLevelA2),
            8 => Ok(Self::EepModeLevelA3),
            9 => Ok(Self::EepModeLevelA4),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum CharSet {
    #[default]
    Utf16Be = 6,
    Utf8 = 15,
    EbuLatin = 0,
}

impl TryFrom<u8> for CharSet {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(Self::Utf16Be),
            15 => Ok(Self::Utf8),
            0 => Ok(Self::EbuLatin),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum PowerUpState {
    #[default]
    AppRunning = 3,
    BootloaderRunning = 2,
    Reset = 0,
}

impl TryFrom<u8> for PowerUpState {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::AppRunning),
            2 => Ok(Self::BootloaderRunning),
            0 => Ok(Self::Reset),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum Local {
    #[default]
    OverEntireEnsemble = 0,
    OverPartEnsemble = 1,
}

impl TryFrom<u8> for Local {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OverEntireEnsemble),
            1 => Ok(Self::OverPartEnsemble),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum AudioMute {
    #[default]
    Muted = 1,
    Unmuted = 0,
}

impl TryFrom<u8> for AudioMute {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Muted),
            0 => Ok(Self::Unmuted),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ServiceSelector {
    #[default]
    Data = 1,
    Audio = 0,
}

impl TryFrom<u8> for ServiceSelector {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Data),
            0 => Ok(Self::Audio),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ServiceMode {
    #[default]
    FicService = 6,
    MscDataPacketService = 3,
    DabPlus = 4,
    FidcService = 2,
    Dab = 5,
    NoMedia = 8,
    XpadData = 7,
    DataStreamService = 1,
    AudioStreamService = 0,
}

impl TryFrom<u8> for ServiceMode {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(Self::FicService),
            3 => Ok(Self::MscDataPacketService),
            4 => Ok(Self::DabPlus),
            2 => Ok(Self::FidcService),
            5 => Ok(Self::Dab),
            8 => Ok(Self::NoMedia),
            7 => Ok(Self::XpadData),
            1 => Ok(Self::DataStreamService),
            0 => Ok(Self::AudioStreamService),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ImageBuildLocation {
    #[default]
    Branch = 1,
    Tag = 0,
    Trunk = 2,
}

impl TryFrom<u8> for ImageBuildLocation {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Branch),
            0 => Ok(Self::Tag),
            2 => Ok(Self::Trunk),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum AudioMode {
    #[default]
    Stereo = 2,
    JointStereo = 3,
    Dual = 0,
    Mono = 1,
}

impl TryFrom<u8> for AudioMode {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Stereo),
            3 => Ok(Self::JointStereo),
            0 => Ok(Self::Dual),
            1 => Ok(Self::Mono),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum CaIdentifier {
    #[default]
    ComponentScrambled = 7,
    NoAccessControl = 0,
}

impl TryFrom<u8> for CaIdentifier {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            7 => Ok(Self::ComponentScrambled),
            0 => Ok(Self::NoAccessControl),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum TimeType {
    #[default]
    LocalTime = 0,
    Utc = 1,
}

impl TryFrom<u8> for TimeType {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LocalTime),
            1 => Ok(Self::Utc),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum Injection {
    #[default]
    Automatic = 0,
    HighSide = 2,
    LowSide = 1,
}

impl TryFrom<u8> for Injection {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Automatic),
            2 => Ok(Self::HighSide),
            1 => Ok(Self::LowSide),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ClockMode {
    #[default]
    OscillatorOffDifferentialBuffer = 3,
    XtalMode = 1,
    PoweredDown = 0,
    OscillatorOffSingleEndedBuffer = 2,
}

impl TryFrom<u8> for ClockMode {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::OscillatorOffDifferentialBuffer),
            1 => Ok(Self::XtalMode),
            0 => Ok(Self::PoweredDown),
            2 => Ok(Self::OscillatorOffSingleEndedBuffer),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ServiceProgramType {
    #[default]
    EasyListening = 12,
    Travel = 22,
    FolkMusic = 28,
    Culture = 7,
    CurrentAffairs = 2,
    ChildrensProgrammes = 18,
    JazzMusic = 24,
    WeatherMeteorology = 16,
    Information = 3,
    News = 1,
    SocialAffairs = 19,
    PhoneIn = 21,
    Drama = 6,
    Documentary = 29,
    Religion = 20,
    FinanceBusiness = 17,
    Sport = 4,
    Education = 5,
    OtherMusic = 15,
    RockMusic = 11,
    PopMusic = 10,
    Varied = 9,
    SeriousClassical = 14,
    NoProgrammeType = 0,
    LightClassical = 13,
    OldiesMusic = 27,
    NationalMusic = 26,
    Leisure = 23,
    CountryMusic = 25,
    Science = 8,
}

impl TryFrom<u8> for ServiceProgramType {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            12 => Ok(Self::EasyListening),
            22 => Ok(Self::Travel),
            28 => Ok(Self::FolkMusic),
            7 => Ok(Self::Culture),
            2 => Ok(Self::CurrentAffairs),
            18 => Ok(Self::ChildrensProgrammes),
            24 => Ok(Self::JazzMusic),
            16 => Ok(Self::WeatherMeteorology),
            3 => Ok(Self::Information),
            1 => Ok(Self::News),
            19 => Ok(Self::SocialAffairs),
            21 => Ok(Self::PhoneIn),
            6 => Ok(Self::Drama),
            29 => Ok(Self::Documentary),
            20 => Ok(Self::Religion),
            17 => Ok(Self::FinanceBusiness),
            4 => Ok(Self::Sport),
            5 => Ok(Self::Education),
            15 => Ok(Self::OtherMusic),
            11 => Ok(Self::RockMusic),
            10 => Ok(Self::PopMusic),
            9 => Ok(Self::Varied),
            14 => Ok(Self::SeriousClassical),
            0 => Ok(Self::NoProgrammeType),
            13 => Ok(Self::LightClassical),
            27 => Ok(Self::OldiesMusic),
            26 => Ok(Self::NationalMusic),
            23 => Ok(Self::Leisure),
            25 => Ok(Self::CountryMusic),
            8 => Ok(Self::Science),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ServiceType {
    #[default]
    DataService = 1,
    HdAudioInfo = 2,
    HdDataInfo = 3,
    AudioService = 0,
}

impl TryFrom<u8> for ServiceType {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::DataService),
            2 => Ok(Self::HdAudioInfo),
            3 => Ok(Self::HdDataInfo),
            0 => Ok(Self::AudioService),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ActiveProcessingImage {
    #[default]
    Amhd = 5,
    AmhdDemod = 6,
    TdmbOrDataOnlyDab = 3,
    DabDemod = 7,
    FmhdDemod = 4,
    Bootloader = 0,
    Dab = 2,
    Fmhd = 1,
}

impl TryFrom<u8> for ActiveProcessingImage {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(Self::Amhd),
            6 => Ok(Self::AmhdDemod),
            3 => Ok(Self::TdmbOrDataOnlyDab),
            7 => Ok(Self::DabDemod),
            4 => Ok(Self::FmhdDemod),
            0 => Ok(Self::Bootloader),
            2 => Ok(Self::Dab),
            1 => Ok(Self::Fmhd),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}