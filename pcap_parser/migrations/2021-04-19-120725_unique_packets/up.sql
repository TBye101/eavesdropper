-- Your SQL goes here

CREATE UNIQUE INDEX deduplication_index ON packets_pcap_parser(payload, captured_timestamp)