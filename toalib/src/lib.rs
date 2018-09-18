#![deny(bare_trait_objects)]
#![feature(nll)]
#![feature(slice_patterns)]

extern crate rand;
extern crate objekt;

#[macro_use]
extern crate lazy_static;

mod vec;
mod config;
#[macro_use]
mod misc;
mod item;
mod command;
mod world;
