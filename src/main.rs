/*

    https://crates.io/crates/cargo-inspect

    Testing:
    https://crates.io/crates/static_assertions
    https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html

    db:
    https://github.com/tikv/tikv

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

use std::sync::Arc;
use std::thread::JoinHandle;
use pcap::Device;

use log_t::logging_abstraction::Logger;
use log_t::logging_implementations::FileLogger;

fn main() {
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

    while true {}
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