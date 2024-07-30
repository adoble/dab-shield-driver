use si468x_pac::DeviceError;

use embedded_hal::digital::ErrorKind;

#[derive(Copy, Clone, Debug)]
pub enum DabShieldError {
    Spi(DeviceError),
    // Add other errors for your driver here.
    Pin(ErrorKind),
}
