#![deny(bare_trait_objects)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod vec;
pub use vec::*;

mod config;
pub use config::*;

mod tilemap;
pub use tilemap::*;

mod team;
pub use team::*;

mod item;
pub use item::*;

mod command;
pub use command::*;

mod world;
pub use world::*;

mod damage;
pub use damage::*;

mod packet;
pub use packet::*;

mod net;
pub use net::*;

mod server;

#[cfg(feature = "client")]
mod cli;

#[cfg(feature = "client")]
mod misc;
#[cfg(feature = "client")]
pub use misc::*;

#[cfg(feature = "client")]
mod vec_compat;
#[cfg(feature = "client")]
pub use vec_compat::*;

#[cfg(feature = "client")]
mod graphics;
#[cfg(feature = "client")]
pub use graphics::*;

#[cfg(feature = "client")]
mod gameobject;
#[cfg(feature = "client")]
pub use gameobject::*;

#[cfg(feature = "client")]
mod sound;
#[cfg(feature = "client")]
pub use sound::*;

#[cfg(feature = "client")]
mod app;
#[cfg(feature = "client")]
pub use app::*;

#[cfg(feature = "client")]
mod menu;
#[cfg(feature = "client")]
pub use menu::*;

#[cfg(feature = "client")]
mod animation;
#[cfg(feature = "client")]
pub use animation::*;

#[cfg(feature = "client")]
pub use sfml::{
    audio::Music,
    graphics::{
        Color, Font, IntRect, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text,
        Texture, Transformable,
    },
    system::{Vector2f, Vector2u},
    window::{mouse::Button, Event, Key, Style, VideoMode},
    SfBox,
};

pub use std::collections::{HashMap, HashSet};
pub use std::{iter, mem};

#[cfg(feature = "client")]
fn main() {
    match cli::get_arg().as_str() {
        "server" => server::run(),
        ip => {
            let mut app = App::connect(&ip);
            app.run();
        }
    };
}

#[cfg(not(feature = "client"))]
fn main() {
    server::run();
}
