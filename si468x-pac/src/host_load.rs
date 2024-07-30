#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command HostLoad

/// HOST_LOAD loads an image from HOST over command interface

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

use crate::host_image_data_loader::HostImageDataLoader;

#[derive(Debug, PartialEq)]
pub struct HostLoadRequest {
    pub arg1: u8,
    pub arg2: u8,
    pub arg3: u8,
    pub image_data: HostImageDataLoader,
}

#[allow(clippy::derivable_impls)]
impl Default for HostLoadRequest {
    fn default() -> Self {
        Self {
            arg1: 0x00,
            arg2: 0x00,
            arg3: 0x00,
            image_data: Default::default(),
        }
    }
}

impl Serialize for HostLoadRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;
        data[1] = 0x00;
        data[2] = 0x00;
        let provider = self.image_data;

        (3, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct HostLoadResponse {
    pub header: Header,
}

impl Deserialize<Self> for HostLoadResponse {

    fn deserialize(buf: &[u8]) -> Result<HostLoadResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;

        Ok(Self {
            header,
        })

    }

}

impl HostLoadRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<HostLoadResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 4099;
    const RESPONSE_BUF_LEN: usize = 4;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, HostLoadResponse> for HostLoadRequest {}

impl Command for HostLoadRequest {
    fn opcode(&self) -> u8 {
        0x4
    }
}