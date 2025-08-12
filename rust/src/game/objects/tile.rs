use std::io::ErrorKind;

use crate::array;
use crate::game::objects::treasure;
use crate::game::objects::treasure::Treasure;
use crate::game::objects::treasure::TreasureKind;
use godot::builtin::GString;
use godot::classes::INode2D;
use godot::classes::Image;
use godot::classes::ImageTexture;

static OASIS_ICON_PATH: &str = "res://assets/icons/tile/oasis.png";
// static DESERT_ICON_PATH: &str = "res://assets/icons/tile/desert.png";

use godot::classes::Node2D;
use godot::global::godot_print;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::{
    builtin::Array,
    classes::{ISprite2D, Sprite2D},
    obj::Base,
    prelude::{godot_api, GodotClass},
};

#[derive(Debug, Clone)]
pub enum CardinalDirection {
    N = 1,
    E = 2,
    S = 3,
    W = 4,
}

impl TryFrom<usize> for CardinalDirection {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CardinalDirection::N),
            2 => Ok(CardinalDirection::E),
            3 => Ok(CardinalDirection::S),
            4 => Ok(CardinalDirection::W),
            _ => Err("Cardinal direction enum is out of range (1, 2, 3, 4)"),
        }
    }
}

impl From<&CardinalDirection> for &'static str {
    fn from(value: &CardinalDirection) -> Self {
        match value {
            CardinalDirection::N => "N",
            CardinalDirection::E => "E",
            CardinalDirection::S => "S",
            CardinalDirection::W => "W",
        }
    }
}

#[derive(GodotClass, Debug)]
#[class(init, base=Node2D)]
pub struct Tile {
    base: Base<Node2D>,

    #[export]
    oasis_layout: Array<bool>,
    #[export]
    treasure_layout: Array<GString>,
}

impl Tile {
    fn get_treasure_at_direction(&self, direction: &CardinalDirection) -> Gd<Treasure> {
        let direction: &str = direction.into();

        self.to_gd()
            .get_node_as::<Treasure>(&format!("./Layout/{}/Treasure", direction))
    }
}

#[godot_api]
impl INode2D for Tile {
    fn ready(&mut self) {
        for (idx, _oasis) in self.oasis_layout.iter_shared().enumerate() {
            let direction = CardinalDirection::try_from(idx + 1).expect("Error in Tile.ready");
            // Add 1 because the cardinal direction is 1-indexed
            let gd_treasure = self.get_treasure_at_direction(&direction);
            let treasure = gd_treasure.bind();

            godot_print!("{:?} {:?}", &direction, &treasure.kind);

            if treasure.kind.is_none() {
                let mut treasure_sprite = treasure.get_sprite();

                treasure_sprite.set_visible(false);
            }
        }
    }
}
