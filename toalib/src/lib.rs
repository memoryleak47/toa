#![deny(bare_trait_objects)]
#![feature(nll)]
#![feature(slice_patterns)]

extern crate sfml;
extern crate rand;
extern crate objekt;

#[macro_use]
extern crate lazy_static;

mod config;
#[macro_use]
mod misc;
mod sound;
mod graphics;
mod item;
mod command;
mod view;
mod world;
mod input;
mod player;
