use godot::prelude::*;

pub mod game;
pub mod scenes;
pub mod ui;
pub mod util;

struct GrandfathersOfTheSaharaExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GrandfathersOfTheSaharaExtension {}
