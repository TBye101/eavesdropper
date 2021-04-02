use std::time::SystemTime;

use super::schema::packets_pcap_parser;
use chrono::{DateTime, NaiveTime, Utc};
use diesel::{Expression, sql_types::{Integer, Timestamp}};
use libc::timeval;

//For what types bind to what SQL types:
//https://docs.rs/diesel/1.4.5/diesel/deserialize/trait.FromSql.html#impl-FromSql%3CDatetime%2C%20Mysql%3E-for-MYSQL_TIME
//How to add custom support: https://stackoverflow.com/questions/49092437/how-do-i-implement-queryable-and-insertable-for-custom-field-types-in-diesel

#[derive(Queryable)]
pub struct Packet {
    pub id: i32,
    pub payload: Vec<u8>,
    pub captured_timestamp: chrono::NaiveDateTime,
    pub capture_length: i32,
    pub packet_length: i32
}

#[derive(Insertable)]
#[table_name="packets_pcap_parser"]
pub struct NewPacket {
    pub payload: Vec<u8>,
    pub captured_timestamp: chrono::NaiveDateTime,
    pub capture_length: i32,
    pub packet_length: i32
}