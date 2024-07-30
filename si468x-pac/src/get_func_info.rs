#![allow(unused_imports)]

#![allow(clippy::unnecessary_cast)]

/// Command GetFuncInfo

/// GET_FUNC_INFO returns the function revision number for currently loaded firmware (FMHD, AM etc.) as opposed
/// to GET_PART_INFO command that provides the revision number for the combo firmware. For example,
/// GET_PART_INFO would return A0B is the firmware revision while GET_FUNC_INFO would return 1.0.4 for FM
/// function revision if the currently running firmware function is FM.

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
pub struct GetFuncInfoRequest {
    pub arg1: u8,
}

#[allow(clippy::derivable_impls)]
impl Default for GetFuncInfoRequest {
    fn default() -> Self {
        Self {
            arg1: 0x00,
        }
    }
}

impl Serialize for GetFuncInfoRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0] = 0x00;

        (1, data, provider)
    }

}

#[derive(Debug, PartialEq)]
pub struct GetFuncInfoResponse {
    pub header: Header,
    /// First part of 1.2.3
    pub major_revision_number: u8,
    /// Second part of 1.2.3
    pub minor_revision_number: u8,
    /// Third part of 1.2.3
    pub build_revision_number: u8,
    /// If set, the image has local modifications.
    pub local_modifications: bool,
    /// If set, the image was built from mixed revisions.
    pub mixed_revisions: bool,
    /// If set the build was created with no SVN info.
    pub no_svn: bool,
    /// The location from which the image was built.
    pub location: ImageBuildLocation,
    /// SVN ID for which the image was built.
    pub svn_id: u32,
}

impl Deserialize<Self> for GetFuncInfoResponse {

    fn deserialize(buf: &[u8]) -> Result<GetFuncInfoResponse, DeviceError> {

        let header = Header::deserialize(&buf[0..=3])?;
        let major_revision_number = buf[4].deserialize_word();
        let minor_revision_number = buf[5].deserialize_word();
        let build_revision_number = buf[6].deserialize_word();
        let local_modifications = buf[7].deserialize_bit(0);
        let mixed_revisions = buf[7].deserialize_bit(1);
        let no_svn = buf[7].deserialize_bit(7);
        let location = buf[7].deserialize_field(4, 5).try_into()?;
        let svn_id = buf[8..=11].deserialize_word();

        Ok(Self {
            header,
            major_revision_number,
            minor_revision_number,
            build_revision_number,
            local_modifications,
            mixed_revisions,
            no_svn,
            location,
            svn_id,
        })

    }

}

impl GetFuncInfoRequest {
pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<GetFuncInfoResponse, DeviceError> {
    let f = | h: Header | h.cts;

    const REQUEST_BUF_LEN: usize = 1;
    const RESPONSE_BUF_LEN: usize = 12;
    const STATUS_HEADER_LEN: usize = 4;

    let response = self.polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(spi, f)?;

    Ok(response)
}}

impl<SPI: SpiDevice> Transmit<SPI, GetFuncInfoResponse> for GetFuncInfoRequest {}

impl Command for GetFuncInfoRequest {
    fn opcode(&self) -> u8 {
        0x12
    }
}