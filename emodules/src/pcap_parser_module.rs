use abi_stable::{rvec, std_types::{RString, RVec}};
use eframework::{RVersion::RVersion, analysis_framework::{AnalysisModule, ModuleInfo}};

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

impl Drop for PCapParserModule {
    fn drop(&mut self) {
        println!("[{}] Destroyed instance!", 1);
    }
}