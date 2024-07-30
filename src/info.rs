pub use si468x_pac::types::ImageBuildLocation;

/// Basic information about the device
pub struct PartInfo {
    pub chip_revision: u8,
    pub rom_id: u8,
    pub part_number: u16,
}

/// Function revision number for currently loaded firmware (FMHD, AM etc.) as opposed
/// to PartInfo that provides the revision number for the combo firmware.
pub struct FunctionalInfo {
    /// First part of 1.2.3
    pub major_revision_number: u8,
    /// Second part of 1.2.3
    pub minor_revision_number: u8,
    /// Third part of 1.2.3
    pub build_revision_number: u8,
    /// If set the build was created with no SVN info
    pub no_svn: bool,
    /// The location from which the image was built
    pub location: ImageBuildLocation,
    /// If set, the image was built from mixed revisions
    pub mixed_revisions: bool,
    /// f set, the image has local modifications
    pub local_modifications: bool,
    /// SVN ID for which the image was built
    pub svn_id: u32,
}
