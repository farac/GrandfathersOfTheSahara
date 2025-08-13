use crate::game::objects::treasure::Treasure;
use crate::game::objects::treasure::TreasureKind;
use crate::godot_dbg;
use godot::builtin::GString;
use godot::classes::INode2D;

use godot::classes::Node2D;
use godot::global::godot_print;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::{
    builtin::Array,
    classes::Sprite2D,
    obj::Base,
    prelude::{godot_api, GodotClass},
};

#[derive(Debug, Clone)]
pub enum CardinalDirection {
    N,
    E,
    S,
    W,
}

impl TryFrom<usize> for CardinalDirection {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CardinalDirection::N),
            1 => Ok(CardinalDirection::E),
            2 => Ok(CardinalDirection::S),
            3 => Ok(CardinalDirection::W),
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
    // fn get_layout_node(&self) -> Gd<Control> {
    //     self.to_gd().get_node_as::<Control>("./Layout")
    // }
    fn get_treasure_at_direction(&self, direction: &CardinalDirection) -> Gd<Treasure> {
        let direction: &str = direction.into();

        self.to_gd()
            .get_node_as::<Treasure>(&format!("./Layout/{}/Treasure", direction))
    }
    fn set_desert(&self, is_desert: bool) {
        self.to_gd()
            .get_node_as::<Sprite2D>("./Layout/C/Desert")
            .set_visible(is_desert);
    }
}

#[godot_api]
impl INode2D for Tile {
    fn ready(&mut self) {
        self.set_desert(true);

        godot_dbg!(self);

        for (idx, oasis) in self.oasis_layout.iter_shared().enumerate() {
            let direction = CardinalDirection::try_from(idx).expect("Error in Tile.ready");
            let mut gd_treasure = self.get_treasure_at_direction(&direction);

            if !oasis {
                gd_treasure.set_visible(false);

                continue;
            }
            let mut treasure = gd_treasure.bind_mut();

            if oasis {
                self.set_desert(false);
            }

            let treasure_at_idx = self.treasure_layout.get(idx).unwrap_or(GString::from(""));

            let treasure_kind: TreasureKind = treasure_at_idx
                .try_into()
                .expect("Couldn't parse treasure_layout into");

            if treasure_kind != TreasureKind::None {
                self.set_desert(false);
            }

            treasure.set_kind(treasure_kind);
        }
    }
}
