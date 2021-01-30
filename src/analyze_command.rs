extern crate libloading;

#[path = "./analysis_framework.rs"]
mod analysis_framework;

use std::collections::HashMap;

use crate::analyze_command::analysis_framework::AnalysisModule;
use analysis_framework::ModuleInfo;
use cliargs_t::{Command, Flag};
use libloading::Error;
use petgraph::{Directed, graphmap::{DiGraphMap, GraphMap}};

//TODO:
//Use graph to verify all dependencies are found and there are no cyclic dependencies
//  This should take into account versions

pub struct AnalyzeCommand {}

impl AnalyzeCommand {
    ///Parses a directory for analysis modules and constructs the found modules.
    fn parse_analyzer_plugins(&self, modules_directory: String) -> HashMap<String, Box<dyn AnalysisModule>> {
        let file_names = std::fs::read_dir(modules_directory).unwrap();//Should be safe to unwrap as the is_dir() in execute_command() should disallow getting to this point for the error conditions throwable.
        let mut modules: HashMap<String, Box<dyn AnalysisModule>> = HashMap::new();
        for path in file_names {
            self.parse_plugin(&path.unwrap().path().into_os_string().into_string().unwrap(), &mut modules);
        }
        return modules;
    }

    ///Loads a single plugin library and extracts all analyzers from it.
    ///Inserts the analyzers into the specified HashMap.
    fn parse_plugin(&self, plugin_path: &String, loaded_modules: &mut HashMap<String, Box<dyn AnalysisModule>>) {
        let module_load_attempt = self.load_module(plugin_path.to_string());

        if module_load_attempt.is_ok() {
            let modules = module_load_attempt.ok().unwrap();
            for analyzer in modules {
                let analyzer_info = analyzer.get_info();
                let previous_value = loaded_modules.insert(analyzer_info.name.to_string(), analyzer);
                if previous_value.is_some() {
                    println!("Either module was loaded twice, or two modules with the same names were present: {}", analyzer_info.name);
                }
            }
        }
    }

    ///Loads a plugin library and returns the analyzers from it.
    fn load_module(&self, module_path: String) -> Result<Vec<Box<dyn AnalysisModule>>, String> {
        let lib_load = libloading::Library::new(&module_path);
        if lib_load.is_err() {
            return Err(lib_load.err().unwrap().to_string());
        }

        let lib = lib_load.unwrap();
        let module_load: Result<libloading::Symbol<extern "Rust" fn() -> Vec<Box<dyn AnalysisModule>>>, Error>;
        unsafe {
            module_load = lib.get(b"get_modules");
        }

        if module_load.is_ok() {
            return Ok(module_load.unwrap()());
        }
        else {
            println!("Error loading module: {}", module_path);
            return Err(module_load.err().unwrap().to_string());
        }
    }

    ///Returns a graph if the depencies are all okay, otherwise returns None.
    fn are_analyzer_dependencies_ok(&self, modules: &HashMap<String, Box<dyn AnalysisModule>>) -> Option<DiGraphMap<ModuleInfo, ()>> {
        let graph: GraphMap<&String, (), Directed> = GraphMap::new();//<moduleName, edgeType>
        todo!();//Consider loading the analysis modules into a dictionary of <name, moduleInfo> for performance reasons
    }

    fn determine_execution_order(&self, module_graph: GraphMap<ModuleInfo, (), Directed>) -> Option<Vec<Box<dyn AnalysisModule>>> {
        todo!();
    }

    fn run_all_modules(&self, module_execution_order: Vec<Box<dyn AnalysisModule>>) {
        todo!();
    }
}

impl Command for AnalyzeCommand {
    
    fn execute_command(&self, flags: std::collections::HashMap<std::string::String, std::string::String>) { 
        let module_directory = flags.get_key_value("m").unwrap().1;
        let module_dir_exists = std::path::Path::new(module_directory).is_dir(); //We can unwrap the flag here as it is a required flag and is guaranteed to be present
        if !module_dir_exists {
            println!("Module directory does not exist");
            return; 
        }
        let discovered_modules = self.parse_analyzer_plugins(module_directory.to_string());
        //https://docs.rs/petgraph/0.5.1/petgraph/graphmap/struct.GraphMap.html
        //Add these to a graph and verify that there are no cyclic dependencies
        //Then grab our starting analyzer, and start queueing up work via rayon as required dependencies finish

        let validated_graph = self.are_analyzer_dependencies_ok(&discovered_modules);
        match validated_graph {
            None => println!("Failed to validate module graph"),
            Some(graph) => {
                match self.determine_execution_order(graph) {
                    None => println!("Failed to determine execution order"),
                    Some(execution_order) => self.run_all_modules(execution_order)
                }
            } 
        }
    }

    fn get_information(&self) -> cliargs_t::CommandInformation { 
        return cliargs_t::CommandInformation {
            command_name: "analyze",
            command_help: "Analyzes pcap files found within a directory using all available modules and gleans as much information as possible from them.",
            flags: vec![
                Flag {
                    identifier: "p",
                    flag_help: "The relative path to the directory containing the pcap files to analyze", 
                    required: true 
                },
                Flag {
                    identifier: "m",
                    flag_help: "The relative path to the directory containing the modules we would like to use to analyze the pcap files",
                    required: true
                }
            ],
        }
    }
}