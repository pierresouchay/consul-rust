#![crate_name = "consul"]
#![crate_type = "lib"]

#![feature(macro_rules)]
#![feature(phase)]

extern crate curl;
extern crate serialize;

#[phase(plugin, link)]
extern crate log;

pub mod agent;

#[cfg(test)]
mod test;
