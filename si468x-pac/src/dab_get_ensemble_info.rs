#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command DabGetEnsembleInfo

/// gets information about the current ensemble such as the ensemble ID and label.

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
pub struct DabGetEnsembleInfoRequest {
    pub arg1: u8,
}

#[allow(clippy::derivable_impls)]
impl Default for DabGetEnsembleInfoRequest {
    fn default() -> Self {
        Self {
            arg1: 0x00,
        }
    }
}

impl Serialize for DabGetEnsembleInfoRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;

        (1, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct DabGetEnsembleInfoResponse {
    pub header: Header,
    /// Ensemble ID EID.
    pub eid: u16,
    /// Ensemble label characters.
    pub label: [u8; 16],
    /// Ensemble ECC.
    pub ensemble_extended_country_code: u8,
    /// Indicates which characters in the label are used to create the abbreviated label
    pub component_label_abbreviation_mask: u16,
}

impl Deserialize<Self> for DabGetEnsembleInfoResponse {

    fn deserialize(buf: &[u8]) -> Result<DabGetEnsembleInfoResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let eid = buf[4..=5].deserialize_word();
        let label = buf[6..].deserialize_repeating_words(16);
        let ensemble_extended_country_code = buf[22].deserialize_word();
        let component_label_abbreviation_mask = buf[24..=25].deserialize_word();

        Ok(Self {
            header,
            eid,
            label,
            ensemble_extended_country_code,
            component_label_abbreviation_mask,
        })

    }

}

impl DabGetEnsembleInfoRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<DabGetEnsembleInfoResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 1;
    const RESPONSE_BUF_LEN: usize = 26;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, DabGetEnsembleInfoResponse> for DabGetEnsembleInfoRequest {}

impl Command for DabGetEnsembleInfoRequest {
    fn opcode(&self) -> u8 {
        0xB4
    }
}