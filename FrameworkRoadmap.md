# Framework Roadmap

## Version 0.1.0
- [x] Basic network sniffing support
  - [x] Listen to all available network interfaces and record to file
- [ ] Basic plugin system
  - [x] Load plugins from directory
  - [x] Can determine execution order from dependency and version information
  - [x] Executes plugins against pcap data
  - [ ] Tests
- [ ] Complete documentation on everything so far
- [x] Document how to make a plugin

## Version 0.2.0
- [ ] Parallel plugin execution
  - [ ] Whenever a plugin finishes executing, re-evaluate which plugins can be executed
  - [ ] Execute a sane (based upon cores) number of plugins in parallel
  - [ ] Documentation
  - [ ] Tests
- [ ] Redo packet capturing GUI
  - [ ] Should display interfaces currently listening on
  - [ ] Packets received for each interface
  - [ ] Packets dropped due to OS or other issues for each interface
  - [ ] Documentation
  - [ ] Tests

## Version 0.3.0
- [ ] Remote listeners
  - [ ] Able to receive packets captured by this tool from another machine
  - [ ] Able to send captured packets to another machine running an instance of this tool
  - [ ] Ignores or filters this traffic out in order to not capture and analyze them
  - [ ] Communication between machines is encrypted
  - [ ] Document how to setup remote listeners and receivers
  - [ ] Tests
- [ ] Listener configuration
  - [ ] Can configure what listeners are being utilized
  - [ ] Documentation
  - [ ] Tests

## Version 0.4.0
- [ ] Distributed Analysis (Centralized)
  - [ ] Manager machine directs network of worker machines
  - [ ] Supports concept of database machines that only exist to store the data from the analysis
  - [ ] Worker machines process jobs and send data to either the manager machine, or designated database machines
  - [ ] Communication between machines is encrypted
  - [ ] Document how to setup and use centralized distributed analysis
  - [ ] Tests

## Version 0.5.0
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
    - [ ] Documentation
    - [ ] Tests

## Version 0.6.0
- [ ] Realtime analysis
  - [ ] Analysis machine or analysis network can analyze packets as they are received
  - [ ] Document how to setup realtime analysis
  - [ ] Tests
- [ ] Dashboards or Reports
  - [ ] Some sort of way to build dashboards or reports
  - [ ] Automatically update as data changes
  - [ ] Can be saved
  - [ ] Documentation
  - [ ] Tests

## Version 0.7.0
- [ ] Partial re-analysis
  - [ ] Allow for partial re-analysis as data is decrypted in plugins
  - [ ] Allow manual input of encryption credentials
  - [ ] Document how to write plugins that support re-analysis
  - [ ] Tests

## Version 0.8.0
- [ ] Allow user to specify what plugins run
- [ ] Allow user to ignore specific addresses from being analyzed
- [ ] Allow scheduling of sniffing
- [ ] Allow scheduling of sniffed packet transmission
- [ ] Allow scheduling of analysis
- [ ] Documentation
- [ ] Tests

## Version 1.0.0
 - [ ] Stabilize the API
   - [ ] No more breaking changes until version 2.0.0
   - [ ] Documentation
   - [ ] Tests