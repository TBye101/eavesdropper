use abi_stable::{StableAbi, std_types::ROption};

use crate::rversion::RVersion;

//, Eq, Ord
#[repr(C)]
#[derive(Clone, StableAbi, Eq, PartialEq)]
pub struct RVersionReq {

    ///The minimum version that is required (inclusive).
    pub minimum_version: RVersion,

    ///The maximum version that is allowed (inclusive).
    ///If is None, then there is no maximum version allowed.
    pub maximum_version: ROption<RVersion>
}

impl RVersionReq {
    ///Returns true if the version is within our allowed version range.
    pub fn compatible_with(&self, version: &RVersion) -> bool {
        if self.maximum_version.is_some() {
            if self.minimum_version.eq(version) || self.maximum_version.clone().unwrap().eq(version) {
                return true;
            }

            version.cmp(&self.minimum_version) == std::cmp::Ordering::Greater && version.cmp(&self.maximum_version.clone().unwrap()) == std::cmp::Ordering::Less
        }
        else {
            version.cmp(&self.minimum_version) == std::cmp::Ordering::Greater || version.eq(&self.minimum_version)
        }
    }
}

impl std::fmt::Display for RVersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.maximum_version.is_some() {
            write!(f, "[min: {}, max: {}]", self.minimum_version, self.maximum_version.clone().unwrap())
        }
        else {
            write!(f, "[min: {}]", self.minimum_version)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rversion::RVersion;
    use abi_stable::std_types::*;

    ///Verifies that when the minimum required version is x.y.z, then x.y.z is deemed compatible.
    #[test]
    fn test_minimum_compatibility() {
        let version = RVersion { major: 0, minor: 1, patch: 0 };
        let min = RVersion { major: 0, minor: 1, patch: 0 };
        let max = RVersion { major: 2, minor: 1, patch: 0 };
        let required_version = crate::rversion_req::RVersionReq { minimum_version: min, maximum_version: RSome(max) };
        assert!(required_version.compatible_with(&version));
    }

    ///Verifies that when the version being compatibility checked is between the minimum and the maximum required, then the version is deemed compatible.
    #[test]
    fn test_inbetween_compatibility() {
        let version = RVersion { major: 1, minor: 6, patch: 3 };
        let min = RVersion { major: 0, minor: 1, patch: 0 };
        let max = RVersion { major: 2, minor: 1, patch: 0 };
        let required_version = crate::rversion_req::RVersionReq { minimum_version: min, maximum_version: RSome(max) };
        assert!(required_version.compatible_with(&version));
    }

    ///Verifies that when x.y.z is the maximum compatible version, then a version x.y.z is deemed compatible.
    #[test]
    fn test_maximum_compatibility() {
        let version = RVersion { major: 2, minor: 1, patch: 0 };
        let min = RVersion { major: 0, minor: 1, patch: 0 };
        let max = RVersion { major: 2, minor: 1, patch: 0 };
        let required_version = crate::rversion_req::RVersionReq { minimum_version: min, maximum_version: RSome(max) };
        assert!(required_version.compatible_with(&version));
    }

    ///Verifies that when there is no upper bound on maximum compatible version, then any version greater than the minimum required version is compatible.
    #[test]
    fn test_unbounded_compatibility() {
        let version = RVersion { major: 3, minor: 6, patch: 3 };
        let min = RVersion { major: 0, minor: 1, patch: 0 };
        let required_version = crate::rversion_req::RVersionReq { minimum_version: min, maximum_version: RNone };
        assert!(required_version.compatible_with(&version));
    }

    ///Verifies that when the version being compatibility checked is less than the minimum compatible version, then they are deemed not compatible.
    #[test]
    fn test_less_than_minimum() {
        let version = RVersion { major: 0, minor: 6, patch: 3 };
        let min = RVersion { major: 0, minor: 7, patch: 0 };
        let max = RVersion { major: 2, minor: 1, patch: 0 };
        let required_version = crate::rversion_req::RVersionReq { minimum_version: min, maximum_version: RSome(max) };
        assert!(!required_version.compatible_with(&version));
    }

    ///Verifies that when the version being compatibility checked is greater than the maximum compatible version, then they are deemed not compatible.
    #[test]
    fn test_greater_than_maximum() {
        let version = RVersion { major: 2, minor: 2, patch: 3 };
        let min = RVersion { major: 0, minor: 1, patch: 0 };
        let max = RVersion { major: 2, minor: 1, patch: 0 };
        let required_version = crate::rversion_req::RVersionReq { minimum_version: min, maximum_version: RSome(max) };
        assert!(!required_version.compatible_with(&version));
    }
}