//! A plugin for eavesdropper that parser pcap files and stores their information in a postgresql database.
//! Generally, this plugin will be a dependency for all other plugins in order to avoid reparsing the pcap files.

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

pub mod schema;//Needed by diesel in order to generate a bunch of code to represent tables and their columns.
pub mod models;//Where our table definitions are held

pub mod pcap_parser_module;