#![deny(bare_trait_objects)]
#![feature(nll)]
#![feature(slice_patterns)]

extern crate rand;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

pub mod vec;
pub mod config;
#[macro_use]
pub mod misc;
pub mod team;
pub mod item;
pub mod command;
pub mod world;
pub mod aim;
pub mod damage;
pub mod packet;
pub mod net;
