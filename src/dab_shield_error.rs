use si468x_pac::DeviceError;

use embedded_hal::{
    digital::ErrorKind,
    spi::{Operation, SpiDevice},
};

#[derive(Copy, Clone, Debug)]
pub enum DabShieldError {
    Spi(DeviceError),
    // Add other errors for your driver here.
    Pin(ErrorKind),
    Command(si468x_pac::types::CommandError),
}

pub fn handle_command_error<SPI: SpiDevice>(
    spi: &mut SPI,
    header: si468x_pac::header::Header,
) -> Result<(), DabShieldError> {
    if header.err_cmd {
        // Request a response including the error reason

        //todo!("Check the logic against the original driver");

        // let mut response_buf = [0u8; 5];
        // spi.transaction(&mut [Operation::Read(&mut response_buf)])
        //     .map_err(|_| DabShieldError::Spi(DeviceError::Transmit))?;

        let response = si468x_pac::rd_reply(|req| {})
            .send(spi)
            .map_err(|err| DabShieldError::Spi(DeviceError::Transmit))?;

        Err(DabShieldError::Command(response.error_code))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
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

        let response_header = si468x_pac::header::Header {
            err_cmd: true,
            ..Default::default()
        };

        if let Err(DabShieldError::Command(command_error)) =
            handle_command_error(&mut spi, response_header)
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
