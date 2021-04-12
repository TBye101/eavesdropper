use abi_stable::{StableAbi};

///Represents a version number that is FFI friendly via [abi_stable](https://github.com/rodrimati1992/abi_stable_crates/).
#[repr(C)]
#[derive(Clone, StableAbi, Eq, PartialEq, Ord, PartialOrd)]
pub struct RVersion {
    /// The major version, to be incremented on incompatible changes.
    pub major: u64,
    /// The minor version, to be incremented when functionality is added in a
    /// backwards-compatible manner.
    pub minor: u64,
    /// The patch version, to be incremented when backwards-compatible bug
    /// fixes are made.
    pub patch: u64
}

impl std::fmt::Display for RVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.major, self.minor, self.patch)
    }
}

impl RVersion {

    pub fn new(major: u64, minor: u64, patch: u64) -> RVersion {
        RVersion { major, minor, patch }
    }

    ///Returns true if the two versions are compatible in a semantic versioning sense, AKA their major versions are the same.
    pub fn is_compatible(&self, other_version: &RVersion) -> bool {
        self.major == other_version.major
    }
}