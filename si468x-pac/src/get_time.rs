#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command GetTime

/// Gets the ensemble time adjusted for the local time offset

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
pub struct GetTimeRequest {
    pub time_type: TimeType,
}

#[allow(clippy::derivable_impls)]
impl Default for GetTimeRequest {
    fn default() -> Self {
        Self {
            time_type: Default::default(),
        }
    }
}

impl Serialize for GetTimeRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0].serialize_field(self.time_type as u8, 0, 7);

        (1, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct GetTimeResponse {
    pub header: Header,
    pub year: u16,
    pub months: u8,
    pub days: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Deserialize<Self> for GetTimeResponse {

    fn deserialize(buf: &[u8]) -> Result<GetTimeResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let year = buf[4..=5].deserialize_word();
        let months = buf[6].deserialize_word();
        let days = buf[7].deserialize_word();
        let hours = buf[8].deserialize_word();
        let minutes = buf[9].deserialize_word();
        let seconds = buf[10].deserialize_word();

        Ok(Self {
            header,
            year,
            months,
            days,
            hours,
            minutes,
            seconds,
        })

    }

}

impl GetTimeRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<GetTimeResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 1;
    const RESPONSE_BUF_LEN: usize = 11;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, GetTimeResponse> for GetTimeRequest {}

impl Command for GetTimeRequest {
    fn opcode(&self) -> u8 {
        0xBC
    }
}