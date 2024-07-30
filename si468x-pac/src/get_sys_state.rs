#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command GetSysState

/// GET_SYS_STATE reports basic system state information such as which mode is active; FM, DAB, etc. The
/// command is complete when the CTS bit (and optional interrupt) is set. The ERR bit (and optional interrupt) is set if
/// an invalid argument is sent. Note that only a single interrupt occurs if both the CTS and ERR bits are set. The
/// command may only be sent in powerup mode. Note: GET_SYS_STATE command is not supported in firmware
/// revision A0A.

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
pub struct GetSysStateRequest {
    pub arg1: u8,
}

#[allow(clippy::derivable_impls)]
impl Default for GetSysStateRequest {
    fn default() -> Self {
        Self {
            arg1: 0x00,
        }
    }
}

impl Serialize for GetSysStateRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[1] = 0x00;

        (2, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct GetSysStateResponse {
    pub header: Header,
    pub image: ActiveProcessingImage,
    pub dummy: u8,
}

impl Deserialize<Self> for GetSysStateResponse {

    fn deserialize(buf: &[u8]) -> Result<GetSysStateResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let image = buf[4].deserialize_field(0, 7).try_into()?;
        let dummy = buf[5].deserialize_word();

        Ok(Self {
            header,
            image,
            dummy,
        })

    }

}

impl GetSysStateRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<GetSysStateResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 2;
    const RESPONSE_BUF_LEN: usize = 6;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, GetSysStateResponse> for GetSysStateRequest {}

impl Command for GetSysStateRequest {
    fn opcode(&self) -> u8 {
        0x9
    }
}