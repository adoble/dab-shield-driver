#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command DabGetSubchanInfo

/// Gets information about the sub-channel (service mode, protection, subchannel bps).

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
pub struct DabGetSubchanInfoRequest {
    pub dummy: [u8; 3],
    pub service_id: u32,
    pub component_id: u32,
}

#[allow(clippy::derivable_impls)]
impl Default for DabGetSubchanInfoRequest {
    fn default() -> Self {
        Self {
            dummy: [0; 3],
            service_id: Default::default(),
            component_id: Default::default(),
        }
    }
}

impl Serialize for DabGetSubchanInfoRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;
        data[3..=6].serialize_word(self.service_id);
        data[7..=10].serialize_word(self.component_id);

        (11, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct DabGetSubchanInfoResponse {
    pub header: Header,
    pub service_mode: ServiceMode,
    /// Protection profile of the sub-channel.
    pub protection_info: ProtectionProfile,
    /// Sub-channel bit rate (kpbs).
    pub sub_channel_bit_rate: u16,
    /// Number of Capacity units assigned to this service component.
    pub number_capacity_units: u16,
    /// CU starting address of this subchannel within the CIF
    pub capacity_unit_address: u16,
}

impl Deserialize<Self> for DabGetSubchanInfoResponse {

    fn deserialize(buf: &[u8]) -> Result<DabGetSubchanInfoResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let service_mode = buf[4].deserialize_field(0, 7).try_into()?;
        let protection_info = buf[5].deserialize_field(0, 7).try_into()?;
        let sub_channel_bit_rate = buf[6..=7].deserialize_word();
        let number_capacity_units = buf[8..=9].deserialize_word();
        let capacity_unit_address = buf[10..=11].deserialize_word();

        Ok(Self {
            header,
            service_mode,
            protection_info,
            sub_channel_bit_rate,
            number_capacity_units,
            capacity_unit_address,
        })

    }

}

impl DabGetSubchanInfoRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<DabGetSubchanInfoResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 11;
    const RESPONSE_BUF_LEN: usize = 12;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, DabGetSubchanInfoResponse> for DabGetSubchanInfoRequest {}

impl Command for DabGetSubchanInfoRequest {
    fn opcode(&self) -> u8 {
        0xBE
    }
}