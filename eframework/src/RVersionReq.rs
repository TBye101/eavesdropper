use abi_stable::{StableAbi};

use crate::RVersion::RVersion;

//, Eq, Ord
#[repr(C)]
#[derive(Clone, StableAbi, Eq, PartialEq)]
pub struct RVersionReq {

    ///The minimum version that is required (inclusive).
    minimum_version: RVersion,

    ///The maximum version that is allowed (inclusive)
    maximum_version: RVersion
}

impl RVersionReq {
    ///Returns true if the version is within our allowed version range.
    pub fn compatible_with(&self, version: &RVersion) -> bool {
        if self.minimum_version.eq(version) || self.maximum_version.eq(version) {
            return true;
        }

        return version.cmp(&self.minimum_version) == std::cmp::Ordering::Greater && version.cmp(&self.maximum_version) == std::cmp::Ordering::Less;
    }
}

impl std::fmt::Display for RVersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} < Required < {}", self.minimum_version, self.maximum_version)
    }
}