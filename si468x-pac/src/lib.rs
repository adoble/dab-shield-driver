#![cfg_attr(not(test), no_std)]

/// Re-exports
pub use crate::types::*;
pub use crate::error::DeviceError;

pub mod header;

pub mod dab_get_audio_info;
pub mod dab_get_service_info;
pub mod get_time;
pub mod read_offset;
pub mod boot;
pub mod dab_tune_freq;
pub mod stop_digital_service;
pub mod host_load;
pub mod dab_get_ensemble_info;
pub mod dab_get_event_status;
pub mod flash_load;
pub mod get_sys_state;
pub mod get_part_info;
pub mod dab_digrad_status;
pub mod get_func_info;
pub mod dab_get_subchan_info;
pub mod load_init;
pub mod dab_set_freq_list;
pub mod start_digital_service;
pub mod power_up;
pub mod set_property;
pub mod dab_get_digital_service_list;

pub mod error;
pub mod types;
pub mod deserialize;
pub mod serialize;
pub mod request;
pub mod response;
pub mod bits;
pub mod command;
pub mod transmit;

/// Providers
pub mod host_image_data_loader;pub fn create_request<R, F>(f: F) -> R
    where
        R: Default,
        F: Fn(&mut R),
{
    let mut request = R::default();
    f(&mut request);
    request
}

/// Entry points
pub fn dab_get_audio_info<F>(f:F) -> dab_get_audio_info::DabGetAudioInfoRequest
where
    F: Fn(&mut dab_get_audio_info::DabGetAudioInfoRequest),
{
    create_request(f)
}

pub fn dab_get_service_info<F>(f:F) -> dab_get_service_info::DabGetServiceInfoRequest
where
    F: Fn(&mut dab_get_service_info::DabGetServiceInfoRequest),
{
    create_request(f)
}

pub fn get_time<F>(f:F) -> get_time::GetTimeRequest
where
    F: Fn(&mut get_time::GetTimeRequest),
{
    create_request(f)
}

pub fn read_offset<F>(f:F) -> read_offset::ReadOffsetRequest
where
    F: Fn(&mut read_offset::ReadOffsetRequest),
{
    create_request(f)
}

pub fn boot<F>(f:F) -> boot::BootRequest
where
    F: Fn(&mut boot::BootRequest),
{
    create_request(f)
}

pub fn dab_tune_freq<F>(f:F) -> dab_tune_freq::DabTuneFreqRequest
where
    F: Fn(&mut dab_tune_freq::DabTuneFreqRequest),
{
    create_request(f)
}

pub fn stop_digital_service<F>(f:F) -> stop_digital_service::StopDigitalServiceRequest
where
    F: Fn(&mut stop_digital_service::StopDigitalServiceRequest),
{
    create_request(f)
}

pub fn host_load<F>(f:F) -> host_load::HostLoadRequest
where
    F: Fn(&mut host_load::HostLoadRequest),
{
    create_request(f)
}

pub fn dab_get_ensemble_info<F>(f:F) -> dab_get_ensemble_info::DabGetEnsembleInfoRequest
where
    F: Fn(&mut dab_get_ensemble_info::DabGetEnsembleInfoRequest),
{
    create_request(f)
}

pub fn dab_get_event_status<F>(f:F) -> dab_get_event_status::DabGetEventStatusRequest
where
    F: Fn(&mut dab_get_event_status::DabGetEventStatusRequest),
{
    create_request(f)
}

pub fn flash_load<F>(f:F) -> flash_load::FlashLoadRequest
where
    F: Fn(&mut flash_load::FlashLoadRequest),
{
    create_request(f)
}

pub fn get_sys_state<F>(f:F) -> get_sys_state::GetSysStateRequest
where
    F: Fn(&mut get_sys_state::GetSysStateRequest),
{
    create_request(f)
}

pub fn get_part_info<F>(f:F) -> get_part_info::GetPartInfoRequest
where
    F: Fn(&mut get_part_info::GetPartInfoRequest),
{
    create_request(f)
}

pub fn dab_digrad_status<F>(f:F) -> dab_digrad_status::DabDigradStatusRequest
where
    F: Fn(&mut dab_digrad_status::DabDigradStatusRequest),
{
    create_request(f)
}

pub fn get_func_info<F>(f:F) -> get_func_info::GetFuncInfoRequest
where
    F: Fn(&mut get_func_info::GetFuncInfoRequest),
{
    create_request(f)
}

pub fn dab_get_subchan_info<F>(f:F) -> dab_get_subchan_info::DabGetSubchanInfoRequest
where
    F: Fn(&mut dab_get_subchan_info::DabGetSubchanInfoRequest),
{
    create_request(f)
}

pub fn load_init<F>(f:F) -> load_init::LoadInitRequest
where
    F: Fn(&mut load_init::LoadInitRequest),
{
    create_request(f)
}

pub fn dab_set_freq_list<F>(f:F) -> dab_set_freq_list::DabSetFreqListRequest
where
    F: Fn(&mut dab_set_freq_list::DabSetFreqListRequest),
{
    create_request(f)
}

pub fn start_digital_service<F>(f:F) -> start_digital_service::StartDigitalServiceRequest
where
    F: Fn(&mut start_digital_service::StartDigitalServiceRequest),
{
    create_request(f)
}

pub fn power_up<F>(f:F) -> power_up::PowerUpRequest
where
    F: Fn(&mut power_up::PowerUpRequest),
{
    create_request(f)
}

pub fn set_property<F>(f:F) -> set_property::SetPropertyRequest
where
    F: Fn(&mut set_property::SetPropertyRequest),
{
    create_request(f)
}

pub fn dab_get_digital_service_list<F>(f:F) -> dab_get_digital_service_list::DabGetDigitalServiceListRequest
where
    F: Fn(&mut dab_get_digital_service_list::DabGetDigitalServiceListRequest),
{
    create_request(f)
}