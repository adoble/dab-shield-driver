#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command DabDigradStatus

/// B_DIGRAD_STATUS returns status information about the digital radio and ensemble including a change in
/// ensemble acquisition state, current estimates for ensemble's MSC (Main Service Channel) BER (bit error rate),
/// FIC (Fast Information Channel) BER along with number of FIBs (Fast Information Block) that failed a CRC check
/// and number of Reed-Solomon decoder errors (DAB+ and DMB only)

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
pub struct DabDigradStatusRequest {
    /// Clears the STC interrupt status when set.
    pub stc_ack: bool,
    /// Return the values as of DAB_VALID_RSSI_TIME after tune. Only the signal quality
/// metric RSSI is affected by setting this bit.
/// 0 : Return the current status
/// 1 : Return the snapshot taken at DAB_VALID_RSSI_TIME
    pub at_tune: bool,
    /// Clears all pending digital radio interrupts.
    pub digrad_ack: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for DabDigradStatusRequest {
    fn default() -> Self {
        Self {
            stc_ack: Default::default(),
            at_tune: Default::default(),
            digrad_ack: Default::default(),
        }
    }
}

impl Serialize for DabDigradStatusRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0].serialize_bit(self.stc_ack, 0);
        data[0].serialize_bit(self.at_tune, 2);
        data[0].serialize_bit(self.digrad_ack, 3);

        (1, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct DabDigradStatusResponse {
    pub header: Header,
    /// Indicates RSSI above DAB_DIGRAD_RSSI_HIGH_THRESHOLD
    pub received_signal_strength_indicator_low: bool,
    /// Indicates RSSI below DAB_DIGRAD_RSSI_LOW_THRESHOLD.
    pub received_signal_strength_indicator_high: bool,
    /// ndicates a change in the ensemble acquisition state.
    pub ensemble_acquisition_state_change_indicator: bool,
    /// Indicates that the FIC decoder has unrecoverable errors. Possibly due to low signal
    pub fast_information_channel_decoder_err_indicator: bool,
    /// Indicates that the audio had been muted. Possibly due to low signal
    pub hard_mute_indicator: bool,
    /// When set, the RSSI is at or above the valid threshold.
    pub received_signal_strength_indicator_valid: bool,
    /// Set to true when ensemble acquired.
    pub ensemble_acquired: bool,
    /// When set the ensemble is experiencing FIC errors.
    pub fast_information_channel_error: bool,
    /// Sets the mute status the audio
    pub hard_mute: AudioMute,
    /// Received signal strength indicator. Range: -128-63
    pub received_signal_strength_indicator: i8,
    /// SNR in dB.
    pub signal_noise_ratio: u8,
    /// Indicates the current estimate of the ensembles FIC quality. Range 0 - 100.
    pub fast_information_channel_quality: u8,
    /// CUrrent estimate of CNR in dB. Range 0 - 54.
    pub cnr: u8,
    /// Number of fast information blocks received with errors.
    pub fast_information_block_error_count: u16,
    /// Currently tuned frequency in kHz.
    pub tuned_frequency: u32,
    /// Indicates the frequency offset of the DQPSK tones of the OFDM signal relative to the center of the FFT bins of the digital demod
    pub fft_offset: u8,
    /// Antenna tuning capacitance value
    pub read_antenna_capacitance: u16,
    /// Number of currently decoded CUs
    pub cu_level: u16,
    /// Returns the statistical metric for DAB fast detect. 
    pub fast_dect: u8,
}

impl Deserialize<Self> for DabDigradStatusResponse {

    fn deserialize(buf: &[u8]) -> Result<DabDigradStatusResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let received_signal_strength_indicator_low = buf[4].deserialize_bit(0);
        let received_signal_strength_indicator_high = buf[4].deserialize_bit(1);
        let ensemble_acquisition_state_change_indicator = buf[4].deserialize_bit(2);
        let fast_information_channel_decoder_err_indicator = buf[4].deserialize_bit(3);
        let hard_mute_indicator = buf[4].deserialize_bit(4);
        let received_signal_strength_indicator_valid = buf[5].deserialize_bit(0);
        let ensemble_acquired = buf[5].deserialize_bit(2);
        let fast_information_channel_error = buf[5].deserialize_bit(3);
        let hard_mute = buf[5].deserialize_field(4, 4).try_into()?;
        let received_signal_strength_indicator = buf[6].deserialize_word();
        let signal_noise_ratio = buf[7].deserialize_word();
        let fast_information_channel_quality = buf[8].deserialize_word();
        let cnr = buf[9].deserialize_word();
        let fast_information_block_error_count = buf[10..=11].deserialize_word();
        let tuned_frequency = buf[12..=15].deserialize_word();
        let fft_offset = buf[17].deserialize_word();
        let read_antenna_capacitance = buf[18..=19].deserialize_word();
        let cu_level = buf[20..=21].deserialize_word();
        let fast_dect = buf[22].deserialize_word();

        Ok(Self {
            header,
            received_signal_strength_indicator_low,
            received_signal_strength_indicator_high,
            ensemble_acquisition_state_change_indicator,
            fast_information_channel_decoder_err_indicator,
            hard_mute_indicator,
            received_signal_strength_indicator_valid,
            ensemble_acquired,
            fast_information_channel_error,
            hard_mute,
            received_signal_strength_indicator,
            signal_noise_ratio,
            fast_information_channel_quality,
            cnr,
            fast_information_block_error_count,
            tuned_frequency,
            fft_offset,
            read_antenna_capacitance,
            cu_level,
            fast_dect,
        })

    }

}

impl DabDigradStatusRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<DabDigradStatusResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 1;
    const RESPONSE_BUF_LEN: usize = 23;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, DabDigradStatusResponse> for DabDigradStatusRequest {}

impl Command for DabDigradStatusRequest {
    fn opcode(&self) -> u8 {
        0xB2
    }
}