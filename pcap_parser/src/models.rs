use super::schema::packets_pcap_parser;

//For what types bind to what SQL types:
//https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html

//How to add custom support: 
//https://stackoverflow.com/questions/49092437/how-do-i-implement-queryable-and-insertable-for-custom-field-types-in-diesel

#[derive(Queryable)]
pub struct Packet {
    pub id: i32,
    pub payload: Vec<u8>,
    pub captured_timestamp: chrono::NaiveDateTime,
    pub capture_length: i32,
    pub packet_length: i32,
    pub payload_hash: String
}

#[derive(Insertable)]
#[table_name="packets_pcap_parser"]
pub struct NewPacket {
    pub payload: Vec<u8>,
    pub captured_timestamp: chrono::NaiveDateTime,
    pub capture_length: i32,
    pub packet_length: i32,
    pub payload_hash: String
}