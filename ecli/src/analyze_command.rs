extern crate libloading;

use std::{collections::HashMap};

use cliargs_t::{Command, Flag};
use eframework::analysis_framework::AnalysisModule;
use libloading::Error;
use petgraph::{Directed, graphmap::{GraphMap}};

pub struct AnalyzeCommand {}

impl AnalyzeCommand {
    ///Parses a directory for analysis modules and constructs the found modules.
    fn parse_analyzer_plugins(&self, modules_directory: String) -> Vec<Box<dyn AnalysisModule>> {
        let file_names = std::fs::read_dir(modules_directory).unwrap();//Should be safe to unwrap as the is_dir() in execute_command() should disallow getting to this point for the error conditions throwable.
        let mut modules: Vec<Box<dyn AnalysisModule>> = Vec::new();
        for path in file_names {
            self.parse_plugin(&path.unwrap().path().into_os_string().into_string().unwrap(), &mut modules);
        }
        return modules;
    }

    ///Loads a single plugin library and extracts all analyzers from it.
    ///Inserts the analyzers into the specified HashMap.
    fn parse_plugin(&self, plugin_path: &String, loaded_modules: &mut Vec<Box<dyn AnalysisModule>>) {
        let module_load_attempt = self.load_module(plugin_path.to_string());

        if module_load_attempt.is_ok() {
            let modules = module_load_attempt.ok().unwrap();
            for analyzer in modules {
                loaded_modules.push(analyzer);
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

    ///Returns a vec with an order to execute modules in, or None should there be a dependency issue or cycle.
    ///Note: This ordering doesn't take into account parallel execution with different execution times per task.
    fn construct_execution_list(&self, mut modules: Vec<Box<dyn AnalysisModule>>) -> Option<Vec<Box<dyn AnalysisModule>>> {
        let mut id_to_module: HashMap<usize, Box<dyn AnalysisModule>> = HashMap::new();
        let mut name_to_id: HashMap<&'static str, usize> = HashMap::new();
        
        while let Some(item) = modules.pop() {
            let module_info = item.get_info();
            id_to_module.insert(id_to_module.len(), item);
            let duplicate = name_to_id.insert(module_info.name, name_to_id.len());
            if duplicate.is_some() {
                println!("Two modules with the same name have been found, stopping execution. The name: {}", module_info.name);
                return None;
            }
        }

        if !self.are_dependencies_valid(&id_to_module, &name_to_id) {
            return None;
        }

        let dependency_graph = self.build_dependency_graph(&id_to_module, &name_to_id);
        if dependency_graph.is_none() {
            return None;
        }

        let execution_order = petgraph::algo::toposort(&dependency_graph.unwrap(), None);

        match execution_order {
            Err(error) => {
                println!("A cyclic dependency was found for node: {}", error.node_id());
                return None;
            },
            Ok(order) => {
                let mut execution_order: Vec<Box<dyn AnalysisModule>> = Vec::new();
                for module_id in order {
                    execution_order.push(id_to_module.remove(&module_id).unwrap());
                }

                return Some(execution_order);
            }
        }
    }

    ///Builds the dependency graph.
    ///Returns None if there are cyclic dependencies.
    fn build_dependency_graph(&self, id_to_module: &HashMap<usize, Box<dyn AnalysisModule>>, name_to_id: &HashMap<&'static str, usize>) -> Option<GraphMap<usize, (), Directed>> {
        let mut dependency_graph: GraphMap<usize, (), Directed> = GraphMap::new();

        for module in id_to_module {
            dependency_graph.add_node(*module.0);

            for dependency in module.1.get_info().dependencies {
                let dependency_id = name_to_id[dependency.name];
                dependency_graph.add_edge(*module.0, dependency_id, ());
            }
        }

        return Some(dependency_graph);
    }

    ///Verifies that all dependencies are found and that their versions satisfy any requirements.
    fn are_dependencies_valid(&self, id_to_module: &HashMap<usize, Box<dyn AnalysisModule>>, name_to_id: &HashMap<&'static str, usize>) -> bool {
        for module in id_to_module {
            let currently_verifying_module = module.1.get_info();
            for dependency in  currently_verifying_module.dependencies {
                //Verify that the required module even exists
                let required_analyzer_id = name_to_id.get_key_value(dependency.name);
                if required_analyzer_id.is_none() {
                    println!("{} is missing a required module: {}", currently_verifying_module.name, dependency.name);
                    return false;
                }
                //Verify that the version is correct for the dependency
                let required_module = id_to_module.get_key_value(required_analyzer_id.unwrap().1);
                let found_version = required_module.unwrap().1.get_info().version;
                if !dependency.version_requirement.matches(&found_version) {
                    println!("{} is missing the required version of module {}", currently_verifying_module.name, dependency.name);
                    println!("The version that was found is {}, but the required version is {}", found_version, dependency.version_requirement);
                    return false;
                }
            }
        }
        return true;
    }

    ///Executes all modules in a ~~multi-threaded~~ single threaded manner.
    fn run_all_modules(&self, module_execution_order: Vec<Box<dyn AnalysisModule>>) {
        let mut table_names = Vec::new();
        for module in module_execution_order {
            let new_tables = module.analyze(&table_names);
            table_names.extend(new_tables);
        }
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
        let execution_order = self.construct_execution_list(discovered_modules);
        if execution_order.is_some() {
            self.run_all_modules(execution_order.unwrap());
        }
        else {
            println!("Can't prepare modules for execution.");
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