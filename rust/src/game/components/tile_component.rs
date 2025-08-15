use godot::builtin::GString;

use godot::classes::{INode, Node};
use godot::obj::Gd;
use godot::prelude::GodotClass;
use godot::{builtin::Array, obj::Base, prelude::godot_api};

#[derive(Default, Debug, Clone)]
pub struct TileData {
    pub is_cross: bool,
    pub oasis_layout: [bool; 4],
    pub treasure_layout: [String; 4],
}

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct TileComponent {
    base: Base<Node>,

    #[export]
    pub is_cross: bool,
    #[export]
    pub oasis_layout: Array<bool>,
    #[export]
    pub treasure_layout: Array<GString>,
}

#[godot_api]
impl INode for TileComponent {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            is_cross: false,
            treasure_layout: godot::builtin::array!["", "", "", ""],
            oasis_layout: godot::builtin::array![false, false, false, false],
        }
    }
}

#[godot_api]
impl TileComponent {
    fn from_tile_data(tile_data: TileData) -> Gd<Self> {
        let oasis_layout: Array<bool> = tile_data.oasis_layout.into_iter().collect();
        let treasure_layout = tile_data
            .treasure_layout
            .into_iter()
            .map(|s| GString::from(s.to_owned()))
            .collect();

        Gd::from_init_fn(|base| Self {
            base,
            is_cross: tile_data.is_cross,
            oasis_layout,
            treasure_layout,
        })
    }
}

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct TileDeckComponent {
    base: Base<Node>,

    pub tiles: [TileData; 17],
}

#[godot_api]
impl INode for TileDeckComponent {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            tiles: std::array::from_fn(|_| TileData::default()),
        }
    }
}
