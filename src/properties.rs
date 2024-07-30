use si468x_pac::set_property::SetPropertyRequest;

#[allow(dead_code)]
#[derive(Default, Clone, Copy)]
pub enum FmProperties {
    #[default]
    RdReply = 0x00, // Returns the status byte and data for the last command sent to the device.
    PowerUp = 0x01,               // Power-up the device and set system settings.
    HostLoad = 0x04,              // Loads an image from HOST over command interface.
    FlashLoad = 0x05,             // Loads an image from external FLASH over secondary SPI bus.
    LoadInit = 0x06,              // Prepares the bootloader to receive a new image.
    Boot = 0x07,                  // Boots the image currently loaded in RAM.
    GetPartInfo = 0x08,           // Reports basic information about the device.
    GetSysState = 0x09,           // Reports system state information.
    GetPowerUpArgs = 0x0A, // Reports basic information about the device such as arguments used during POWER_UP.
    ReadOffset = 0x10,     // Reads a portion of response buffer from an offset.
    GetFuncInfo = 0x12,    // Returns the Function revision information of the device.
    SetProperty = 0x13,    // Sets the value of a property.
    GetProperty = 0x14,    // Retrieve the value of a property.
    FmTuneFreq = 0x30,     // Tunes the FM receiver to a frequency in 10 kHz steps.
    FmSeekStart = 0x31, // Initiates a seek for a channel that meets the validation criteria for FM.
    FmRsqStatus = 0x32, // Returns status information about the received signal quality.
    FmAcfStatus = 0x33, // Returns status information about automatically controlled features.
    FmRdsStatus = 0x34, // Queries the status of RDS decoder and Fifo.
    FmRdsBlockcount = 0x35, // Queries the block statistic info of RDS decoder.
    GetDigitalServiceList = 0x80, // Gets a service list of the ensemble.
    StartDigitalService = 0x81, // Starts an audio or data service.
    StopDigitalService = 0x82, // Stops an audio or data service.
    GetDigitalServiceData = 0x84, // Gets a block of data associated with one of the enabled data components of a digital services.
    HdDigradStatus = 0x92, // Returns status information about the digital radio and ensemble.
    HdGetEventStatus = 0x93, // Gets information about the various events related to the HD services.
    HdGetStationInfo = 0x94, // Retrieves information about the ensemble broadcaster.
    HdGetPsdDecode = 0x95,   // Retrieves PSD information.
    HdGetAlertMsg = 0x96,    // Retrieves the HD Alert message.
    HdPlayAlertTone = 0x97,  // Plays the HD Alert Tone.
    HdTestGetBerInfo = 0x98, // Reads the current BER information.
    HdSetEnabledPorts = 0x99, // Sets default ports retrieved after acquisition.
    HdGetEnabledPorts = 0x9A, // Gets default ports retrieved after acquisition.
    TestGetRssi = 0xE5,      // Returns the reported RSSI in 8.8 format.
}
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum DabProperties {
    /// Interrupt enable property
    IntCtlEnable = 0x0000,
    /// Interrupt repeat property
    IntCtlRepeat = 0x0001,
    /// Selects digital audio Master or Slave.
    DigitalIoOutputSelect = 0x0200,
    /// Sets output sample audio rate in units of 1Hz.
    DigitalIoOutputSampleRate = 0x0201,
    /// Configure digital output format.
    DigitalIoOutputFormat = 0x0202,
    /// Deviations from the standard framing mode
    DigitalIoOutputFormatOverrides1 = 0x0203,
    /// Deviations from the standard framing mode
    DigitalIoOutputFormatOverrides2 = 0x0204,
    /// Deviations from the standard framing mode
    DigitalIoOutputFormatOverrides3 = 0x0205,
    /// Deviations from the standard framing mode
    DigitalIoOutputFormatOverrides4 = 0x0206,
    /// Sets the audio analog volume.
    AudioAnalogVolume = 0x0300,
    /// AUDIO_MUTE property mutes/unmutes each audio output independently.
    AudioMute = 0x0301,
    /// AUDIO_OUTPUT_CONFIG is used to configure various settings of the audio output.
    AudioOutputConfig = 0x0302,
    /// Pin configuration property
    PinConfigEnable = 0x0800,
    /// Enables the wake tone feature.
    WakeToneEnable = 0x0900,
    /// Sets the wake tone duty cycle.
    WakeTonePeriod = 0x0901,
    /// Sets the wake tone frequency.
    WakeToneFreq = 0x0902,
    /// Sets the wake tone amplitude.
    WakeToneAmplitude = 0x0903,
    /// DAB/DMB Front End Varactor configuration slope
    DabTuneFeVarm = 0x1710,
    /// DAB/DMB Front End Varactor configuration intercept
    DabTuneFeVarb = 0x1711,
    /// Additional configuration options for the front end.
    DabTuneFeCfg = 0x1712,
    /// Configures the interrupt sources for digital services
    DigitalServiceIntSource = 0x8100,
    /// Sets the delay time (in milliseconds) to restart digital service when recovering from acquisition loss
    DigitalServiceRestartDelay = 0x8101,
    /// Configures interrupts related to digital receiver.
    DabDigradInterruptSource = 0xB000,
    /// Selects which XPAD data will forwarded to the host.
    DabXpadEnable = 0xB400,
    /// Set the time window threshold (in milliseconds) to mute audio
    DabCtrlDabMuteWinThreshold = 0xB502,
    /// Set the time window threshold (in milliseconds) to unmute audio
    DabCtrlDabUnmuteWinThreshold = 0xB503,
    /// Set the signal RSSI threshold to mute audio. RSSI below this threshold indicates that signal is lost. In this case, audio will be muted.
    DabCtrlDabMuteSiglossThreshold = 0xB504,
    /// Set the signal SNR threshold. The fic_quality based audio mute operation only engages when signal SNR is below this threshold.
    DabCtrlDabMuteSiglowThreshold = 0xB505,
    /// Sets up and enables the DAB BER test
    DabTestBerConfig = 0xE800,
}
