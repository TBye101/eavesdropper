// build.rs

use std::env;
use std::fs;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let test_module_dir = out_dir.clone() + "/../../../../../ModuleTest";
    let pcap_dir = out_dir.clone() + "/../../../../../PcapFiles";
    let lib_path = out_dir.clone() + "/../../../libpcap_parser.so";
    let copied_lib_path = out_dir.clone() + "/../../../../../ModuleTest/pcap_parser.so";

    println!("Output dir: {}", out_dir.clone());
    println!("test module dir: {}", test_module_dir);
    println!("pcap dir: {}", pcap_dir.clone());
    println!("lib path: {}", lib_path.clone());
    println!(" copied lib path: {}", copied_lib_path);

    let module_dir_created = fs::create_dir(test_module_dir);
    let pcap_dir_created = fs::create_dir(pcap_dir);
    let deleted_old = fs::remove_file(copied_lib_path.clone());
    let copied = fs::copy(lib_path.clone(), copied_lib_path);

    if copied.is_err() {
        let err = copied.err().unwrap();
        println!("copy error: {}", err);
    }

    if module_dir_created.is_err() {
        let err = module_dir_created.err().unwrap();
        println!("module dir creation error: {}", err)
    }

    if pcap_dir_created.is_err() {
        let err = pcap_dir_created.err().unwrap();
        println!("pcap dir creation error: {}", err);
    }

    if deleted_old.is_err() {
        let err = deleted_old.err().unwrap();
        println!("deleted old creation error: {}", err);
    }
    else {
        println!("Deleted old lib");
    }
}