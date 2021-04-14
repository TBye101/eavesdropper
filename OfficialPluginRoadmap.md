# Official Plugins Roadmap

## Version 0.1.0
 - [x] PCapParser
   - [x] Parses raw PCAP data to extract pcap data
   - [x] Puts parsed data into database table
   - [x] Documentation
   - [x] Tests

## Version 0.2.0
- [ ] PCapParser
  - [ ] Add column to table to contain hash of captured packet
  - [ ] Check to ensure the packet isn't already in the table via its hash in order to allow users to avoid redundant data when re-running the same captures
  - [ ] Update documentation
  - [ ] Tests
- [ ] TCP Parsing
  - [ ] Be able to identify TCP protocol packets
  - [ ] Be able to extract header information
  - [ ] Documentation
  - [ ] Tests
- [ ] UDP Parsing
  - [ ] Be able to identify UDP protocol packets
  - [ ] Be able to extract header information
  - [ ] Documentation
  - [ ] Tests
- [ ] IP Parsing
  - [ ] Be able to identify IP protocol packets
  - [ ] Be able to extract header information
  - [ ] Documentation
  - [ ] Tests
- [ ] DNS Parsing
  - [ ] Be able to identify DNS protocol packets
  - [ ] Be able to extract DNS information
  - [ ] Documentation
  - [ ] Tests
- [ ] WhoIs
  - [ ] Lookup information about IP addresses found
  - [ ] Documentation
  - [ ] Testing

## Version 0.3.0
- [ ] HTTP/S Parsing
  - [ ] Be able to identify HTTP/S protocol packets
  - [ ] Be able to extract information
  - [ ] Documentation
  - [ ] Tests
- [ ] Program Identifier
  - [ ] Be able to identify what program sent packets
  - [ ] Documentation
  - [ ] Tests

## Version 1.0.0
 - [ ] Stabilize the API
   - [ ] No more breaking changes until version 2.0.0
   - [ ] Documentation
   - [ ] Tests