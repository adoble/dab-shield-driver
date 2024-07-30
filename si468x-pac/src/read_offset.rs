#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command ReadOffset

/// Used for applications that cannot read the entire response buffer. This type of application can
/// use this command to read the response buffer in segments

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
pub struct ReadOffsetRequest {
    pub dummy: u8,
    pub offset: u16,
}

#[allow(clippy::derivable_impls)]
impl Default for ReadOffsetRequest {
    fn default() -> Self {
        Self {
            dummy: 0x00,
            offset: Default::default(),
        }
    }
}

impl Serialize for ReadOffsetRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;
        data[1..=2].serialize_word(self.offset);

        (3, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct ReadOffsetResponse {
    pub header: Header,
    pub data: u8,
}

impl Deserialize<Self> for ReadOffsetResponse {

    fn deserialize(buf: &[u8]) -> Result<ReadOffsetResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let data = buf[4].deserialize_word();

        Ok(Self {
            header,
            data,
        })

    }

}

impl ReadOffsetRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<ReadOffsetResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 3;
    const RESPONSE_BUF_LEN: usize = 5;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, ReadOffsetResponse> for ReadOffsetRequest {}

impl Command for ReadOffsetRequest {
    fn opcode(&self) -> u8 {
        0x10
    }
}