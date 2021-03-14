/// # Design
/// * For each module create a new table
///     * moduleName_Guid
/// * Give that table to the module
/// * Have a module list with module orderings/dependencies
/// * Conditional re-running of modules with new information somehow?
/// https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
 
use semver::VersionReq;
use semver::Version;
use static_assertions::*;

///Used to specify that a module requires another module to be run before it can be run.
#[derive(Eq, PartialEq)]
pub struct Dependency {
    pub name: &'static str,
    pub version_requirement: VersionReq
}

///Holds information about a module.
#[repr(C)]
pub struct ModuleInfo {
    ///The name of the module.
    pub name: &'static str,

    ///The version of the module
    pub version: Version,

    ///A list of the modules this module depends on.
    /// This module will be executed after the modules it depends on.
    pub dependencies: Vec<Dependency>
}

///Specifies a common protocol for analyzing our available data.
pub trait AnalysisModule {
    ///Returns information about the module.
    fn get_info(&self) -> ModuleInfo;

    ///Called when the framework is ready for the module to perform its analysis.
    ///table_names: The names of the tables that have already been created.
    ///Should return the names of the tables that have been created by this module. 
    fn analyze(&self, table_names: &Vec<String>, pcap_input_directory: &String) -> Vec<String>; 
}
assert_obj_safe!(AnalysisModule);