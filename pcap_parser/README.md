# PCap_Parser

A plugin to extract information from raw PCAP files and store them in the shared database.

## Table Definition

This plugin creates one table to store its extracted PCAP information:

[Raw SQL](migrations/2021-03-21-142606_create_packet_table/up.sql)

[Importable Insert and Query Diesel Structs](src/models.rs)

## Environment Variables

This plugin requires the following to be declared in the .env file:

```PCAP_PARSER_BATCH_SIZE=batch_size_number```

Where ```batch_size_number``` is the number of packets to batch before inserting into the database. A decent default for this might be 4096.

## Unique Packets
Current behavior for this plugin is to ensure that each packet stored is unique. This is to enable the same capture files to be run again without duplicating data in the database. Criteria for uniqueness is a combination of a hash of the packet's data as well as the timestamp of when the packet was captured. If a packet in a batch is not unique, then an error will be logged and the packet batch will not be stored in the database.