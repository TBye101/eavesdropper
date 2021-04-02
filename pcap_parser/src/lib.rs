#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

pub mod schema;//Needed by diesel in order to generate a bunch of code to represent tables and their columns.
pub mod models;//Where our table definitions are held

pub mod pcap_parser_module;