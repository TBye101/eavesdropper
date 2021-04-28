# Eavesdropper

[![dependency status](https://deps.rs/repo/github/tbye101/eavesdropper/status.svg)](https://deps.rs/repo/github/tbye101/eavesdropper)

A packet sniffing and analyzing tool with a plugin system for analyzers. Currently only a cli tool, but will evolve into a large scale automated analysis tool.

## Roadmap
You can find the official plugin roadmap [here](OfficialPluginRoadmap.md)

The framework roadmap can be found [here](FrameworkRoadmap.md)

## Plugins
Eavesdropper performs packet analysis via plugins.

### How to Make a Plugin
This example will show how to make a plugin as well as use [Diesel](https://diesel.rs/) to interact with the shared SQL database.

Example code without comments can be found [here](example_plugin.rs)
***

In your Cargo.toml file, this will have your library create a C assembly which is what is needed by the eavesdropper cli to load the plugin. The rust assembly is useful for if other plugins depend on the database tables created by your plugin, but don't want to redefine your Diesel table structs.
``` TOML
[lib]
crate-type = ["cdylib", "rlib"]
```

These use statements are required to be a plugin as well as to use the basics of Diesel.
``` Rust

use dotenv::dotenv;
use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, rvec, sabi_extern_fn, sabi_trait::prelude::TU_Opaque, std_types::{RString, RVec}};
use eframework::{analysis_framework::{AnalysisModule, AnalysisModuleBox, AnalysisModule_TO, Dependency, ModuleInfo, Plugin, Plugin_Ref}, rversion::RVersion, rversion_req::RVersionReq};
use diesel::{Connection, RunQueryDsl, pg::PgConnection};

```
This statement here has Diesel embed its ```/migrations``` directory directly into your plugin. Necessary to keep plugins portable and downloadable via binaries or from crates.io
``` Rust
embed_migrations!();//Embed our Diesel migrations into this crate so we can run them upon beginning analysis later.
```

This little snippet is what allows the Eavesdropper cli to load the plugin out of a compiled library.
The only thing that should be customized here would be changing the name of the struct to something that suits your plugin better.
Do remember to change the name in the rest of the example plugin.
``` Rust
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
```

Here we implement the AnalysisModule trait, which enables our plugin to be called in the analysis process.
The first function, ```get_info``` is what enables Eavesdropper to gather the name, version and dependency information about your plugin.
The second function, ```analyze``` is what is called when it is your plugins turn to perform its analysis.
``` Rust
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

```

### List of Official Plugins
[PCAP Parser](pcap_parser/README.md)

### Curated List of Community Plugins
This is sad
