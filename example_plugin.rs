use dotenv::dotenv;
use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, rvec, sabi_extern_fn, sabi_trait::prelude::TU_Opaque, std_types::{RString, RVec}};
use eframework::{analysis_framework::{AnalysisModule, AnalysisModuleBox, AnalysisModule_TO, Dependency, ModuleInfo, Plugin, Plugin_Ref}, rversion::RVersion, rversion_req::RVersionReq};
use diesel::{Connection, RunQueryDsl, pg::PgConnection};

embed_migrations!();//Embed our Diesel migrations into this crate so we can run them upon beginning analysis later.

#[export_root_module]
pub fn get_library() -> Plugin_Ref {
    Plugin {
        get_analyzer,
    }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn get_analyzer() -> AnalysisModuleBox {
    AnalysisModule_TO::from_value(ExampleModule {}, TU_Opaque)
}

pub struct ExampleModule { }

impl AnalysisModule for ExampleModule {
    fn get_info(&self) -> eframework::analysis_framework::ModuleInfo {
        return ModuleInfo {
            name: RString::from("ExamplePluginName"),
            version: RVersion::new(0, 1, 0),
            dependencies: rvec![                
                Dependency 
                { 
                    name: RString::from("PCapParser"), //Base plugin for most if not all plugins.
                    version_requirement: RVersionReq { 
                        minimum_version: RVersion::new(0, 1, 0), 
                        maximum_version: RVersion::new(1, 0, 0) 
                    }
                } 
            ]
        }
    }

    fn analyze(&self, pcap_input_directory: &RString, connection_string: &RString) {
        println!("Starting the example plugin!");
    
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection: PgConnection = PgConnection::establish(&connection_string)
        .expect(&format!("Error connecting to {}", database_url));

        //Run our embedded Diesel migrations
        let migration_result = embedded_migrations::run(&connection);

        if migration_result.is_err() {
            println!("An error occurred while running migrations: {}", migration_result.err().unwrap());
            return;
        }

        //Do some packet analysis... 
    }
}