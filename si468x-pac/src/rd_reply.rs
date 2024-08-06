#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

/// Command RdReply

/// RD_REPLY command must be called to return the status byte and data for the last command sent to the device.
/// This command is also used to poll the status byte as needed.

/// Generated with version 0.2.1 of ddgen
use embedded_hal::spi::SpiDevice;

use crate::command::Command;
use crate::deserialize::Deserialize;
use crate::error::DeviceError;

use crate::request::{RequestArray, RequestBit, RequestField, RequestStruct, RequestWord};
use crate::response::{ResponseArray, ResponseBit, ResponseField, ResponseWord};
use crate::serialize::Serialize;
use crate::types::*;

use crate::transmit::Transmit;

use crate::header::Header;

#[derive(Debug, PartialEq)]
pub struct RdReplyRequest {}

#[allow(clippy::derivable_impls)]
impl Default for RdReplyRequest {
    fn default() -> Self {
        Self {}
    }
}

impl Serialize for RdReplyRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item = u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        (0, data, provider)
    }
}

#[derive(Debug, PartialEq)]
pub struct RdReplyResponse {
    pub header: Header,
    pub error_code: CommandError,
}

impl Deserialize<Self> for RdReplyResponse {
    fn deserialize(buf: &[u8]) -> Result<RdReplyResponse, DeviceError> {
        let header = Header::deserialize(&buf[0..=3])?;
        let error_code = buf[4].deserialize_field(0, 7).try_into()?;

        Ok(Self { header, error_code })
    }
}

impl RdReplyRequest {
    pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<RdReplyResponse, DeviceError> {
        const REQUEST_BUF_LEN: usize = 0;
        const RESPONSE_BUF_LEN: usize = 5;

        let response = self.transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN>(spi)?;
        Ok(response)
    }
}

impl<SPI: SpiDevice> Transmit<SPI, RdReplyResponse> for RdReplyRequest {}

impl Command for RdReplyRequest {
    fn opcode(&self) -> u8 {
        0x0
    }
}
