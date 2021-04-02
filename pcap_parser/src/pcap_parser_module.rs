use std::{env, time::SystemTime};
use dotenv::dotenv;
use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, rvec, sabi_extern_fn, sabi_trait::prelude::TU_Opaque, std_types::{RString, RVec}};
use eframework::{RVersion::RVersion, analysis_framework::{AnalysisModule, AnalysisModuleBox, ModuleInfo, Plugin, Plugin_Ref, AnalysisModule_TO}};
use diesel::{Connection, RunQueryDsl, pg::PgConnection};
use pcap::{Capture, Offline};
use crate::models::NewPacket;
use crate::schema;

#[export_root_module]
pub fn get_library() -> Plugin_Ref {
    Plugin {
        get_analyzer,
    }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn get_analyzer() -> AnalysisModuleBox {
    AnalysisModule_TO::from_value(PCapParserModule {}, TU_Opaque)
}

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

    fn analyze(&self, pcap_input_directory: &RString, connection_string: &RString) {
        println!("Starting the PCapParser module!");
    
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection: PgConnection = PgConnection::establish(&connection_string)
        .expect(&format!("Error connecting to {}", database_url));

        let migration_result = diesel_migrations::run_pending_migrations(&connection);

        if migration_result.is_err() {
            println!("An error occurred while running migrations: {}", migration_result.err().unwrap());
            return;
        }

        self.parse_captures(pcap_input_directory, &connection);
    }
}

impl PCapParserModule {
    fn parse_captures(&self, pcap_input_directory: &RString, connection: &PgConnection) {
        let captures_result = std::fs::read_dir(pcap_input_directory.to_string());
        if captures_result.is_err() {
            println!("An error occurred while running migrations: {}", captures_result.err().unwrap());
            return;
        }

        let captures = captures_result.unwrap();
        for capture_file in captures {
            let stored_capture = Capture::from_file(capture_file.unwrap().path().into_os_string().into_string().unwrap());
            match stored_capture {
                Err(e) => {
                    println!("An error occured while parsing capture: {}", e.to_string());
                },
                Ok(capture) => {
                    self.parse_capture(connection, capture);
                }
            }
        }
    }

    fn parse_capture(&self, connection: &PgConnection, mut capture: Capture<Offline>) {
        while let Ok(packet) = capture.next() {
            let header = packet.header;
            let data = packet.data;
            let parsed_packet = NewPacket {
                payload: data.to_vec(),
                captured_timestamp: chrono::NaiveDateTime::from_timestamp(header.ts.tv_sec, 0),
                capture_length: header.caplen as i32,
                packet_length: header.len as i32,
            };

            let insert = diesel::insert_into(crate::schema::packets_pcap_parser::table)
                .values(&parsed_packet)
                .get_result::<crate::models::Packet>(connection);
            
            if insert.is_err() {
                println!("Error inserting packet into database");
            }
        }
    }
}