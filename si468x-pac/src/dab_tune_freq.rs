#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command DabTuneFreq

/// DAB_TUNE_FREQ sets the DAB Receiver to tune to a frequency between 168.16 MHz and 239.20 MHz defined
/// by the table through DAB_SET_FREQ_LIST.

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
pub struct DabTuneFreqRequest {
    pub injection: Injection,
    /// Frequency index for the tuned frequency.
    pub freq_index: u8,
    /// Antenna tuning capacitor value in 250 fF units (31.75 pF Max). Range: 0-128.
/// 0 : Automatically determines the cap setting.
    pub antenna_capacitance: u16,
}

#[allow(clippy::derivable_impls)]
impl Default for DabTuneFreqRequest {
    fn default() -> Self {
        Self {
            injection: Default::default(),
            freq_index: Default::default(),
            antenna_capacitance: Default::default(),
        }
    }
}

impl Serialize for DabTuneFreqRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0].serialize_field(self.injection as u8, 0, 1);
        data[1].serialize_word(self.freq_index);
        data[3..=4].serialize_word(self.antenna_capacitance);

        (5, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct DabTuneFreqResponse {
    pub header: Header,
}

impl Deserialize<Self> for DabTuneFreqResponse {

    fn deserialize(buf: &[u8]) -> Result<DabTuneFreqResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;

        Ok(Self {
            header,
        })

    }

}

impl DabTuneFreqRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<DabTuneFreqResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 5;
    const RESPONSE_BUF_LEN: usize = 4;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, DabTuneFreqResponse> for DabTuneFreqRequest {}

impl Command for DabTuneFreqRequest {
    fn opcode(&self) -> u8 {
        0xB0
    }
}