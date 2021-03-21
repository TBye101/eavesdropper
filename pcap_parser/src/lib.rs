#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;

pub mod schema;//Needed by diesel in order to generate a bunch of code to represent tables and their columns.
pub mod models;//Where our table definitions are held

embed_migrations!();//Embed our Diesel migrations into this crate so we can run them upon beginning analysis later.