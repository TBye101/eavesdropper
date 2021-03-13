use eframework::analysis_framework::AnalysisModule;

#[no_mangle]
pub extern "Rust" fn get_offical_modules() -> Vec<Box<dyn AnalysisModule>> {
    println!("Hello from our official module fetcher function thingy");
    Vec::new()
}