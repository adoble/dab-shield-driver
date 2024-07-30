#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command DabGetDigitalServiceList

/// GET_DIGITAL_SERVICE_LIST gets a service list of the ensemble.

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
pub struct DabGetDigitalServiceListRequest {
    /// Sets the type of service list to retrieve.
    pub service_type: ServiceType,
}

#[allow(clippy::derivable_impls)]
impl Default for DabGetDigitalServiceListRequest {
    fn default() -> Self {
        Self {
            service_type: Default::default(),
        }
    }
}

impl Serialize for DabGetDigitalServiceListRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0].serialize_field(self.service_type as u8, 0, 1);

        (1, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct DabGetDigitalServiceListResponse {
    pub header: Header,
    /// The size of the service list in bytes.
    pub service_size: u16,
    /// A max of 2047 bytes of service information
    pub data: [u16; 2047],
}

impl Deserialize<Self> for DabGetDigitalServiceListResponse {

    fn deserialize(buf: &[u8]) -> Result<DabGetDigitalServiceListResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let service_size = buf[4..=5].deserialize_word();
        let data = buf[6..].deserialize_repeating_words(service_size as usize);

        Ok(Self {
            header,
            service_size,
            data,
        })

    }

}

impl DabGetDigitalServiceListRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<DabGetDigitalServiceListResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 1;
    const RESPONSE_BUF_LEN: usize = 4100;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, DabGetDigitalServiceListResponse> for DabGetDigitalServiceListRequest {}

impl Command for DabGetDigitalServiceListRequest {
    fn opcode(&self) -> u8 {
        0x80
    }
}