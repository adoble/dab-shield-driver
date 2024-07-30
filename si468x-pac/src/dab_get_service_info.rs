#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command DabGetServiceInfo

/// Gets information about a service

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
pub struct DabGetServiceInfoRequest {
    pub dummy1: u8,
    pub dummy2: u8,
    pub dummy3: u8,
    pub service_id: u32,
}

#[allow(clippy::derivable_impls)]
impl Default for DabGetServiceInfoRequest {
    fn default() -> Self {
        Self {
            dummy1: 0x00,
            dummy2: 0x00,
            dummy3: 0x00,
            service_id: Default::default(),
        }
    }
}

impl Serialize for DabGetServiceInfoRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;
        data[1] = 0x00;
        data[2] = 0x00;
        data[3..=6].serialize_word(self.service_id);

        (7, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct DabGetServiceInfoResponse {
    pub header: Header,
    /// Set to true if service linking info is available for this service.
    pub service_linking_info: bool,
    pub pd_flag: ServiceSelector,
    pub service_program_type: ServiceProgramType,
    /// Number of components in this service.
    pub number_components: u8,
    /// Access control used.
    pub ca_id: CaIdentifier,
    /// Service is available over the entire or part of the ensemble service area.
    pub local: Local,
    /// Indicates char set
    pub si_char_set: CharSet,
    pub service_extended_country_code: u8,
    /// Service label characters
    pub label: [u8; 16],
    /// Used to indicate which characters in the label are used to create the abbreviated label.
    pub service_label_abbreviation_mask: u16,
}

impl Deserialize<Self> for DabGetServiceInfoResponse {

    fn deserialize(buf: &[u8]) -> Result<DabGetServiceInfoResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let service_linking_info = buf[4].deserialize_bit(6);
        let pd_flag = buf[4].deserialize_field(0, 0).try_into()?;
        let service_program_type = buf[4].deserialize_field(1, 5).try_into()?;
        let number_components = buf[5].deserialize_field(0, 3);
        let ca_id = buf[5].deserialize_field(4, 6).try_into()?;
        let local = buf[5].deserialize_field(7, 7).try_into()?;
        let si_char_set = buf[6].deserialize_field(0, 3).try_into()?;
        let service_extended_country_code = buf[7].deserialize_word();
        let label = buf[8..].deserialize_repeating_words(16);
        let service_label_abbreviation_mask = buf[24..=25].deserialize_word();

        Ok(Self {
            header,
            service_linking_info,
            pd_flag,
            service_program_type,
            number_components,
            ca_id,
            local,
            si_char_set,
            service_extended_country_code,
            label,
            service_label_abbreviation_mask,
        })

    }

}

impl DabGetServiceInfoRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<DabGetServiceInfoResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 7;
    const RESPONSE_BUF_LEN: usize = 26;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, DabGetServiceInfoResponse> for DabGetServiceInfoRequest {}

impl Command for DabGetServiceInfoRequest {
    fn opcode(&self) -> u8 {
        0xC0
    }
}