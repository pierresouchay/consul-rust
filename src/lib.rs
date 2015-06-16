#![crate_name = "consul"]
#![crate_type = "lib"]

extern crate curl;
extern crate rustc_serialize;


pub mod agent;
pub mod catalog;
pub mod structs;
pub mod health;
pub mod client;

#[cfg(test)]
mod test;
