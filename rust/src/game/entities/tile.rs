use crate::game::components::hover_outline::CollisionOutline;
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
use godot::global::godot_error;
use godot::global::godot_print;
use godot::global::MouseButton;
use godot::obj::Gd;
use godot::obj::InstanceId;
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

impl CardinalDirection {
    fn invert(&self) -> Self {
        match self {
            CardinalDirection::N => CardinalDirection::S,
            CardinalDirection::E => CardinalDirection::W,
            CardinalDirection::S => CardinalDirection::N,
            CardinalDirection::W => CardinalDirection::E,
        }
    }
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

impl From<&CardinalDirection> for CardinalDirectionFlags {
    fn from(value: &CardinalDirection) -> Self {
        match value {
            CardinalDirection::N => CardinalDirectionFlags::N,
            CardinalDirection::E => CardinalDirectionFlags::E,
            CardinalDirection::S => CardinalDirectionFlags::S,
            CardinalDirection::W => CardinalDirectionFlags::W,
        }
    }
}

#[derive(GodotClass, Debug)]
#[class(init, base=Node2D)]
pub struct TileCollisionAreas {
    base: Base<Node2D>,
}

impl TileCollisionAreas {
    fn get_collision_outlines(&self) -> [Gd<CollisionOutline>; 4] {
        let base = self.base();

        [
            base.get_node_as::<CollisionOutline>("./CollisionN"),
            base.get_node_as::<CollisionOutline>("./CollisionE"),
            base.get_node_as::<CollisionOutline>("./CollisionS"),
            base.get_node_as::<CollisionOutline>("./CollisionW"),
        ]
    }
    fn get_collision_area_at_direction(
        &self,
        direction: &CardinalDirection,
    ) -> Gd<CollisionOutline> {
        let base = self.base();

        match direction {
            CardinalDirection::N => base.get_node_as::<CollisionOutline>("./CollisionN"),
            CardinalDirection::E => base.get_node_as::<CollisionOutline>("./CollisionE"),
            CardinalDirection::S => base.get_node_as::<CollisionOutline>("./CollisionS"),
            CardinalDirection::W => base.get_node_as::<CollisionOutline>("./CollisionW"),
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

    // TODO: Replace this with a global manager implemented in code
    active_collisions: Vec<InstanceId>,
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
        self.disable_all_collisions();

        let mut gd_tile_component = self.get_tile_component();
        let mut tile_component = gd_tile_component.bind_mut();

        tile_component.is_cross = tile_component_data.is_cross;
        tile_component.oasis_layout = tile_component_data.oasis_layout.clone();
        tile_component.treasure_layout = tile_component_data.treasure_layout.clone();
    }
    pub fn disable_all_collisions(&mut self) {
        for direction in DIRECTIONS.iter() {
            self.disable_collision_at_direction(direction);
        }
    }
    pub fn enable_all_collisions(&mut self) {
        for direction in DIRECTIONS.iter() {
            self.enable_collision_at_direction(direction);
        }
    }
    fn disable_collision_at_direction(&mut self, direction: &CardinalDirection) {
        let mut gd_collision_area = self.get_collision_area_at_direction(direction);
        let mut collision_area = gd_collision_area.bind_mut();

        collision_area.disable_collision();
    }
    fn enable_collision_at_direction(&mut self, direction: &CardinalDirection) {
        let mut gd_collision_area = self.get_collision_area_at_direction(direction);
        let mut collision_area = gd_collision_area.bind_mut();

        collision_area.enable_collision();
    }
    pub fn set_active(&mut self) {
        self.is_active = true;
    }
    pub fn place_at(&mut self, direction: CardinalDirection, position: Vector2) -> Vector2 {
        // TODO: A low of hacks here with enabling and disabling collisions instead of just setting
        // a sensible default state and flipping only the ones we need. Just prototyping
        self.enable_all_collisions();

        let (offset_x, offset_y) = match direction {
            // Tiles are 250px. Their default scale is 0.2. Therefore, the offset is 50px
            CardinalDirection::N => (-50, -50),
            CardinalDirection::E => (0, -50),
            CardinalDirection::S => (0, 0),
            CardinalDirection::W => (-50, 0),
        };

        let outside_connection = direction.invert();

        let mut gd_collision_area = self.get_collision_area_at_direction(&outside_connection);
        let mut collision_area = gd_collision_area.bind_mut();

        collision_area.disable_collision();

        self.is_active = false;

        position + Vector2::from_tuple((offset_x as f32, offset_y as f32))
    }
    pub fn get_collision_areas(&self) -> Gd<TileCollisionAreas> {
        self.base()
            .get_node_as::<TileCollisionAreas>("./Collisions")
    }
    pub fn get_collision_area_at_direction(
        &self,
        direction: &CardinalDirection,
    ) -> Gd<CollisionOutline> {
        self.base()
            .get_node_as::<TileCollisionAreas>("./Collisions")
            .bind()
            .get_collision_area_at_direction(direction)
    }
}

#[godot_api]
impl Tile {
    // TODO: Replace this with a global manager implemented in code
    #[func]
    fn insert_active_collision(&mut self, id: InstanceId) {
        self.active_collisions.push(id);
    }
    // TODO: Replace this with a global manager implemented in code
    #[func]
    fn remove_active_collision(&mut self, id: InstanceId) {
        let index = self.active_collisions.iter().position(|inner| *inner == id);

        if let Some(index) = index {
            self.active_collisions.swap_remove(index);
        }
    }
}

#[godot_api]
impl INode2D for Tile {
    fn ready(&mut self) {
        let mut outside_connections = CardinalDirectionFlags::empty();
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
                    outside_connections = CardinalDirectionFlags::all();
                } else {
                    tile_config = Some(
                        parsed_config.cross.get_side(&cross_id).unwrap()[self.cross_index as usize]
                            .clone(),
                    );

                    match cross_id.as_str() {
                        "cross_n" => {
                            if self.cross_index < 4 {
                                outside_connections =
                                    CardinalDirectionFlags::N | CardinalDirectionFlags::S;
                            } else {
                                outside_connections = CardinalDirectionFlags::S;
                            }
                        }
                        "cross_e" => {
                            if self.cross_index < 4 {
                                outside_connections =
                                    CardinalDirectionFlags::E | CardinalDirectionFlags::W;
                            } else {
                                outside_connections = CardinalDirectionFlags::W;
                            }
                        }
                        "cross_s" => {
                            if self.cross_index < 4 {
                                outside_connections =
                                    CardinalDirectionFlags::S | CardinalDirectionFlags::N;
                            } else {
                                outside_connections = CardinalDirectionFlags::N;
                            }
                        }
                        "cross_w" => {
                            if self.cross_index < 4 {
                                outside_connections =
                                    CardinalDirectionFlags::W | CardinalDirectionFlags::E;
                            } else {
                                outside_connections = CardinalDirectionFlags::E;
                            }
                        }
                        _ => godot_error!("Expected `cross_id` to be one of cross_[c, n, e, s, w]"),
                    }
                }
            }

            if let Some(tile_config) = tile_config {
                let tile_data = TileData::from(tile_config);

                let mut tile_components = gd_tile_components.bind_mut();
                tile_components.oasis_layout = tile_data.oasis_layout;

                let treasure_layout = tile_data.treasure_layout.iter().map(GString::from);

                tile_components.treasure_layout = Array::from_iter(treasure_layout);
                tile_components.is_cross = tile_data.is_cross;
            }
        }

        let gd_tile_components = self.get_tile_component();
        let tile_components = gd_tile_components.bind();

        self.show_desert_icon_if_not_cross(true);

        for oasis_idx in 0..3 {
            let flag = (tile_components.oasis_layout.bits() >> (oasis_idx * 4)) as u8;

            let directions: Vec<CardinalDirection> =
                CardinalDirectionFlags::from_bits_truncate(flag).into();

            for (direction_idx, direction) in DIRECTIONS.iter().enumerate() {
                let mut gd_treasure = self.get_treasure_at_direction(direction);

                if directions.contains(direction) {
                    let mut gd_path = self.get_path_at_direction(direction);

                    gd_path.set_default_color(Color::from_rgb(255., 255., 255.));
                    gd_treasure.set_visible(true);
                    self.show_desert_icon_if_not_cross(false);
                }

                let treasure_at_idx = tile_components
                    .treasure_layout
                    .get(direction_idx)
                    .unwrap_or(GString::from(""));

                let treasure_kind: TreasureKind = treasure_at_idx
                    .try_into()
                    .expect("Couldn't parse treasure_layout into");

                let mut treasure = gd_treasure.bind_mut();

                if treasure_kind != TreasureKind::None {
                    let mut sprite = treasure.get_sprite();
                    sprite.set_visible(true);

                    self.show_desert_icon_if_not_cross(false);
                }

                treasure.set_kind(treasure_kind);
            }
        }

        for connection in outside_connections.iter() {
            let directions: Vec<CardinalDirection> = connection.into();

            for direction in directions.iter() {
                self.disable_collision_at_direction(direction);
            }
        }
    }
    fn process(&mut self, _dt: f64) {
        if !self.is_active {
            return;
        }

        let input = Input::singleton();
        let pressed = input.is_mouse_button_pressed(MouseButton::LEFT);
        let mut target_position: Option<Vector2> = None;

        if pressed {
            if self.active_collisions.len() != 1 {
                godot_error!("Must have exactly 1 collision to place tile");
            } else {
                self.is_active = false;

                let collision_id = self.active_collisions[0];

                // TODO: This isn't safe, but we can potentially make it safe(r) _by implementing a manager_
                let collision: Gd<CollisionOutline> = Gd::from_instance_id(collision_id);
                let position = collision.get_global_position();
                let side: Vec<CardinalDirection> =
                    CardinalDirectionFlags::from_bits_truncate(collision.bind().side).into();

                target_position = Some(self.place_at(side[0].clone(), position));

                let mut base = self.base_mut();
                base.set_scale(Vector2::from_tuple((0.2, 0.2)))
            }
        }

        let mut base = self.base_mut();
        let mouse_position = base
            .get_viewport()
            .expect("Expected node to have a viewport")
            .get_mouse_position();

        base.set_position(target_position.unwrap_or(mouse_position));
    }
}
