-- Your SQL goes here
ALTER TABLE packets_pcap_parser
ADD COLUMN payload_hash varchar(16) NOT NULL;

CREATE UNIQUE INDEX deduplication_index ON packets_pcap_parser(payload_hash, captured_timestamp)