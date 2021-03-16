use abi_stable::{StableAbi};

//, Eq, Ord
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

// impl Ord for RVersion {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         let major_comparison = self.major.cmp(other.major);
//         if major_comparison == std::cmp::Ordering::Equal {
//             return major_comparison;
//         }

//         let minor_comparison = self.minor.cmp(other.minor);
//         if minor_comparison == std::cmp::Ordering::Equal {
//             return minor_comparison;
//         }

//         return self.patch.cmp(other.patch);
//     }
// }

impl RVersion {

    pub fn new(major: u64, minor: u64, patch: u64) -> RVersion {
        RVersion { major, minor, patch }
    }

    ///Returns true if the two versions are compatible in a semantic versioning sense, AKA their major versions are the same.
    pub fn is_compatible(&self, otherVersion: &RVersion) -> bool {
        return self.major == otherVersion.major;
    }
}