#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command DabGetAudioInfo

/// Gets information about the current audio service (decoder bps, audio mode)

/// Generated with version 0.2.1 of ddgen

use embedded_hal::spi::SpiDevice;

use crate::command::Command;
use crate::deserialize::Deserialize;
use crate::error::DeviceError;

use crate::request::{RequestArray, RequestBit, RequestField, RequestWord, RequestStruct};
use crate::response::{ResponseArray, ResponseBit, ResponseField, ResponseWord};
use crate::serialize::Serialize;
use crate::types::*;

use crate::transmit::Transmit;

use crate::header::Header;

#[derive(Debug, PartialEq)]
pub struct DabGetAudioInfoRequest {
    pub dummy: u8,
}

#[allow(clippy::derivable_impls)]
impl Default for DabGetAudioInfoRequest {
    fn default() -> Self {
        Self {
            dummy: 0x00,
        }
    }
}

impl Serialize for DabGetAudioInfoRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;

        (1, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct DabGetAudioInfoResponse {
    pub header: Header,
    /// Audio bit rate of the current audio service (kbps).
    pub audio_bit_rate: u16,
    /// Sample rate of the current audio service (Hz).
    pub audio_sample_rate: u16,
    /// Spectral Band Replication. Only for DAB+
    pub audio_sbr_flag: bool,
    /// PS usage. Only for DAB+
    pub audio_ps_flag: bool,
    pub audio_mode: AudioMode,
    /// The transmitter specified dynamic range control (DRC) gain of the current audio
/// service. The range of this field is from 0 to 63, representing 0 to 15.75dB in increment of 0.25dB.
    pub audio_drc_gain: u8,
}

impl Deserialize<Self> for DabGetAudioInfoResponse {

    fn deserialize(buf: &[u8]) -> Result<DabGetAudioInfoResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let audio_bit_rate = buf[4..=5].deserialize_word();
        let audio_sample_rate = buf[6..=7].deserialize_word();
        let audio_sbr_flag = buf[8].deserialize_bit(2);
        let audio_ps_flag = buf[8].deserialize_bit(3);
        let audio_mode = buf[8].deserialize_field(0, 1).try_into()?;
        let audio_drc_gain = buf[9].deserialize_word();

        Ok(Self {
            header,
            audio_bit_rate,
            audio_sample_rate,
            audio_sbr_flag,
            audio_ps_flag,
            audio_mode,
            audio_drc_gain,
        })

    }

}

impl DabGetAudioInfoRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<DabGetAudioInfoResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 1;
    const RESPONSE_BUF_LEN: usize = 10;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, DabGetAudioInfoResponse> for DabGetAudioInfoRequest {}

impl Command for DabGetAudioInfoRequest {
    fn opcode(&self) -> u8 {
        0xBD
    }
}