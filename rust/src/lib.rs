use godot::prelude::*;

pub mod components;

use godot::engine::{Button, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Button)]
struct TestNode {
    #[export]
    speed: GString,
    angular_speed: f64,

    base: Base<Button>,
}

struct GrandfathersOfTheSaharaExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GrandfathersOfTheSaharaExtension {}
