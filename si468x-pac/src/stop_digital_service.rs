#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command StopDigitalService

/// START_DIGITAL_SERVICE stops an audio or data service.

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
pub struct StopDigitalServiceRequest {
    /// Sets the type of service to start for HD applications.
    pub service_type: ServiceSelector,
    pub arg2: u8,
    pub arg3: u8,
    /// The service's Service Number (HD).
    pub service_id: u32,
    /// The service's Port/Program Number (HD)
    pub comp_id: u32,
}

#[allow(clippy::derivable_impls)]
impl Default for StopDigitalServiceRequest {
    fn default() -> Self {
        Self {
            service_type: Default::default(),
            arg2: 0x00,
            arg3: 0x00,
            service_id: Default::default(),
            comp_id: Default::default(),
        }
    }
}

impl Serialize for StopDigitalServiceRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0].serialize_field(self.service_type as u8, 0, 0);
        data[1] = 0x00;
        data[2] = 0x00;
        data[3..=6].serialize_word(self.service_id);
        data[7..=10].serialize_word(self.comp_id);

        (11, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct StopDigitalServiceResponse {
    pub header: Header,
}

impl Deserialize<Self> for StopDigitalServiceResponse {

    fn deserialize(buf: &[u8]) -> Result<StopDigitalServiceResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;

        Ok(Self {
            header,
        })

    }

}

impl StopDigitalServiceRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<StopDigitalServiceResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 11;
    const RESPONSE_BUF_LEN: usize = 4;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, StopDigitalServiceResponse> for StopDigitalServiceRequest {}

impl Command for StopDigitalServiceRequest {
    fn opcode(&self) -> u8 {
        0x82
    }
}