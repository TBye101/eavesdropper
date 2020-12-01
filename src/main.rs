/*
    https://crates.io/crates/pcap
    https://crates.io/crates/etherparse
    https://crates.io/crates/cargo-inspect
    https://crates.io/crates/sqlx
    https://crates.io/crates/static_assertions

    https://ip-api.com/
*/

extern crate etherparse;

use std::thread::JoinHandle;
use etherparse::PacketHeaders;
use pcap::Device;

fn main() {
    let mut listener_threads: Vec<JoinHandle<()>> = Vec::new();
    for device in Device::list().unwrap() {
        println!("Listening to device: {:?}", device);
        let join_handle = std::thread::spawn(|| {
            let device_name = device.name.clone();
            let cap_wrapped = device.open();
            if cap_wrapped.is_ok() {
                let mut cap = cap_wrapped.unwrap();
                let mut savefile = cap.savefile(format!("capture_device_{}.pcap", device_name)).unwrap();
                let mut count = 0;
    
                while let Ok(packet) = cap.next() {
                    //parsePacket(&packet);
                    count += 1;
                    println!("Number of packets captured: {:?} captured by: {:?}", count, device_name);
                    savefile.write(&packet);
                }
            }
            else {
                println!("Couldn't listen to {:?}", device_name);
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