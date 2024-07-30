#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command SetProperty

/// SET_PROPERTY sets the value of a property

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
pub struct SetPropertyRequest {
    pub arg1: u8,
    /// ID of the property to set.
    pub property_id: u16,
    /// Value for the written property.
    pub data: i16,
}

#[allow(clippy::derivable_impls)]
impl Default for SetPropertyRequest {
    fn default() -> Self {
        Self {
            arg1: 0x00,
            property_id: Default::default(),
            data: Default::default(),
        }
    }
}

impl Serialize for SetPropertyRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;
        data[1..=2].serialize_word(self.property_id);
        data[3..=4].serialize_word(self.data);

        (5, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct SetPropertyResponse {
    pub header: Header,
}

impl Deserialize<Self> for SetPropertyResponse {

    fn deserialize(buf: &[u8]) -> Result<SetPropertyResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;

        Ok(Self {
            header,
        })

    }

}

impl SetPropertyRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<SetPropertyResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 5;
    const RESPONSE_BUF_LEN: usize = 4;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, SetPropertyResponse> for SetPropertyRequest {}

impl Command for SetPropertyRequest {
    fn opcode(&self) -> u8 {
        0x13
    }
}