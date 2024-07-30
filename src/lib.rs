#![cfg_attr(not(test), no_std)]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{Error, InputPin, OutputPin};
/// Driver for the SI468x digital radio.
/// This driver only works for the DAB part.
///  
use embedded_hal::spi::SpiDevice;

use dab_shield_error::DabShieldError;
use frequency_list::FrequencyList;
use properties::DabProperties;
use si468x_pac::ClockMode;
//use si468x_pac::boot::{BootRequest, BootResponse};
//use si468x_pac::error::DeviceError;
//use si468x_pac::flash_load::{FlashLoadRequest, FlashLoadResponse};
//use si468x_pac::types::*;
use types::FrequencyIndex;

use info::{FunctionalInfo, PartInfo};

//pub mod commands;
pub mod dab_shield_error;
pub mod frequency_list;
mod info;
mod properties;
mod types;

const DAB_FLASH_LOAD_ADDRESS: u32 = 0x6000;

pub struct DabShieldDriver<SPI, INT, RST, EN, DLY> {
    spi: SPI,
    interrupt_pin: INT,
    reset_pin: RST,
    power_enable: EN,
    delay: DLY,

    frequency_index: Option<FrequencyIndex>,
}

impl<SPI, INT, RST, EN, DLY> DabShieldDriver<SPI, INT, RST, EN, DLY>
where
    SPI: SpiDevice,
    INT: InputPin,
    RST: OutputPin,
    EN: OutputPin,
    DLY: DelayNs,
{
    pub fn new(spi: SPI, interrupt_pin: INT, reset_pin: RST, power_enable: EN, delay: DLY) -> Self {
        Self {
            spi,
            interrupt_pin,
            reset_pin,
            power_enable,
            delay,
            frequency_index: None,
        }
    }

    //TODO merge with init_dab
    pub fn init(&mut self, frequency_list: FrequencyList) -> Result<(), DabShieldError> {
        self.reset()?;
        todo!("Where is begin() that also resets the device");
        self.init_dab(&frequency_list)?;

        // TODO Do we need to do this operations here as they directly give a result.
        // Could also combine them
        // si468x_get_part_info();
        self.get_part_info()?;
        // si468x_get_func_info();
        self.get_functional_info()?;

        Ok(())
    }

    fn reset(&mut self) -> Result<(), DabShieldError> {
        self.power_enable
            .set_high()
            .map_err(|err| DabShieldError::Pin(err.kind()))?;

        self.delay.delay_ms(100);

        self.reset_pin
            .set_low()
            .map_err(|err| DabShieldError::Pin(err.kind()))?;
        self.delay.delay_ms(100);
        self.reset_pin
            .set_high()
            .map_err(|err| DabShieldError::Pin(err.kind()))?;
        self.delay.delay_ms(300); //Deviates from cpp driver as the spec want 300 ms

        // si468x_power_up();
        // spiBuf[0] = SI46XX_POWER_UP;
        // spiBuf[1] = 0x00;    ctsien = 0
        // spiBuf[2] = 0x17;   clk_mode = 1 = XtalMode, tr_size = 7
        // spiBuf[3] = 0x48;    ibias = 0x48
        // spiBuf[4] = 0x00;    xtal_freq  = 19200000 Hz
        // spiBuf[5] = 0xf8;    xtal_freq
        // spiBuf[6] = 0x24;    xtal_freq
        // spiBuf[7] = 0x01;    xtal_freq
        // spiBuf[8] = 0x1F;    ctun = 0x1f = 31
        // spiBuf[9] = 0x10;    arg9 = 0x10 (literal)
        // spiBuf[10] = 0x00;   arg10 = 0x00 (literal)
        // spiBuf[11] = 0x00;   arg11 = 0x00 (literal)
        // spiBuf[12] = 0x00;   arg12 = 0x00 (literal)
        // spiBuf[13] = 0x18;   ibias_run = 0x18
        // spiBuf[14] = 0x00;   arg14 = 0x00 (literal)
        // spiBuf[15] = 0x00;   arg15 = 0x00 (literal)

        // Power up
        si468x_pac::power_up(|req| {
            req.cts_is_enabled = false;
            req.clock_mode = ClockMode::XtalMode;
            req.tr_size = 7;
            req.ibais = 72;
            req.xtal_frequency = 19200000;
            req.ctun = 31;
            req.ibias_run = 24;
        })
        .send(&mut self.spi)
        .map_err(|err| DabShieldError::Spi(err))?;

        // si468x_load_init();
        si468x_pac::load_init(|_| {})
            .send(&mut self.spi)
            .map_err(|err| DabShieldError::Spi(err))?;

        // si468x_host_load();
        todo!();
        // si468x_load_init();
        todo!();
    }

    fn init_dab(&mut self, frequency_list: &FrequencyList) -> Result<(), DabShieldError> {
        // flash load

        // si468_pac::flash_load() ...
        si468x_pac::flash_load(|req| req.flash_start_address = DAB_FLASH_LOAD_ADDRESS)
            //.modify(|req| req.flash_start_address = DAB_FLASH_LOAD_ADDRESS)
            .send(&mut self.spi)
            .map_err(|e| DabShieldError::Spi(e))?;

        si468x_pac::boot(|_| {})
            .send(&mut self.spi)
            .map_err(|e| DabShieldError::Spi(e))?;

        // si468x_set_freq_list
        si468x_pac::dab_set_freq_list(|req| {
            req.frequencies[0..].copy_from_slice(frequency_list.frequencies());
            req.number_frequencies = frequency_list.number();
        })
        .send(&mut self.spi)
        .map_err(|e| DabShieldError::Spi(e))?;

        // Set up INTB
        // si468x_set_property(0x0000, 0x0010);
        self.set_property(DabProperties::IntCtlEnable, 0x0010)?;

        // si468x_set_property(0x1710, 0xF83E);
        self.set_property(DabProperties::DabTuneFeVarm, 0xF83Eu16 as i16)?;
        // si468x_set_property(0x1711, 0x01A4);
        self.set_property(DabProperties::DabTuneFeVarb, 0x01A4)?;
        // si468x_set_property(0x1712, 0x0001);
        self.set_property(DabProperties::DabTuneFeCfg, 0x0001)?;

        // si468x_set_property(0x8100, 0x0001);
        // Enable DSRVPCKTINT
        self.set_property(DabProperties::DigitalServiceIntSource, 0x0001)?;
        // si468x_set_property(0xb400, 0x0007);
        // Enable XPAD data
        self.set_property(DabProperties::DabXpadEnable, 0x0007)?;

        Ok(())
    }

    // From si468x_get_part_info();
    pub fn get_part_info(&mut self) -> Result<PartInfo, DabShieldError> {
        let response = si468x_pac::get_part_info(|_| {})
            .send(&mut self.spi)
            .map_err(|e| DabShieldError::Spi(e))?;

        Ok(PartInfo {
            chip_revision: response.chip_mask_revision,
            rom_id: response.rom_id,
            part_number: response.part_number,
        })
    }

    // From si468x_get_func_info();
    pub fn get_functional_info(&mut self) -> Result<FunctionalInfo, DabShieldError> {
        let response = si468x_pac::get_func_info(|_| {})
            .send(&mut self.spi)
            .map_err(|e| DabShieldError::Spi(e))?;

        Ok(FunctionalInfo {
            major_revision_number: response.major_revision_number,
            minor_revision_number: response.minor_revision_number,
            build_revision_number: response.build_revision_number,
            no_svn: response.no_svn,
            location: response.location,
            mixed_revisions: response.mixed_revisions,
            local_modifications: response.local_modifications,
            svn_id: response.svn_id,
        })
    }

    fn set_property(&mut self, id: DabProperties, value: i16) -> Result<(), DabShieldError> {
        si468x_pac::set_property(|req| {
            req.property_id = id as u16;
            req.data = value;
        })
        .send(&mut self.spi)
        .map_err(|e| DabShieldError::Spi(e))?;

        Ok(())
    }

    // pub fn read_foo(&mut self) -> Result<[u8; 2], DriverError<SPI::Error>> {
    //     let mut buf = [0; 2];

    //     // `transaction` asserts and deasserts CS for us. No need to do it manually!
    //     self.spi
    //         .transaction(&mut [Operation::Write(&[0x90]), Operation::Read(&mut buf)])
    //         .map_err(DriverError::Spi)?;

    //     Ok(buf)
    // }
}

#[derive(Copy, Clone, Debug)]
enum DriverError<SPI> {
    Spi(SPI),
    // Add other errors for your driver here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test_() {
        // TODO
    }
}
