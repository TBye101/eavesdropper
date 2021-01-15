/*

    https://crates.io/crates/cargo-inspect

    https://crates.io/crates/indicatif


    Testing:
    https://crates.io/crates/static_assertions
    https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
    https://crates.io/crates/assert_cmd

    db:
    https://github.com/spacejam/sled

    performance:
    https://github.com/async-rs/async-std
    https://github.com/tokio-rs/tokio

    Analysis:
    https://crates.io/crates/pcap
    https://crates.io/crates/etherparse
    https://ip-api.com/

    https://crates.io/crates/clap or preferably https://crates.io/crates/pico-args

*/

extern crate etherparse;

use std::io::Read;
use std::sync::Arc;
use std::thread::JoinHandle;

use pcap::Device;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use log_t::logging_abstraction::Logger;
use log_t::logging_implementations::FileLogger;

fn main() {
    let listen_command: Box<cliargs_t::Command> = Box::new(ListenCommand {});
    let mut commands = vec![
        listen_command
    ];
    let commander = cliargs_t::Commander::new(&mut commands);

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                commander.handle_input(line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

struct ListenCommand {}

impl cliargs_t::Command for ListenCommand {
    
    fn execute_command(&self, _: std::collections::HashMap<std::string::String, std::string::String>) {
        let capture_manager_thread = std::thread::spawn(move || {
            let log = FileLogger::new_from_static_string("eavesdropping_log.txt").ok().unwrap();
            let shared_log = Arc::new(log);
        
            let mut listener_threads: Vec<JoinHandle<()>> = Vec::new();
            for device in Device::list().unwrap() {
                shared_log.write(format!("Listening to device: {:?}", device));
                println!("Listening to device: {:?}", device);
                let shared_log_clone = shared_log.clone();
                let join_handle = std::thread::spawn(move || {
                    let device_name = device.name.clone();
                    let cap_wrapped = device.open();
                    if cap_wrapped.is_ok() {
                        let mut cap = cap_wrapped.unwrap();
                        let mut savefile = cap.savefile(format!("capture_device_{}.pcap", device_name)).unwrap();
                        let mut count = 0;
            
                        while let Ok(packet) = cap.next() {
                            count += 1;
                            shared_log_clone.write(format!("Number of packets captured: {:?} captured by: {:?}", count, device_name));
                            savefile.write(&packet);
                        }
                    }
                    else {
                        shared_log_clone.write(format!("Couldn't listen to {:?}", device_name));
                    }
                });
                listener_threads.push(join_handle);
            }
        });
    }

    fn get_information(&self) -> cliargs_t::CommandInformation { 
        return cliargs_t::CommandInformation {
            command_name: "listen",
            command_help: "Captures packet traffic",
            flags: vec![
                cliargs_t::Flag {
                    identifier: "a",
                    flag_help: "Listen to all available devices/networks",
                    required: false
                }
            ]
        }
    }
}

    // let mut devices = Device::list().unwrap();
    // println!("{:?}", devices);
    // let mut selected = Device::lookup().unwrap();
    // println!("Selected: {:?}", selected);
    // let mut cap = Device::lookup().unwrap().open().unwrap();
    // let mut savefile = cap.savefile("test.pcap").unwrap();
    // let mut count = 0;
    // while let Ok(packet) = cap.next() {
    //     parsePacket(&packet);
    //     count += 1;
    //     println!("Number of packets captured: {}", count);
    //     savefile.write(&packet);
    // }

// fn parsePacket(packet: &pcap::Packet) {
//     match PacketHeaders::from_ethernet_slice(&packet) {
//         Err(value) => println!("Err {:?}", value),
//         Ok(value) => {
//             println!("link: {:?}", value.link);
//             println!("vlan: {:?}", value.vlan);
//             println!("ip: {:?}", value.ip);
//             println!("transport: {:?}", value.transport);
//         }
//     }
// }