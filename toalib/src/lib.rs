#![deny(bare_trait_objects)]
#![feature(nll)]

extern crate rand;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

pub mod vec;
pub mod config;
pub mod tilemap;
pub mod team;
pub mod item;
pub mod command;
pub mod world;
pub mod damage;
pub mod packet;
pub mod net;
