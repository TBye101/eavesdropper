use abi_stable::{StableAbi};

use crate::RVersion::RVersion;

//, Eq, Ord
#[repr(C)]
#[derive(Clone, StableAbi, Eq, PartialEq)]
pub struct RVersionReq {

    ///The minimum version that is required (inclusive).
    minimumVersion: RVersion,

    ///The maximum version that is allowed (inclusive)
    maximumVersion: RVersion
}

impl RVersionReq {
    ///Returns true if the version is within our allowed version range.
    pub fn compatible_with(&self, version: &RVersion) -> bool {
        if self.minimumVersion.eq(version) || self.maximumVersion.eq(version) {
            return true;
        }

        return version.cmp(&self.minimumVersion) == std::cmp::Ordering::Greater && version.cmp(&self.maximumVersion) == std::cmp::Ordering::Less;
    }
}

impl std::fmt::Display for RVersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} < Required < {}", self.minimumVersion, self.maximumVersion)
    }
}