mod pcap_parser_module;

use eframework::analysis_framework::AnalysisModule;
use pcap_parser_module::PCapParserModule;

#[no_mangle]
pub extern "Rust" fn get_modules() -> Box<Vec<Box<dyn AnalysisModule>>> {
    println!("Hello from our official module fetcher function thingy4");
    let official_modules: Vec<Box<dyn AnalysisModule>> = vec![
        Box::new(PCapParserModule { })
    ];
    println!("Number of modules returning: {}", official_modules.len());
    return Box::new(official_modules);
}