use crate::game::components::tile_component::TileComponent;
use crate::game::components::tile_component::TileData;
use crate::game::entities::treasure::Treasure;
use crate::game::entities::treasure::TreasureKind;
use crate::util::loader::GameConfig;
use crate::util::loader::TileConfig;
use crate::util::loader::TilesetConfig;
use crate::util::loader::TomlLoader;
use bitflags::bitflags;
use godot::builtin::Array;
use godot::builtin::Color;
use godot::builtin::GString;
use godot::builtin::Vector2;
use godot::classes::INode2D;

use godot::classes::Input;
use godot::classes::Line2D;
use godot::classes::Node2D;
use godot::global::MouseButton;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::{
    classes::Sprite2D,
    obj::Base,
    prelude::{godot_api, GodotClass},
};

const CROSS_IDS: [&str; 5] = ["cross_c", "cross_n", "cross_e", "cross_s", "cross_w"];

bitflags! {
    #[derive(Debug, Clone, PartialEq)]
    pub struct CardinalDirectionFlags: u8 {
        const N = 0b0001;
        const E = 0b0010;
        const S = 0b0100;
        const W = 0b1000;
    }
}

bitflags! {
    #[derive(Debug, Clone, PartialEq)]
    pub struct OasisLayoutFlags: u16 {
        const N1 = CardinalDirectionFlags::N.bits() as u16;
        const E1 = CardinalDirectionFlags::E.bits() as u16;
        const S1 = CardinalDirectionFlags::S.bits() as u16;
        const W1 = CardinalDirectionFlags::W.bits() as u16;
        const N2 = (CardinalDirectionFlags::N.bits() as u16) << 4;
        const E2 = (CardinalDirectionFlags::E.bits() as u16) << 4;
        const S2 = (CardinalDirectionFlags::S.bits() as u16) << 4;
        const W2 = (CardinalDirectionFlags::W.bits() as u16) << 4;
        const N3 = (CardinalDirectionFlags::N.bits() as u16) << 8;
        const E3 = (CardinalDirectionFlags::E.bits() as u16) << 8;
        const S3 = (CardinalDirectionFlags::S.bits() as u16) << 8;
        const W3 = (CardinalDirectionFlags::W.bits() as u16) << 8;
        const N4 = (CardinalDirectionFlags::N.bits() as u16) << 12;
        const E4 = (CardinalDirectionFlags::E.bits() as u16) << 12;
        const S4 = (CardinalDirectionFlags::S.bits() as u16) << 12;
        const W4 = (CardinalDirectionFlags::W.bits() as u16) << 12;
    }
}

impl OasisLayoutFlags {
    pub fn from_cardinal_direction_flags(f: &CardinalDirectionFlags, idx: u8) -> Self {
        let bits = (f.bits() as u16) << (4 * idx);
        OasisLayoutFlags::from_bits_truncate(bits)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardinalDirection {
    N,
    E,
    S,
    W,
}

const DIRECTIONS: [CardinalDirection; 4] = [
    CardinalDirection::N,
    CardinalDirection::E,
    CardinalDirection::S,
    CardinalDirection::W,
];

impl From<CardinalDirectionFlags> for Vec<CardinalDirection> {
    fn from(value: CardinalDirectionFlags) -> Self {
        let mut direction_vec = vec![];

        value.iter_names().for_each(|f| match f.0 {
            "N" => direction_vec.push(CardinalDirection::N),
            "E" => direction_vec.push(CardinalDirection::E),
            "S" => direction_vec.push(CardinalDirection::S),
            "W" => direction_vec.push(CardinalDirection::W),
            _ => (),
        });

        direction_vec
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

    #[init(val = false)]
    is_active: bool,

    #[export]
    #[init(val = GString::from(""))]
    cross_id: GString,
    #[export]
    #[init(val = 0)]
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
    fn show_desert_icon_if_not_cross(&self, is_desert: bool) {
        let is_cross = self.get_tile_component().bind().is_cross;

        self.base()
            .get_node_as::<Sprite2D>("./Layout/C/Desert")
            .set_visible(is_desert && !is_cross);
    }
    pub fn set_from_tile_component(&mut self, tile_component_data: &TileComponent) {
        let mut gd_tile_component = self.get_tile_component();
        let mut tile_component = gd_tile_component.bind_mut();

        tile_component.is_cross = tile_component_data.is_cross;
        tile_component.oasis_layout = tile_component_data.oasis_layout;
        tile_component.treasure_layout = tile_component_data.treasure_layout.clone();
    }
    pub fn set_active(&mut self) {
        self.is_active = true;
    }
    pub fn place(&mut self) {
        self.is_active = false;
    }
}

#[godot_api]
impl INode2D for Tile {
    fn ready(&mut self) {
        {
            let mut gd_tile_components = self.get_tile_component();

            let cross_id = self.cross_id.to_string();
            let mut tile_config: Option<TileConfig> = None;

            if !cross_id.is_empty() && CROSS_IDS.contains(&cross_id.as_str()) {
                let tileset = TomlLoader::get(&self.base(), GameConfig::Tileset)
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
                tile_components.oasis_layout = tile_data.oasis_layout.bits();

                let treasure_layout = tile_data.treasure_layout.iter().map(GString::from);

                tile_components.treasure_layout = Array::from_iter(treasure_layout);
                tile_components.is_cross = tile_data.is_cross;
            }
        }

        let gd_tile_components = self.get_tile_component();
        let tile_components = gd_tile_components.bind();

        self.show_desert_icon_if_not_cross(true);

        for oasis_idx in 0..3 {
            let flag = (OasisLayoutFlags::from_bits_truncate(tile_components.oasis_layout).bits()
                >> (oasis_idx * 4)) as u8;

            let directions: Vec<CardinalDirection> =
                CardinalDirectionFlags::from_bits_truncate(flag).into();

            for direction in DIRECTIONS {
                let mut gd_treasure = self.get_treasure_at_direction(&direction);

                if flag == 0 {
                    let mut gd_path = self.get_path_at_direction(&direction);

                    gd_path.set_default_color(Color::from_rgb(255., 255., 255.));

                    continue;
                }

                if directions.contains(&direction) {
                    gd_treasure.set_visible(true);
                    self.show_desert_icon_if_not_cross(false);
                }

                let mut treasure = gd_treasure.bind_mut();

                let treasure_at_idx = tile_components
                    .treasure_layout
                    .get(oasis_idx)
                    .unwrap_or(GString::from(""));

                let treasure_kind: TreasureKind = treasure_at_idx
                    .try_into()
                    .expect("Couldn't parse treasure_layout into");

                if treasure_kind != TreasureKind::None {
                    self.show_desert_icon_if_not_cross(false);
                }

                treasure.set_kind(treasure_kind);
            }
        }
    }
    fn process(&mut self, _dt: f64) {
        if !self.is_active {
            return;
        }

        let input = Input::singleton();
        let pressed = input.is_mouse_button_pressed(MouseButton::LEFT);

        if pressed {
            self.is_active = false;
        }

        let mut base = self.base_mut();
        let mouse_position = base
            .get_viewport()
            .expect("Expected node to have a viewport")
            .get_mouse_position();

        base.set_position(mouse_position);
        if pressed {
            base.set_scale(Vector2::from_tuple((0.2, 0.2)))
        }
    }
}
