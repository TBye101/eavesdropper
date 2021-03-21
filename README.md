 # Eavesdropper
 A packet sniffing and analyzing tool with a plugin system for analyzers.

Why? Because the people should have the same digital capabilities as their governments do.

## Plugins
Eavesdropper performs packet analysis via plugins.

### Creating a Plugin
Required crates:
* eframework
* [abi_stable](https://crates.io/crates/abi_stable)

### Example Plugin

``` Rust

use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, rvec, sabi_extern_fn, sabi_trait::prelude::TU_Opaque, std_types::{RString, RVec}};

use eframework::{RVersion::RVersion, analysis_framework::{AnalysisModule, AnalysisModuleBox, ModuleInfo, Plugin, Plugin_Ref, AnalysisModule_TO}};

#[export_root_module]
pub fn get_library() -> Plugin_Ref {
    Plugin {
        get_analyzer,
    }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn get_analyzer() -> AnalysisModuleBox {
    AnalysisModule_TO::from_value(PCapParserModule {}, TU_Opaque)
}

pub struct PCapParserModule { }

impl AnalysisModule for PCapParserModule {
    fn get_info(&self) -> eframework::analysis_framework::ModuleInfo {
        println!("We made it to the get_info()");
        return ModuleInfo {
            name: RString::from("PCapParser"),
            version: RVersion::new(0, 1, 0),
            dependencies: rvec![],
        }
    }

    fn analyze(&self, table_names: &RVec<RString>, pcap_input_directory: &RString) -> RVec<RString> {
        println!("Test hello world from the analysis function");
        return rvec!();
    }
}

```

 ## Framework Roadmap

 ### Version 0.1.0
- [x] Basic network sniffing support
  - [x] Listen to all available network interfaces and record to file
- [ ] Basic plugin system
  - [x] Load plugins from directory
  - [x] Can determine execution order from dependency and version information
  - [ ] Executes plugins against pcap data
  - [ ] Tests
- [ ] Complete documentation on everything so far
- [ ] Document how to make a plugin

### Version 0.2.0
- [ ] Parallel plugin execution
  - [ ] Whenever a plugin finishes executing, re-evaluate which plugins can be executed
  - [ ] Execute a sane (based upon cores) number of plugins in parallel
  - [ ] Tests
- [ ] Redo packet capturing GUI
  - [ ] Should display interfaces currently listening on
  - [ ] Packets received for each interface
  - [ ] Packets dropped due to OS or other issues for each interface
  - [ ] Tests

### Version 0.3.0
- [ ] Remote listeners
  - [ ] Able to receive packets captured by this tool from another machine
  - [ ] Able to send captured packets to another machine running an instance of this tool
  - [ ] Ignores or filters this traffic out in order to not capture and analyze them
  - [ ] Communication between machines is encrypted
  - [ ] Document how to setup remote listeners and receivers
  - [ ] Tests
- [ ] Listener configuration
  - [ ] Can configure what listeners are being utilized
  - [ ] Tests

### Version 0.4.0
- [ ] Distributed Analysis (Centralized)
  - [ ] Manager machine directs network of worker machines
  - [ ] Supports concept of database machines that only exist to store the data from the analysis
  - [ ] Worker machines process jobs and send data to either the manager machine, or designated database machines
  - [ ] Communication between machines is encrypted
  - [ ] Document how to setup and use centralized distributed analysis
  - [ ] Tests

### Version 0.5.0
- [ ] Distributed Analysis (Decentralized public model)
  - [ ] Can handle machines joining mid analysis
  - [ ] Can handle machines leaving mid analysis
  - [ ] Communication is encrypted between machines
  - [ ] If controller leaves, new controller is elected
  - [ ] Supports each machine having their own database or using another machine loyal to them as their database
  - [ ] Any machine can join the analysis network
  - [ ] Document how to setup and use decentralized public model
  - [ ] Optional authorization required to join the network
  - [ ] Tests

- [ ] Distributed Analysis (Fault tolerance model)
  - [ ] Distributed Analysis model but with a few differences:
    - [ ] Supports any number of database redundancies
    - [ ] Optional authorization required to join the network
    - [ ] Tests

### Version 0.6.0
- [ ] Realtime analysis
  - [ ] Analysis machine or analysis network can analyze packets as they are received
  - [ ] Document how to setup realtime analysis
  - [ ] Tests

### Version 0.7.0
- [ ] Partial re-analysis
  - [ ] Allow for partial re-analysis as data is decrypted in plugins
  - [ ] Allow manual input of encryption credentials
  - [ ] Document how to write plugins that support re-analysis
  - [ ] Tests

### Version 0.8.0
- [ ] Allow user to specify what plugins run
- [ ] Allow user to ignore specific addresses from being analyzed
- [ ] Allow scheduling of sniffing
- [ ] Allow scheduling of sniffed packet transmission
- [ ] Allow scheduling of analysis
- [ ] Tests

### Version 0.9.0
- [ ] ?

 ### Version 1.0.0
 - [ ] Stabilize the API
   - [ ] No more breaking changes until version 2.0.0

 ## Official Plugins Roadmap

 ### Version 0.1.0
 - [ ] Base analyzer plugin
   - [ ] Parses raw PCAP data to extract pcap data
   - [ ] Puts parsed data into database table
   - [ ] Tests

### Version 0.2.0
- [ ] TCP Parsing
  - [ ] Be able to identify TCP protocol packets
  - [ ] Be able to extract header information
  - [ ] Tests
- [ ] UDP Parsing
  - [ ] Be able to identify UDP protocol packets
  - [ ] Be able to extract header information
  - [ ] Tests
- [ ] IP Parsing
  - [ ] Be able to identify IP protocol packets
  - [ ] Be able to extract header information
  - [ ] Tests

### Version 0.3.0
- [ ] HTTP/S Parsing
  - [ ] Be able to identify HTTP/S protocol packets
  - [ ] Be able to extract header information
  - [ ] Tests
- [ ] DNS Parsing
  - [ ] Be able to identify DNS protocol packets
  - [ ] Be able to extract header information
  - [ ] Tests

 ### Version 1.0.0
 - [ ] Stabilize the API
   - [ ] No more breaking changes until version 2.0.0