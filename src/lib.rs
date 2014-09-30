#![crate_name = "consul"]
#![crate_type = "lib"]

#![feature(macro_rules)]
#![feature(phase)]

extern crate curl;
extern crate serialize;

#[phase(plugin, link)]
extern crate log;

pub mod agent;
pub mod catalog;
pub mod structs;
pub mod health;

#[cfg(test)]
mod test;
