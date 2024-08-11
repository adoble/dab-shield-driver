use si468x_pac::{
    header::{self, HasHeader},
    types::Response,
    DeviceError,
};

use embedded_hal::{digital::ErrorKind, spi::SpiDevice};

#[derive(Copy, Clone, Debug)]
pub enum DabShieldError {
    Spi(DeviceError),
    // Add other errors for your driver here.
    Pin(ErrorKind),
    Command(si468x_pac::types::CommandError),
}

impl DabShieldError {
    pub fn handle_command_error<'a, SPI: SpiDevice, R: Response>(
        spi: &mut SPI,
        response: R,
    ) -> Result<R, DabShieldError> {
        if response.err_cmd() {
            // Request a response including the error reason

            //todo!("Check the logic against the original driver which uses a spi transfer");

            let response = si468x_pac::rd_reply(|req| {})
                .send(spi)
                .map_err(|err| DabShieldError::Spi(DeviceError::Transmit))?;

            Err(DabShieldError::Command(response.error_code))
        } else {
            Ok(response)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};

    use si468x_pac::header::Header;
    use si468x_pac::power_up::PowerUpResponse;

    #[test]
    fn test_handle_command_error() {
        let spi_expectations = [
            SpiTransaction::transaction_start(),
            SpiTransaction::write(0x00),
            SpiTransaction::write_vec(vec![0; 0]), // Empty array
            SpiTransaction::read_vec(vec![0x00u8, 0x00, 0x00, 0x00, 0x10]),
            SpiTransaction::transaction_end(),
        ];

        let mut spi = SpiMock::new(&spi_expectations);

        let response = si468x_pac::power_up::PowerUpResponse {
            header: Header {
                err_cmd: true,
                ..Default::default()
            },
        };

        if let Err(DabShieldError::Command(command_error)) =
            DabShieldError::handle_command_error(&mut spi, response)
        {
            assert_eq!(
                command_error,
                si468x_pac::types::CommandError::CommandNotFound
            );
        } else {
            assert!(false, "Error not returned!")
        }

        spi.done();
    }
}
