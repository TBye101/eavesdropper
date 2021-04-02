use std::{env, time::{Instant, SystemTime}};
use dotenv::dotenv;
use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, rvec, sabi_extern_fn, sabi_trait::prelude::TU_Opaque, std_types::{RString, RVec}};
use eframework::{RVersion::RVersion, analysis_framework::{AnalysisModule, AnalysisModuleBox, ModuleInfo, Plugin, Plugin_Ref, AnalysisModule_TO}};
use diesel::{Connection, RunQueryDsl, pg::PgConnection};
use pcap::{Capture, Offline};
use crate::models::NewPacket;
use crate::schema;

embed_migrations!();//Embed our Diesel migrations into this crate so we can run them upon beginning analysis later.

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

        let migration_result = embedded_migrations::run(&connection);

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
            let file_name = capture_file.unwrap().path().into_os_string().into_string().unwrap();
            let stored_capture = Capture::from_file(file_name.clone());
            match stored_capture {
                Err(e) => {
                    println!("An error occured while parsing capture: {}", e.to_string());
                },
                Ok(capture) => {
                    self.parse_capture(connection, capture, &file_name);
                }
            }
        }
    }

    fn parse_capture(&self, connection: &PgConnection, mut capture: Capture<Offline>, file_name: &String) {
        let mut packet_count: u64 = 0;
        let start_time = Instant::now();
        let batch_size_string = env::var("PCAP_PARSER_BATCH_SIZE").expect("Failed to get batch size for pcap_parser module");
        let batch_size = batch_size_string.parse::<usize>().unwrap();
        let mut batch: Vec<NewPacket> = Vec::with_capacity(batch_size);

        while let Ok(packet) = capture.next() {
            packet_count += 1;
            let header = packet.header;
            let data = packet.data;
            let parsed_packet = NewPacket {
                payload: data.to_vec(),
                captured_timestamp: chrono::NaiveDateTime::from_timestamp(header.ts.tv_sec, 0),
                capture_length: header.caplen as i32,
                packet_length: header.len as i32,
            };

            batch.push(parsed_packet);

            //Push the batch to the database if it is at our batch size
            if batch.len() >= batch_size {
                self.insert_packet_batch(connection, &batch);
                batch.clear();
            }
        }
        //Ensure that even if the last batch didn't exactly equal the batch size, it still makes it to the database
        self.insert_packet_batch(connection, &batch);

        println!("Loaded {} captured packets into the database in {} seconds from {}", packet_count, start_time.elapsed().as_secs(), file_name);
    }

    fn insert_packet_batch(&self, connection: &PgConnection, packets: &Vec<NewPacket>) {
        if packets.len() > 0 {
            let insert = diesel::insert_into(crate::schema::packets_pcap_parser::table)
            .values(packets)
            .get_result::<crate::models::Packet>(connection);
        
            if insert.is_err() {
                println!("Error inserting packets into database");
            }
        }
    }
}