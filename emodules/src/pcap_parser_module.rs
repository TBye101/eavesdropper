use eframework::analysis_framework::{AnalysisModule, ModuleInfo};
use semver::Version;

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

pub struct PCapParserModule { }

impl AnalysisModule for PCapParserModule {
    fn get_info(&self) -> eframework::analysis_framework::ModuleInfo {
        println!("We made it to the get_info()");
        return ModuleInfo {
            name: "PCapParser",
            version: Version::new(0, 1, 0),
            dependencies: vec![],
        }
    }

    fn analyze(&self, table_names: &Vec<String>, pcap_input_directory: &String) -> Vec<String> {
        println!("Test hello world from the analysis function");
        return vec!();
    }
}

impl Drop for PCapParserModule {
    fn drop(&mut self) {
        println!("[{}] Destroyed instance!", 1);
    }
}