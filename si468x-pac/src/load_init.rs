#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command LoadInit

/// LOAD_INIT prepares the bootloader to receive a new image.

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
pub struct LoadInitRequest {
    pub arg1: u8,
}

#[allow(clippy::derivable_impls)]
impl Default for LoadInitRequest {
    fn default() -> Self {
        Self {
            arg1: 0x00,
        }
    }
}

impl Serialize for LoadInitRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;

        (1, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct LoadInitResponse {
    pub header: Header,
}

impl Deserialize<Self> for LoadInitResponse {

    fn deserialize(buf: &[u8]) -> Result<LoadInitResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;

        Ok(Self {
            header,
        })

    }

}

impl LoadInitRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<LoadInitResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 1;
    const RESPONSE_BUF_LEN: usize = 4;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, LoadInitResponse> for LoadInitRequest {}

impl Command for LoadInitRequest {
    fn opcode(&self) -> u8 {
        0x6
    }
}