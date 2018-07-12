#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate redis;
extern crate r2d2;
extern crate r2d2_redis;
extern crate rocket_cors;

pub use server::launch;
pub mod server;

pub use temperaturedata::TemperatureData;
pub mod temperaturedata;