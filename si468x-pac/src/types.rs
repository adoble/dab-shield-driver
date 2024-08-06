/// Types used in the driver
///
/// Generated with version 0.2.1 of ddgen

use crate::error::DeviceError;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ServiceType {
    #[default]
    HdDataInfo = 3,
    AudioService = 0,
    DataService = 1,
    HdAudioInfo = 2,
}

impl TryFrom<u8> for ServiceType {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::HdDataInfo),
            0 => Ok(Self::AudioService),
            1 => Ok(Self::DataService),
            2 => Ok(Self::HdAudioInfo),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ServiceSelector {
    #[default]
    Audio = 0,
    Data = 1,
}

impl TryFrom<u8> for ServiceSelector {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Audio),
            1 => Ok(Self::Data),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ServiceMode {
    #[default]
    DataStreamService = 1,
    FidcService = 2,
    FicService = 6,
    MscDataPacketService = 3,
    DabPlus = 4,
    NoMedia = 8,
    Dab = 5,
    XpadData = 7,
    AudioStreamService = 0,
}

impl TryFrom<u8> for ServiceMode {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::DataStreamService),
            2 => Ok(Self::FidcService),
            6 => Ok(Self::FicService),
            3 => Ok(Self::MscDataPacketService),
            4 => Ok(Self::DabPlus),
            8 => Ok(Self::NoMedia),
            5 => Ok(Self::Dab),
            7 => Ok(Self::XpadData),
            0 => Ok(Self::AudioStreamService),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum PowerUpState {
    #[default]
    BootloaderRunning = 2,
    AppRunning = 3,
    Reset = 0,
}

impl TryFrom<u8> for PowerUpState {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::BootloaderRunning),
            3 => Ok(Self::AppRunning),
            0 => Ok(Self::Reset),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ImageBuildLocation {
    #[default]
    Trunk = 2,
    Tag = 0,
    Branch = 1,
}

impl TryFrom<u8> for ImageBuildLocation {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Trunk),
            0 => Ok(Self::Tag),
            1 => Ok(Self::Branch),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum AudioMode {
    #[default]
    Mono = 1,
    Dual = 0,
    Stereo = 2,
    JointStereo = 3,
}

impl TryFrom<u8> for AudioMode {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Mono),
            0 => Ok(Self::Dual),
            2 => Ok(Self::Stereo),
            3 => Ok(Self::JointStereo),
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
pub enum ClockMode {
    #[default]
    OscillatorOffSingleEndedBuffer = 2,
    XtalMode = 1,
    OscillatorOffDifferentialBuffer = 3,
    PoweredDown = 0,
}

impl TryFrom<u8> for ClockMode {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::OscillatorOffSingleEndedBuffer),
            1 => Ok(Self::XtalMode),
            3 => Ok(Self::OscillatorOffDifferentialBuffer),
            0 => Ok(Self::PoweredDown),
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
pub enum CommandError {
    #[default]
    NotAvailable = 3,
    BadFrequency = 5,
    AtBandLimit = 25,
    ReplyOverflow = 2,
    BadArg3 = 19,
    NotAcquired = 80,
    BadNvm = 32,
    Unspecified = 1,
    BadArg6 = 22,
    NotSupported = 4,
    BadProperty = 64,
    CommandBusy = 24,
    BadArg7 = 23,
    BadArg5 = 21,
    BadArg1 = 17,
    CommandNotFound = 16,
    BadBootmode = 49,
    BadPatch = 48,
    BadArg4 = 20,
    AppNotSupported = 255,
    BadArg2 = 18,
}

impl TryFrom<u8> for CommandError {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::NotAvailable),
            5 => Ok(Self::BadFrequency),
            25 => Ok(Self::AtBandLimit),
            2 => Ok(Self::ReplyOverflow),
            19 => Ok(Self::BadArg3),
            80 => Ok(Self::NotAcquired),
            32 => Ok(Self::BadNvm),
            1 => Ok(Self::Unspecified),
            22 => Ok(Self::BadArg6),
            4 => Ok(Self::NotSupported),
            64 => Ok(Self::BadProperty),
            24 => Ok(Self::CommandBusy),
            23 => Ok(Self::BadArg7),
            21 => Ok(Self::BadArg5),
            17 => Ok(Self::BadArg1),
            16 => Ok(Self::CommandNotFound),
            49 => Ok(Self::BadBootmode),
            48 => Ok(Self::BadPatch),
            20 => Ok(Self::BadArg4),
            255 => Ok(Self::AppNotSupported),
            18 => Ok(Self::BadArg2),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ProtectionProfile {
    #[default]
    EepModeLevelB3 = 12,
    EepModeLevelA3 = 8,
    UepModeLevel4 = 4,
    UepModeLevel3 = 3,
    UepModeLevel2 = 2,
    UepModeLevel5 = 5,
    EepModeLevelA1 = 6,
    EepModeLevelA4 = 9,
    EepModeLevelB1 = 10,
    EepModeLevelB4 = 13,
    UepModeLevel1 = 1,
    EepModeLevelB2 = 11,
    EepModeLevelA2 = 7,
}

impl TryFrom<u8> for ProtectionProfile {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            12 => Ok(Self::EepModeLevelB3),
            8 => Ok(Self::EepModeLevelA3),
            4 => Ok(Self::UepModeLevel4),
            3 => Ok(Self::UepModeLevel3),
            2 => Ok(Self::UepModeLevel2),
            5 => Ok(Self::UepModeLevel5),
            6 => Ok(Self::EepModeLevelA1),
            9 => Ok(Self::EepModeLevelA4),
            10 => Ok(Self::EepModeLevelB1),
            13 => Ok(Self::EepModeLevelB4),
            1 => Ok(Self::UepModeLevel1),
            11 => Ok(Self::EepModeLevelB2),
            7 => Ok(Self::EepModeLevelA2),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum CaIdentifier {
    #[default]
    NoAccessControl = 0,
    ComponentScrambled = 7,
}

impl TryFrom<u8> for CaIdentifier {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoAccessControl),
            7 => Ok(Self::ComponentScrambled),
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
pub enum ServiceProgramType {
    #[default]
    Drama = 6,
    FinanceBusiness = 17,
    Religion = 20,
    JazzMusic = 24,
    Education = 5,
    RockMusic = 11,
    Culture = 7,
    Travel = 22,
    OldiesMusic = 27,
    PopMusic = 10,
    FolkMusic = 28,
    CurrentAffairs = 2,
    Documentary = 29,
    Leisure = 23,
    Information = 3,
    PhoneIn = 21,
    Science = 8,
    Varied = 9,
    Sport = 4,
    SocialAffairs = 19,
    OtherMusic = 15,
    News = 1,
    NationalMusic = 26,
    LightClassical = 13,
    WeatherMeteorology = 16,
    ChildrensProgrammes = 18,
    CountryMusic = 25,
    SeriousClassical = 14,
    NoProgrammeType = 0,
    EasyListening = 12,
}

impl TryFrom<u8> for ServiceProgramType {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(Self::Drama),
            17 => Ok(Self::FinanceBusiness),
            20 => Ok(Self::Religion),
            24 => Ok(Self::JazzMusic),
            5 => Ok(Self::Education),
            11 => Ok(Self::RockMusic),
            7 => Ok(Self::Culture),
            22 => Ok(Self::Travel),
            27 => Ok(Self::OldiesMusic),
            10 => Ok(Self::PopMusic),
            28 => Ok(Self::FolkMusic),
            2 => Ok(Self::CurrentAffairs),
            29 => Ok(Self::Documentary),
            23 => Ok(Self::Leisure),
            3 => Ok(Self::Information),
            21 => Ok(Self::PhoneIn),
            8 => Ok(Self::Science),
            9 => Ok(Self::Varied),
            4 => Ok(Self::Sport),
            19 => Ok(Self::SocialAffairs),
            15 => Ok(Self::OtherMusic),
            1 => Ok(Self::News),
            26 => Ok(Self::NationalMusic),
            13 => Ok(Self::LightClassical),
            16 => Ok(Self::WeatherMeteorology),
            18 => Ok(Self::ChildrensProgrammes),
            25 => Ok(Self::CountryMusic),
            14 => Ok(Self::SeriousClassical),
            0 => Ok(Self::NoProgrammeType),
            12 => Ok(Self::EasyListening),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ActiveProcessingImage {
    #[default]
    FmhdDemod = 4,
    Dab = 2,
    Amhd = 5,
    Bootloader = 0,
    AmhdDemod = 6,
    TdmbOrDataOnlyDab = 3,
    DabDemod = 7,
    Fmhd = 1,
}

impl TryFrom<u8> for ActiveProcessingImage {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(Self::FmhdDemod),
            2 => Ok(Self::Dab),
            5 => Ok(Self::Amhd),
            0 => Ok(Self::Bootloader),
            6 => Ok(Self::AmhdDemod),
            3 => Ok(Self::TdmbOrDataOnlyDab),
            7 => Ok(Self::DabDemod),
            1 => Ok(Self::Fmhd),
            _ => Err(DeviceError::EnumConversion),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum Injection {
    #[default]
    HighSide = 2,
    LowSide = 1,
    Automatic = 0,
}

impl TryFrom<u8> for Injection {
    type Error = DeviceError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::HighSide),
            1 => Ok(Self::LowSide),
            0 => Ok(Self::Automatic),
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