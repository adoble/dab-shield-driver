#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command DabSetFreqList

/// DAB_SET_FREQ_LIST command sets the DAB frequency table. The frequencies are in units of 1 kHz. The table
/// can be populated with a single entry or a regional list (for example 5 or 6 entries]. It is recommended to make the
/// list regional to increase scanning speed. The command is complete when the CTS bit (and optional interrupt) is
/// set. The ERR bit (and optional interrupt) is set if an invalid argument is sent. Note that only a single interrupt occurs
/// if both the CTS and ERR bits are set. The command may only be sent in powerup mode

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
pub struct DabSetFreqListRequest {
    pub number_frequencies: u8,
    pub frequencies: [u32; 48],
}

#[allow(clippy::derivable_impls)]
impl Default for DabSetFreqListRequest {
    fn default() -> Self {
        Self {
            number_frequencies: Default::default(),
            frequencies: [0; 48],
        }
    }
}

impl Serialize for DabSetFreqListRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0].serialize_word(self.number_frequencies);
        data[3..].serialize_repeating_words(self.frequencies, self.number_frequencies as usize);

        (1 + (4 * self.number_frequencies as usize), data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct DabSetFreqListResponse {
    pub header: Header,
}

impl Deserialize<Self> for DabSetFreqListResponse {

    fn deserialize(buf: &[u8]) -> Result<DabSetFreqListResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;

        Ok(Self {
            header,
        })

    }

}

impl DabSetFreqListRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<DabSetFreqListResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 195;
    const RESPONSE_BUF_LEN: usize = 4;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, DabSetFreqListResponse> for DabSetFreqListRequest {}

impl Command for DabSetFreqListRequest {
    fn opcode(&self) -> u8 {
        0xB8
    }
}