use si468x_pac::boot::BootRequest;
use si468x_pac::dab_set_freq_list::DabSetFreqListRequest;
use si468x_pac::flash_load::FlashLoadRequest;
use si468x_pac::set_property::SetPropertyRequest;

pub fn create_request<R, F>(f: F) -> R
where
    R: Default,
    F: Fn(&mut R),
{
    let mut request = R::default();

    f(&mut request);
    request
}

pub fn flash_load<F>(f: F) -> FlashLoadRequest
where
    F: Fn(&mut FlashLoadRequest),
{
    create_request(f)
}

// pub fn flash_load<F>(f: F) -> FlashLoadRequest
// where
//     F: Fn(&mut FlashLoadRequest),
// {
//     let mut request = FlashLoadRequest {
//         ..Default::default()
//     };

//     f(&mut request);
//     request

// }

pub fn boot<F>(f: F) -> BootRequest
where
    F: Fn(&mut BootRequest),
{
    let mut request = BootRequest {
        ..Default::default()
    };

    f(&mut request);
    request
}

pub fn dab_set_freq_list<F>(f: F) -> DabSetFreqListRequest
where
    F: Fn(&mut DabSetFreqListRequest),
{
    let mut request = DabSetFreqListRequest {
        ..Default::default()
    };

    f(&mut request);
    request
}

pub fn set_property<F>(f: F) -> SetPropertyRequest
where
    F: Fn(&mut SetPropertyRequest),
{
    let mut request = SetPropertyRequest {
        ..Default::default()
    };

    f(&mut request);
    request
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_command() {
        // let commands = Commands::new();

        let req = flash_load(|req| req.flash_start_address = 0x600);

        let mut expected_req = FlashLoadRequest {
            ..Default::default()
        };
        expected_req.flash_start_address = 0x600;

        assert_eq!(req, expected_req);
    }
}
