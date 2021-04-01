use super::schema::packets_pcap_parser;
use chrono::{DateTime, NaiveTime, Utc};
use diesel::sql_types::Integer;
use libc::timeval;

//For what types bind to what SQL types:
//https://docs.rs/diesel/1.4.5/diesel/deserialize/trait.FromSql.html#impl-FromSql%3CDatetime%2C%20Mysql%3E-for-MYSQL_TIME

#[derive(Queryable)]
pub struct Packets {
    id: u32,
    payload: Vec<u8>,
    captured_timestamp: timeval,
    capture_length: u32,
    packet_length: u32
}

#[derive(Insertable)]
#[table_name="packets_pcap_parser"]
pub struct NewPacket {
    payload: Vec<u8>,
    captured_timestamp: DateTime<Utc>,
    capture_length: i32,
    packet_length: i32
}