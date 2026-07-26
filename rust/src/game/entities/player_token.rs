use godot::builtin::Color;
use godot::classes::INode2D;
use godot::classes::Node2D;
use godot::classes::Sprite2D;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::game::entities::player::PlayerName;

#[derive(GodotClass, Debug)]
#[class(init, base=Node2D)]
pub struct PlayerToken {
    base: Base<Node2D>,

    pub player: PlayerName,
}

impl PlayerToken {
    fn get_sprite(&self) -> Gd<Sprite2D> {
        self.base().get_node_as("./Sprite2D")
    }
    fn tint(&self) {
        let color = self.player.color();

        // The camel icon is light brown, so a plain multiply mutes the tint.
        // Overshoot the modulate so the primary saturates over that base.
        let vibrant = Color {
            r: color.r * 3.0,
            g: color.g * 3.0,
            b: color.b * 3.0,
            a: 1.0,
        };

        self.get_sprite().set_modulate(vibrant);
    }
    pub fn assign_token_to_player(&mut self, player: PlayerName) {
        self.player = player;
        self.tint();
    }
}

#[godot_api]
impl INode2D for PlayerToken {
    fn ready(&mut self) {
        self.tint();
    }
}
