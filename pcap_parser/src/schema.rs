table! {
    packets_pcap_parser (id) {
        id -> Int4,
        payload -> Bytea,
        captured_timestamp -> Timestamp,
        capture_length -> Int4,
        packet_length -> Int4,
    }
}