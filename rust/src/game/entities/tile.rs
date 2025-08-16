use crate::game::components::tile_component::TileComponent;
use crate::game::components::tile_component::TileData;
use crate::game::entities::treasure::Treasure;
use crate::game::entities::treasure::TreasureKind;
use crate::util::loader::GameConfig;
use crate::util::loader::TileConfig;
use crate::util::loader::TilesetConfig;
use crate::util::loader::TomlLoader;
use godot::builtin::Array;
use godot::builtin::Color;
use godot::builtin::GString;
use godot::classes::INode2D;

use godot::classes::Line2D;
use godot::classes::Node2D;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::{
    classes::Sprite2D,
    obj::Base,
    prelude::{godot_api, GodotClass},
};

static CROSS_IDS: [&str; 5] = ["cross_c", "cross_n", "cross_e", "cross_s", "cross_w"];

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
    cross_id: GString,
    #[export]
    cross_index: u8,
}

impl Tile {
    fn get_tile_component(&self) -> Gd<TileComponent> {
        self.base().get_node_as::<TileComponent>("./TileComponent")
    }
    fn get_treasure_at_direction(&self, direction: &CardinalDirection) -> Gd<Treasure> {
        let direction: &str = direction.into();

        self.base()
            .get_node_as::<Treasure>(&format!("./Layout/{}/Treasure", direction))
    }
    fn get_path_at_direction(&self, direction: &CardinalDirection) -> Gd<Line2D> {
        let direction: &str = direction.into();

        self.base()
            .get_node_as::<Line2D>(&format!("./Paths/{}", direction))
    }
    fn show_desert_icon(&self, is_desert: bool) {
        let is_cross = self.get_tile_component().bind().is_cross;

        self.base()
            .get_node_as::<Sprite2D>("./Layout/C/Desert")
            .set_visible(is_desert && !is_cross);
    }
}

#[godot_api]
impl INode2D for Tile {
    fn ready(&mut self) {
        self.show_desert_icon(true);

        let mut gd_tile_components = self.get_tile_component();

        let cross_id = self.cross_id.to_string();
        let mut tile_config: Option<TileConfig> = None;

        if !cross_id.is_empty() && CROSS_IDS.contains(&cross_id.as_str()) {
            let node: Gd<Node2D> = self.base.to_gd();
            let tileset = TomlLoader::get(node.upcast(), GameConfig::Tileset)
                .expect("Couldn't load tileset. Check if config/tileset.toml exists");

            let parsed_config = TilesetConfig::try_from(&tileset)
                .expect("Couldn't parse tileset. Check syntax of config/tileset.toml");

            if &cross_id == "cross_c" {
                tile_config = Some(parsed_config.cross.get_center());
            } else {
                tile_config = Some(
                    parsed_config.cross.get_side(&cross_id).unwrap()[self.cross_index as usize]
                        .clone(),
                );
            }
        }

        if let Some(tile_config) = tile_config {
            let tile_data = TileData::from(tile_config);

            let mut tile_components = gd_tile_components.bind_mut();
            tile_components.oasis_layout = Array::from(&tile_data.oasis_layout);

            let treasure_layout = tile_data.treasure_layout.iter().map(GString::from);

            tile_components.treasure_layout = Array::from_iter(treasure_layout);
        }

        let tile_components = gd_tile_components.bind();

        for (idx, oasis) in tile_components.oasis_layout.iter_shared().enumerate() {
            let direction = CardinalDirection::try_from(idx).expect("Error in Tile.ready");
            let mut gd_treasure = self.get_treasure_at_direction(&direction);

            if !oasis {
                gd_treasure.set_visible(false);
                let mut gd_path = self.get_path_at_direction(&direction);

                gd_path.set_default_color(Color::from_rgb(255., 255., 255.));

                continue;
            }
            let mut treasure = gd_treasure.bind_mut();

            if oasis {
                self.show_desert_icon(false);
            }

            let treasure_at_idx = tile_components
                .treasure_layout
                .get(idx)
                .unwrap_or(GString::from(""));

            let treasure_kind: TreasureKind = treasure_at_idx
                .try_into()
                .expect("Couldn't parse treasure_layout into");

            if treasure_kind != TreasureKind::None {
                self.show_desert_icon(false);
            }

            treasure.set_kind(treasure_kind);
        }
    }
}
