use godot::prelude::*;

pub mod components;
pub mod game;

struct GrandfathersOfTheSaharaExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GrandfathersOfTheSaharaExtension {}
