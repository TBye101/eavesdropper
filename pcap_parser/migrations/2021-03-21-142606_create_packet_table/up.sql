CREATE TABLE packets_pcap_parser (
    id SERIAL PRIMARY KEY,
    payload BYTEA NOT NULL,
    captured_timestamp TIMESTAMP NOT NULL,
    capture_length INTEGER NOT NULL, -- How much data was captured of the packet
    packet_length INTEGER NOT NULL -- How much data the header said was in the packet
)