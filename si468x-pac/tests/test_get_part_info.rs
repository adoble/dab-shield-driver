#![allow(unused_imports)]
use si468x::response::{ResponseBit, ResponseWord};
use si468x::transmit::Transmit;
use si468x::DeviceError;
use si468x::{command::Command, deserialize::Deserialize, serialize::Serialize};

//use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
//use embedded_hal::spi::{Operation, SpiDevice};
use embedded_hal::spi::SpiDevice;

use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};

use si468x::get_part_info::{GetPartInfoRequest, GetPartInfoResponse};
use si468x::header::Header;

#[test]
// Single poll
fn test_polled_request_1() {
    let request = GetPartInfoRequest { arg1: 0 };

    let expected_header = Header {
        dsrvint: false,
        pup_state: si468x::PowerUpState::AppRunning,
        devntint: false,
        dsp_err: false,
        error_nr: false,
        repo_fatal_error: false,
        stcint: false,
        dacqint: false,
        err_cmd: false,
        cts: true,
        cmdo_fatal_error: false,
        arb_error: false,
    };

    let expected_response = GetPartInfoResponse {
        header: expected_header,
        chip_mask_revision: 12,
        rom_id: 5,
        part_number: 4711,
        _padding: [0_u8; 13],
    };

    //let (size, data, _) = request.serialize::<2>();

    let expected_header_bytes = vec![0b1000_0000, 0b0, 0b0, 0b1100_0000];
    let expected_payload_bytes = vec![
        0b0000_1100, //CHIPPREV chip_mask_revision
        0b0000_0101, //ROMID
        0b0,
        0b0,
        0x67, // LSB Part number 4711
        0x12, // MSB Part number 4711
        0b0,
        0b0,
        0b0,
        0b0,
        0b0,
        0b0,
        0b0,
        0b0,
        0b0,
        0b0,
        0b0,
        0b0,
        0b0,
    ];

    let spi_expectations = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write(0x08),
        SpiTransaction::write(0x00),
        SpiTransaction::read_vec(expected_header_bytes),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::read_vec(expected_payload_bytes),
        SpiTransaction::transaction_end(),
    ];

    let mut spi = SpiMock::new(&spi_expectations);

    // let mut cs = PinMock::new(&cs_expectations);

    let response: GetPartInfoResponse = request.send(&mut spi).unwrap();

    assert_eq!(response, expected_response);

    spi.done();
}
