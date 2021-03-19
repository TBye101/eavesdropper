use std::path::Path;

/// # Design
/// * For each module create a new table
///     * moduleName_Guid
/// * Give that table to the module
/// * Have a module list with module orderings/dependencies
/// * Conditional re-running of modules with new information somehow?
/// https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
 
use semver::VersionReq;
use static_assertions::*;
use abi_stable::{DynTrait, StableAbi, library::{LibraryError, RootModule}, package_version_strings, sabi_trait, sabi_types::VersionStrings, std_types::{RBox, RStr, RString, RVec}};

use crate::{RVersion::RVersion, RVersionReq::RVersionReq};

///Used to specify that a module requires another module to be run before it can be run.
#[repr(C)]
#[derive(Eq, PartialEq, StableAbi)]
pub struct Dependency {
    pub name: RString,
    pub version_requirement: RVersionReq
}

///Holds information about a module.
#[repr(C)]
#[derive(StableAbi)]
pub struct ModuleInfo {
    ///The name of the module.
    pub name: RString,

    ///The version of the module
    pub version: RVersion,

    ///A list of the modules this module depends on.
    /// This module will be executed after the modules it depends on.
    pub dependencies: RVec<Dependency>
}

///Specifies a common protocol for analyzing our available data.
#[sabi_trait] //Create an ffi-safe trait object from this trait definition
pub trait AnalysisModule {
    ///Returns information about the module.
    fn get_info(&self) -> ModuleInfo;

    ///Called when the framework is ready for the module to perform its analysis.
    ///table_names: The names of the tables that have already been created.
    ///Should return the names of the tables that have been created by this module. 
    #[sabi(last_prefix_field)]//This attribute will stay here until we bump the "major" version of the library, in which case we will then move it to the last method at the time of bumping.
    fn analyze(&self, table_names: &RVec<RString>, pcap_input_directory: &RString) -> RVec<RString>; 
}
//Create an type alias for the automatically generated trait object for the AnalysisModule trait. https://docs.rs/abi_stable/0.9.3/abi_stable/docs/sabi_trait_attribute/index.html#trait_trait
pub type AnalysisModuleBox = AnalysisModule_TO<'static, RBox<()>>;
assert_obj_safe!(AnalysisModule);

///Used for the plugin system in order to pass through FFI boundaries using the [abi_stable](https://github.com/rodrimati1992/abi_stable_crates/) crate.
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref="Plugin_Ref")))]
#[sabi(missing_field(panic))]
pub struct Plugin {
    #[sabi(last_prefix_field)]//Stays here until we bump the major version number. Then it moves to the last field of the struct.
    pub get_analyzer: extern "C" fn() -> AnalysisModuleBox,

    // #[sabi(last_prefix_field)] //Stays here until we bump the major version number. Then it moves to the last field of the struct.
    // pub new_boxed_plugin: extern "C" fn() -> BoxedPluginInterface<'static>
}

///Some versioning information for determining which version of a plugin will compile with which version of the plugin system.
impl RootModule for Plugin_Ref {
    abi_stable::declare_root_module_statics!{Plugin_Ref}

    const BASE_NAME: &'static str = "Plugin";
    const NAME: &'static str = "Plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

///This is needed because it somehow describes our ffi traits...?
#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Sync, Send, Debug, Display))]
pub struct PluginInterface;
pub type BoxedPluginInterface<'borr> = DynTrait<'borr, RBox<()>, PluginInterface>;

/// This loads the root from the library in the `directory` folder.
pub fn load_plugin_from_directory(directory: &Path) -> Result<Box<dyn AnalysisModule>, String> {
    let test: Result<Plugin_Ref, LibraryError> = Plugin_Ref::load_from_file(directory);
    match test {
        Err(e) => Err(format!("Failed to load plugin: {} | Error: {}", directory.to_str().unwrap(), e)),
        Ok(plugin) => {
            let analysis_module_boxed = plugin.get_analyzer()();//Mysteriously turns into AnalysisModuleBox, which can be boxed up and magically treated like an instance of the AnalysisModule trait? Some Stable_Abi crate magic happening here...
            return Ok(Box::new(analysis_module_boxed));
        }
    }
}