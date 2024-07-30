#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command DabGetEventStatus

/// Gets information about the various events related to the DAB radio

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
pub struct DabGetEventStatusRequest {
    /// Clears all pending digital radion event interrupts.
    pub event_ack: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for DabGetEventStatusRequest {
    fn default() -> Self {
        Self {
            event_ack: Default::default(),
        }
    }
}

impl Serialize for DabGetEventStatusRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0].serialize_bit(self.event_ack, 0);

        (1, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct DabGetEventStatusResponse {
    pub header: Header,
    /// Indicates that a new digital service list is available.
    pub service_list_interrupt: bool,
    /// Indicates that new frequency infomation is available.
    pub frequency_infomation_interrupt: bool,
    /// New announcement info interrupt.
    pub announcement_interrupt: bool,
    /// Ensemble reconfiguration warning. RECFGWRNINT bit
    pub ensemble_reconfiguration_warning: bool,
    /// Ensemble reconfiguration event. RECFGINT bit.
    pub ensemble_reconfiguration_event: bool,
    pub service_list_available: bool,
    /// Indicates that frequency information is available.
    pub freqency_information_available: bool,
    /// Current version of the digital service list.
    pub service_list_version: u16,
}

impl Deserialize<Self> for DabGetEventStatusResponse {

    fn deserialize(buf: &[u8]) -> Result<DabGetEventStatusResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let service_list_interrupt = buf[4].deserialize_bit(0);
        let frequency_infomation_interrupt = buf[4].deserialize_bit(1);
        let announcement_interrupt = buf[4].deserialize_bit(3);
        let ensemble_reconfiguration_warning = buf[4].deserialize_bit(6);
        let ensemble_reconfiguration_event = buf[4].deserialize_bit(7);
        let service_list_available = buf[5].deserialize_bit(0);
        let freqency_information_available = buf[5].deserialize_bit(1);
        let service_list_version = buf[6..=7].deserialize_word();

        Ok(Self {
            header,
            service_list_interrupt,
            frequency_infomation_interrupt,
            announcement_interrupt,
            ensemble_reconfiguration_warning,
            ensemble_reconfiguration_event,
            service_list_available,
            freqency_information_available,
            service_list_version,
        })

    }

}

impl DabGetEventStatusRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<DabGetEventStatusResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 1;
    const RESPONSE_BUF_LEN: usize = 8;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, DabGetEventStatusResponse> for DabGetEventStatusRequest {}

impl Command for DabGetEventStatusRequest {
    fn opcode(&self) -> u8 {
        0xB3
    }
}