use godot::builtin::GString;

use godot::classes::{INode, Node};
use godot::obj::Gd;
use godot::prelude::GodotClass;
use godot::{builtin::Array, obj::Base, prelude::godot_api};

use crate::util::loader::{GameConfig, TileConfig, TilesetConfig, TomlLoader};

#[derive(Default, Debug, Clone)]
pub struct TileData {
    pub is_cross: bool,
    pub oasis_layout: [bool; 4],
    pub treasure_layout: [String; 4],
}

impl From<TileConfig> for TileData {
    fn from(value: TileConfig) -> Self {
        let oasis_layout = [
            value.n.unwrap_or(false),
            value.e.unwrap_or(false),
            value.s.unwrap_or(false),
            value.w.unwrap_or(false),
        ];
        let treasure_layout = [
            value.treasure_n.clone().unwrap_or("none".to_owned()),
            value.treasure_e.clone().unwrap_or("none".to_owned()),
            value.treasure_s.clone().unwrap_or("none".to_owned()),
            value.treasure_w.clone().unwrap_or("none".to_owned()),
        ];

        Self {
            is_cross: false,
            oasis_layout,
            treasure_layout,
        }
    }
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
    pub fn from_tile_data(tile_data: TileData) -> Gd<Self> {
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

pub trait NextTileData {
    fn get_next_tile_data(&mut self) -> Option<NextTileDataRemaining>;
}

#[derive(Debug, Clone)]
pub struct NextTileDataRemaining(pub TileData, pub u8);

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct TileDeckComponent {
    base: Base<Node>,

    pub index: u8,
    pub tiles: [TileData; 17],
}

impl NextTileData for TileDeckComponent {
    fn get_next_tile_data(&mut self) -> Option<NextTileDataRemaining> {
        if let Some(tile) = self.tiles.get((self.index) as usize).cloned() {
            self.index += 1;

            Some(NextTileDataRemaining(tile, 17 - self.index))
        } else {
            None
        }
    }
}

#[godot_api]
impl TileDeckComponent {
    pub fn from_tile_deck_index(node: Gd<Node>, deck_index: u8) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            let tileset = TomlLoader::get(node, GameConfig::Tileset)
                .expect("Couldn't load tileset. Check if config/tileset.toml exists");

            let parsed_config = TilesetConfig::try_from(&tileset)
                .expect("Couldn't parse tileset. Check syntax of config/tileset.toml");

            let tiles = parsed_config
                .deck
                .get(deck_index as usize)
                .expect("Couldn't parse tileset. Check syntax of config/tileset.toml")
                .0
                .clone()
                .map(TileData::from);

            Self {
                base,
                index: 0,
                tiles,
            }
        })
    }
}

#[godot_api]
impl INode for TileDeckComponent {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            index: 0,
            tiles: std::array::from_fn(|_| TileData::default()),
        }
    }
}
