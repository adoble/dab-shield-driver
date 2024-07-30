use si468x_pac::flash_load::FlashLoadRequest;

#[test]
fn test_command() {
    let expected_req = FlashLoadRequest {
        flash_start_address: 0,
        ..Default::default()
    };

    let req = si468x_pac::flash_load(|_| {});

    assert_eq!(req, expected_req);
}

#[test]
fn test_modify() {
    let expected_req = FlashLoadRequest {
        flash_start_address: 4000,
        ..Default::default()
    };

    // let req = commands::flash_load().modify(|req| req.flash_start_address = 4000);
    let req = si468x_pac::flash_load(|req| req.flash_start_address = 4000);

    assert_eq!(req, expected_req);
}
