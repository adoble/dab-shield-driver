#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command GetPartInfo

/// GET_PART_INFO reports basic information about the device such as Part Number, Part Version, ROM ID, etc..

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
pub struct GetPartInfoRequest {
    pub arg1: u8,
}

#[allow(clippy::derivable_impls)]
impl Default for GetPartInfoRequest {
    fn default() -> Self {
        Self {
            arg1: 0x00,
        }
    }
}

impl Serialize for GetPartInfoRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;

        (1, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct GetPartInfoResponse {
    pub header: Header,
    pub chip_mask_revision: u8,
    pub rom_id: u8,
    pub part_number: u16,
    pub _padding: [u8; 13],
}

impl Deserialize<Self> for GetPartInfoResponse {

    fn deserialize(buf: &[u8]) -> Result<GetPartInfoResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let chip_mask_revision = buf[4].deserialize_word();
        let rom_id = buf[5].deserialize_word();
        let part_number = buf[8..=9].deserialize_word();
        let _padding = buf[10..].deserialize_repeating_words(13);

        Ok(Self {
            header,
            chip_mask_revision,
            rom_id,
            part_number,
            _padding,
        })

    }

}

impl GetPartInfoRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<GetPartInfoResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 1;
    const RESPONSE_BUF_LEN: usize = 23;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, GetPartInfoResponse> for GetPartInfoRequest {}

impl Command for GetPartInfoRequest {
    fn opcode(&self) -> u8 {
        0x8
    }
}