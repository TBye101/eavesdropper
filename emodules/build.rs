// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let testModuleDir = out_dir.clone() + "/../../../../../ModuleTest";
    let pcapDir = out_dir.clone() + "/../../../../../PcapFiles";
    let libPath = out_dir.clone() + "/../../../libemodules.so";
    let copiedLibPath = out_dir.clone() + "/../../../../../ModuleTest/emodules.so";

    println!("Output dir: {}", out_dir.clone());
    println!("test module dir: {}", testModuleDir.clone());
    println!("pcap dir: {}", pcapDir.clone());
    println!("lib path: {}", libPath.clone());
    println!(" copied lib path: {}", copiedLibPath.clone());

    let moduleDirCreated = fs::create_dir(testModuleDir.clone());
    let pcapDirCreated = fs::create_dir(pcapDir.clone());
    let deletedOld = fs::remove_file(copiedLibPath.clone());
    let copied = fs::copy(libPath.clone(), copiedLibPath.clone());

    if copied.is_err() {
        let err = copied.err().unwrap();
        println!("copy error: {}", err);
    }

    if moduleDirCreated.is_err() {
        let err = moduleDirCreated.err().unwrap();
        println!("module dir creation error: {}", err)
    }

    if pcapDirCreated.is_err() {
        let err = pcapDirCreated.err().unwrap();
        println!("pcap dir creation error: {}", err);
    }

    if deletedOld.is_err() {
        let err = deletedOld.err().unwrap();
        println!("deleted old creation error: {}", err);
    }
    else {
        println!("Deleted old lib");
    }
}