#![deny(bare_trait_objects)]
#![feature(nll)]
#![feature(slice_patterns)]

extern crate rand;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod vec;
mod config;
#[macro_use]
mod misc;
pub mod item;
pub mod command;
pub mod world;
pub mod packet;
