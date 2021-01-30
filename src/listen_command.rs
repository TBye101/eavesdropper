use std::sync::Arc;
use std::thread::JoinHandle;
use pcap::Device;
use log_t::logging_abstraction::Logger;
use log_t::logging_implementations::FileLogger;

pub struct ListenCommand {}

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