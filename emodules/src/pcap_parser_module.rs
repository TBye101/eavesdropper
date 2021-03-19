
use abi_stable::{DynTrait, ImplType, erased_types::TypeInfo, export_root_module, impl_get_type_info, prefix_type::PrefixTypeTrait, rvec, sabi_extern_fn, sabi_trait::prelude::TU_Opaque, std_types::{RString, RVec}};

use eframework::{RVersion::RVersion, analysis_framework::{AnalysisModule, AnalysisModuleBox, BoxedPluginInterface, ModuleInfo, Plugin, Plugin_Ref, AnalysisModule_TO}};

#[export_root_module]
pub fn get_library() -> Plugin_Ref {
    Plugin {
        get_analyzer,
        // new_boxed_plugin
    }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn get_analyzer() -> AnalysisModuleBox {
    AnalysisModule_TO::from_value(PCapParserModule {}, TU_Opaque)
}

// #[sabi_extern_fn]
// pub fn new_boxed_plugin() -> BoxedPluginInterface<'static> {
//     DynTrait::from_value()
// }

pub struct PCapParserModule { }

impl AnalysisModule for PCapParserModule {
    fn get_info(&self) -> eframework::analysis_framework::ModuleInfo {
        println!("We made it to the get_info()");
        return ModuleInfo {
            name: RString::from("PCapParser"),
            version: RVersion::new(0, 1, 0),
            dependencies: rvec![],
        }
    }

    fn analyze(&self, table_names: &RVec<RString>, pcap_input_directory: &RString) -> RVec<RString> {
        println!("Test hello world from the analysis function");
        return rvec!();
    }
}