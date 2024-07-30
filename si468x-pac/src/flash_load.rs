#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command FlashLoad

/// FLASH_LOAD loads the firmware image from an externally attached SPI flash over the secondary SPI bus.

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
pub struct FlashLoadRequest {
    pub arg1: u8,
    pub arg2: u8,
    pub arg3: u8,
    /// Flash byte starting address of image to load
    pub flash_start_address: u32,
    pub arg8: u8,
    pub arg9: u8,
    pub arg10: u8,
    pub arg11: u8,
}

#[allow(clippy::derivable_impls)]
impl Default for FlashLoadRequest {
    fn default() -> Self {
        Self {
            arg1: 0x00,
            arg2: 0x00,
            arg3: 0x00,
            flash_start_address: Default::default(),
            arg8: 0x00,
            arg9: 0x00,
            arg10: 0x00,
            arg11: 0x00,
        }
    }
}

impl Serialize for FlashLoadRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;
        data[1] = 0x00;
        data[2] = 0x00;
        data[3..=6].serialize_word(self.flash_start_address);
        data[7] = 0x00;
        data[8] = 0x00;
        data[9] = 0x00;
        data[10] = 0x00;

        (11, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct FlashLoadResponse {
    pub header: Header,
}

impl Deserialize<Self> for FlashLoadResponse {

    fn deserialize(buf: &[u8]) -> Result<FlashLoadResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;

        Ok(Self {
            header,
        })

    }

}

impl FlashLoadRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<FlashLoadResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 11;
    const RESPONSE_BUF_LEN: usize = 4;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, FlashLoadResponse> for FlashLoadRequest {}

impl Command for FlashLoadRequest {
    fn opcode(&self) -> u8 {
        0x5
    }
}