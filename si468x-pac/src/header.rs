/// Common structure used in the driver
///
/// Generated with version 0.2.1 of ddgen

use crate::deserialize::Deserialize;
use crate::error::DeviceError;
#[allow(unused_imports)]
use crate::request::{RequestArray, RequestBit, RequestField, RequestWord};
#[allow(unused_imports)]
use crate::response::{ResponseArray, ResponseBit, ResponseField, ResponseWord};
use crate::serialize::Serialize;

use crate::types::*;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Header {
    /// Digital radio link change interrupt indicator.
    pub dacqint: bool,
    /// An enabled data component of one of the digital services requires attention.
    pub dsrvint: bool,
    /// Digital radio event change interrupt indicator.
    pub devntint: bool,
    pub arb_error: bool,
    pub error_nr: bool,
    pub repo_fatal_error: bool,
    /// Seek / tune complete
    pub stcint: bool,
    pub err_cmd: bool,
    pub cts: bool,
    pub pup_state: PowerUpState,
    /// The DSP has encountered a frame overrun.
    pub dsp_err: bool,
    pub cmdo_fatal_error: bool,
}

impl Serialize for Header {
    fn serialize<const N: usize>(&self) -> (usize, [u8; N], impl Iterator<Item=u8>) {
        let mut data = [0u8; N];
        #[allow(unused_variables)]
        let provider = core::iter::empty::<u8>();

        data[0].serialize_bit(self.stcint, 0);
        data[0].serialize_bit(self.dsrvint, 4);
        data[0].serialize_bit(self.dacqint, 5);
        data[0].serialize_bit(self.err_cmd, 6);
        data[0].serialize_bit(self.cts, 7);
        data[1].serialize_bit(self.devntint, 5);
        data[3].serialize_bit(self.error_nr, 0);
        data[3].serialize_bit(self.arb_error, 1);
        data[3].serialize_bit(self.cmdo_fatal_error, 2);
        data[3].serialize_bit(self.repo_fatal_error, 3);
        data[3].serialize_bit(self.dsp_err, 4);
        data[3].serialize_field(self.pup_state as u8, 6, 7);

        (4, data, provider)
    }

}

impl Deserialize<Self> for Header {

    fn deserialize(buf: &[u8]) -> Result<Header, DeviceError> {

        let stcint = buf[0].deserialize_bit(0);
        let dsrvint = buf[0].deserialize_bit(4);
        let dacqint = buf[0].deserialize_bit(5);
        let err_cmd = buf[0].deserialize_bit(6);
        let cts = buf[0].deserialize_bit(7);
        let devntint = buf[1].deserialize_bit(5);
        let error_nr = buf[3].deserialize_bit(0);
        let arb_error = buf[3].deserialize_bit(1);
        let cmdo_fatal_error = buf[3].deserialize_bit(2);
        let repo_fatal_error = buf[3].deserialize_bit(3);
        let dsp_err = buf[3].deserialize_bit(4);
        let pup_state = buf[3].deserialize_field(6, 7).try_into()?;

        Ok(Self {
            stcint,
            dsrvint,
            dacqint,
            err_cmd,
            cts,
            devntint,
            error_nr,
            arb_error,
            cmdo_fatal_error,
            repo_fatal_error,
            dsp_err,
            pup_state,
        })

    }

}