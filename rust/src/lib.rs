use godot::prelude::*;

pub mod components;
pub mod game;
pub mod util;

struct GrandfathersOfTheSaharaExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GrandfathersOfTheSaharaExtension {}
