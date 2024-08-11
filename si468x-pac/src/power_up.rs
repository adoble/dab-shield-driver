#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

/// Command PowerUp

/// The POWER_UP initiates the boot process to move the device from power down to power up mode.

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

use crate::header::{HasHeader, Header};

#[derive(Debug, PartialEq)]
pub struct PowerUpRequest {
    /// The bootloader will toggle a host interrupt line when CTS is available.
    pub cts_is_enabled: bool,
    /// Range 1-15
    pub tr_size: u8,
    pub clock_mode: ClockMode,
    /// XTAL IBIAS current at startup. Range 0-127
    pub ibais: u8,
    /// XTAL frequency in Hz
    pub xtal_frequency: u32,
    /// Range 0-63
    pub ctun: u8,
    pub arg9: u8,
    pub arg10: u8,
    pub arg11: u8,
    pub arg12: u8,
    /// TAL IBIAS current at runtime, after the XTAL oscillator has stabilized
    pub ibias_run: u8,
    pub arg14: u8,
    pub arg15: u8,
}

#[allow(clippy::derivable_impls)]
impl Default for PowerUpRequest {
    fn default() -> Self {
        Self {
            cts_is_enabled: Default::default(),
            tr_size: Default::default(),
            clock_mode: Default::default(),
            ibais: Default::default(),
            xtal_frequency: Default::default(),
            ctun: Default::default(),
            arg9: 0b00010000,
            arg10: 0x00,
            arg11: 0x00,
            arg12: 0x00,
            ibias_run: Default::default(),
            arg14: 0x00,
            arg15: 0x00,
        }
    }
}

impl Serialize for PowerUpRequest {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item = u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0].serialize_bit(self.cts_is_enabled, 7);
        data[1].serialize_field(self.tr_size as u8, 0, 3);
        data[1].serialize_field(self.clock_mode as u8, 4, 5);
        data[2].serialize_field(self.ibais as u8, 0, 6);
        data[3..=6].serialize_word(self.xtal_frequency);
        data[7].serialize_field(self.ctun as u8, 0, 5);
        data[8] = 0b00010000;
        data[9] = 0x00;
        data[10] = 0x00;
        data[11] = 0x00;
        data[12].serialize_field(self.ibias_run as u8, 0, 6);
        data[13] = 0x00;
        data[14] = 0x00;

        (15, data, provider)
    }
}

#[derive(Debug, PartialEq)]
pub struct PowerUpResponse {
    pub header: Header,
}

impl Deserialize<Self> for PowerUpResponse {
    fn deserialize(buf: &[u8]) -> Result<PowerUpResponse, DeviceError> {
        let header = Header::deserialize(&buf[0..=3])?;

        Ok(Self { header })
    }
}

impl PowerUpRequest {
    pub fn send<SPI: SpiDevice>(&self, spi: &mut SPI) -> Result<PowerUpResponse, DeviceError> {
        let f = |h: Header| h.cts;

        const REQUEST_BUF_LEN: usize = 15;
        const RESPONSE_BUF_LEN: usize = 4;
        const STATUS_HEADER_LEN: usize = 4;

        let response = self
            .polled_transmit::<REQUEST_BUF_LEN, RESPONSE_BUF_LEN, Header, STATUS_HEADER_LEN>(
                spi, f,
            )?;

        Ok(response)
    }
}

impl<SPI: SpiDevice> Transmit<SPI, PowerUpResponse> for PowerUpRequest {}

impl Command for PowerUpRequest {
    fn opcode(&self) -> u8 {
        0x1
    }
}

impl Response for PowerUpResponse {}

impl HasHeader for PowerUpResponse {
    fn cts(&self) -> bool {
        self.header.cts
    }

    fn err_cmd(&self) -> bool {
        self.header.err_cmd
    }
}
