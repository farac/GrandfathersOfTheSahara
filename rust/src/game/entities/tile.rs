use crate::game::components::{
    hover_outline::CollisionOutline, tile_component::TileComponent, tile_component::TileData,
};
use crate::game::entities::treasure::{Treasure, TreasureKind};
use crate::game::entities::{BoardComponent, Entity, EntityManager, EntityScope};
use crate::util::flags::{
    CardinalDirection, CardinalDirectionFlags, DIRECTIONS, OASIS_CONNECTION_LABELS,
};
use crate::util::input::InputActions;
use crate::util::loader::{GameConfig, TileConfig, TilesetConfig, TomlLoader, CROSS_IDS};
use crate::util::Logger;
use godot::builtin::{Array, Color, GString, Vector2};

use godot::classes::{INode2D, Input, Line2D, Node2D};
use godot::global::godot_error;
use godot::obj::{Gd, InstanceId, WithBaseField};
use godot::{
    classes::Sprite2D,
    obj::Base,
    prelude::{godot_api, GodotClass},
};

#[derive(GodotClass, Debug)]
#[class(init, base=Node2D)]
pub struct TileCollisionAreas {
    base: Base<Node2D>,
}

fn calculate_direction_offset_for_side(direction: &CardinalDirection) -> (i32, i32) {
    match direction {
        CardinalDirection::N => (0, 1),
        CardinalDirection::E => (1, 0),
        CardinalDirection::S => (0, -1),
        CardinalDirection::W => (-1, 0),
    }
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

    #[init(val = 0)]
    id: u64,

    #[init(val = false)]
    is_active: bool,

    #[export]
    #[init(val = GString::from(""))]
    cross_id: GString,
    #[export]
    #[init(val = 0)]
    cross_index: u8,
    #[init(val = 0)]
    outside_connections: u8,

    #[init(val = vec![])]
    // TODO: Replace this with a global manager implemented in code
    active_collisions: Vec<InstanceId>,
    #[init(val = 0.)]
    throttle_wheel: f64,
}

impl Tile {
    fn hide_all_connections(&self) {
        for connection in OASIS_CONNECTION_LABELS {
            let mut connection = self.get_connection_at(connection);

            connection.set_visible(false);
        }
    }
    fn get_connection_at(&self, id: &str) -> Gd<Node2D> {
        self.base()
            .get_node_as::<Node2D>(&format!("./Connections/{}", id))
    }
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
        for direction in DIRECTIONS {
            self.disable_collision_at_direction(&direction);
        }
    }
    pub fn enable_all_collisions(&mut self) {
        for direction in DIRECTIONS {
            self.enable_collision_at_direction(&direction);
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
        self.disable_all_collisions();
    }
    pub fn place_at(
        &mut self,
        direction: CardinalDirection,
        position: Vector2,
        coordinates: (u8, u8),
        adjacent_tiles: Vec<Option<Gd<Tile>>>,
    ) -> Vector2 {
        // TODO: A lot of hacks here with enabling and disabling collisions instead of just setting
        // a sensible default state and flipping only the ones we need. Just prototyping
        self.enable_all_collisions();

        let mut gd_board_component = BoardComponent::get(&self.base());
        let mut board_component = gd_board_component.bind_mut();

        if let Err(error) = board_component.add_tile_at(self.id, coordinates.0, coordinates.1) {
            godot_error!("{error:?}");
        };

        let (offset_x, offset_y) = match direction {
            // Tiles are 250px. Their default scale is 0.2 * 0.9. Therefore, the offset is 45px
            CardinalDirection::N => (-45, -46),
            CardinalDirection::E => (1, -45),
            CardinalDirection::S => (0, 1),
            CardinalDirection::W => (-46, 0),
        };

        let outside_connection = direction.invert();

        let mut gd_collision_area = self.get_collision_area_at_direction(&outside_connection);
        let mut collision_area = gd_collision_area.bind_mut();

        collision_area.disable_collision();

        // TODO: Disable collisions on adjacent tiles
        for _tile in adjacent_tiles {}

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
    pub fn refresh_display_state(&mut self) {
        let gd_tile_components = self.get_tile_component();
        let tile_components = gd_tile_components.bind();

        let mut enable_oasis: CardinalDirectionFlags = CardinalDirectionFlags::empty();

        let mut connections: Vec<&str> = vec![];

        for oasis_idx in 0..4 {
            let flag = (tile_components.oasis_layout.bits() >> (oasis_idx * 4)) as u8;
            let direction_flags = CardinalDirectionFlags::from_bits_truncate(flag);

            let directions: Vec<CardinalDirection> = direction_flags.clone().into();

            let oasis_connections: Vec<&str> = direction_flags.into();

            for connection in oasis_connections {
                connections.push(connection);
            }

            for (direction_idx, direction) in DIRECTIONS.iter().enumerate() {
                if directions.contains(direction) {
                    enable_oasis |= CardinalDirectionFlags::from(direction);
                }

                let mut gd_treasure = self.get_treasure_at_direction(direction);

                let treasure_at_idx = tile_components
                    .treasure_layout
                    .get(direction_idx)
                    .unwrap_or(GString::from(""));

                let treasure_kind: TreasureKind = treasure_at_idx
                    .try_into()
                    .expect("Couldn't parse treasure_layout into");

                let mut treasure = gd_treasure.bind_mut();

                if treasure_kind != TreasureKind::None {
                    let sprite = &mut treasure.get_sprites()[0];
                    sprite.set_visible(true);

                    self.show_desert_icon_if_not_cross(false);
                }

                treasure.kind = treasure_kind;
            }
        }

        self.hide_all_connections();

        for connection in connections {
            let mut connection = self.get_connection_at(connection);

            connection.set_visible(true);
        }

        let disable_oasis: CardinalDirectionFlags = enable_oasis.clone().complement();

        let enable_oasis: Vec<CardinalDirection> = enable_oasis.into();
        let disable_oasis: Vec<CardinalDirection> = disable_oasis.into();

        for direction in enable_oasis {
            let mut gd_path = self.get_path_at_direction(&direction);
            let mut gd_treasure = self.get_treasure_at_direction(&direction);

            gd_path.set_default_color(Color::from_rgb(255., 255., 255.));
            gd_treasure.set_visible(true);
            self.show_desert_icon_if_not_cross(false);
        }

        for direction in disable_oasis {
            let mut gd_path = self.get_path_at_direction(&direction);
            let mut gd_treasure = self.get_treasure_at_direction(&direction);

            gd_path.set_default_color(Color::from_html("#883f12").unwrap());
            gd_treasure.set_visible(false);
        }

        for connection in CardinalDirectionFlags::from_bits_truncate(self.outside_connections) {
            let directions: Vec<CardinalDirection> = connection.into();

            for direction in directions {
                self.disable_collision_at_direction(&direction);
            }
        }
    }
}

impl Entity for Tile {
    fn register(&mut self) {
        let gd_base = self.base();
        let instance_id = gd_base.instance_id();
        let mut entity_manager = EntityManager::get_manager(&gd_base);

        let id = entity_manager
            .bind_mut()
            .register(instance_id, EntityScope::Running);

        self.id = id;
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
        self.register();

        let is_cross_tile: bool;

        {
            let mut gd_tile_components = self.get_tile_component();

            let cross_id = self.cross_id.to_string();
            is_cross_tile = !cross_id.is_empty() && CROSS_IDS.contains(&cross_id.as_str());

            let mut tile_config: Option<TileConfig> = None;

            if is_cross_tile {
                let tileset = TomlLoader::get(&self.base(), GameConfig::Tileset)
                    .expect("Couldn't load tileset. Check if config/tileset.toml exists");

                let parsed_config = TilesetConfig::try_from(&tileset)
                    .expect("Couldn't parse tileset. Check syntax of config/tileset.toml");

                if &cross_id == "cross_c" {
                    tile_config = Some(parsed_config.cross.get_center());
                    self.outside_connections = CardinalDirectionFlags::all().bits();
                } else {
                    tile_config = Some(
                        parsed_config.cross.get_side(&cross_id).unwrap()[self.cross_index as usize]
                            .clone(),
                    );

                    match cross_id.as_str() {
                        "cross_n" => {
                            if self.cross_index < 4 {
                                self.outside_connections =
                                    (CardinalDirectionFlags::N | CardinalDirectionFlags::S).bits();
                            } else {
                                self.outside_connections = CardinalDirectionFlags::S.bits();
                            }
                        }
                        "cross_e" => {
                            if self.cross_index < 4 {
                                self.outside_connections =
                                    (CardinalDirectionFlags::E | CardinalDirectionFlags::W).bits();
                            } else {
                                self.outside_connections = (CardinalDirectionFlags::W).bits();
                            }
                        }
                        "cross_s" => {
                            if self.cross_index < 4 {
                                self.outside_connections =
                                    (CardinalDirectionFlags::S | CardinalDirectionFlags::N).bits();
                            } else {
                                self.outside_connections = (CardinalDirectionFlags::N).bits();
                            }
                        }
                        "cross_w" => {
                            if self.cross_index < 4 {
                                self.outside_connections =
                                    (CardinalDirectionFlags::W | CardinalDirectionFlags::E).bits();
                            } else {
                                self.outside_connections = (CardinalDirectionFlags::E).bits();
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

        self.show_desert_icon_if_not_cross(true);

        self.refresh_display_state();

        let collision_areas = self.get_collision_areas();

        collision_areas
            .bind()
            .get_collision_outlines()
            .iter()
            .enumerate()
            .for_each(|(i, o)| {
                o.signals().submitted_at().connect_other(self, move |tile| {
                    tile.disable_collision_at_direction(&DIRECTIONS[i])
                });
            });

        if is_cross_tile {
            let mut board_component = BoardComponent::get(&self.base());

            let (x, y) = match self.cross_id.to_string().as_str() {
                "cross_c" => (5, 5),
                "cross_n" => (5, self.cross_index + 6),
                "cross_e" => (self.cross_index + 6, 5),
                "cross_s" => (5, 4 - self.cross_index),
                "cross_w" => (4 - self.cross_index, 5),
                _ => {
                    godot_error!("Expected `cross_id` to be one of cross_[c, n, e, s, w]");
                    return;
                }
            };

            let mut board_component = board_component.bind_mut();

            if let Err(error) = board_component.add_tile_at(self.id, x, y) {
                godot_error!("{error:?}");
            }
        }
    }
    fn process(&mut self, dt: f64) {
        if !self.is_active {
            return;
        }

        let input = Input::singleton();
        let pressed = input.is_action_just_pressed(&String::from(InputActions::Primary));
        let mut target_position: Option<Vector2> = None;

        let wheel_up = input.is_action_just_released(&String::from(InputActions::RotateCcw));
        let wheel_down = input.is_action_just_released(&String::from(InputActions::RotateCw));

        if self.throttle_wheel == 0. && (wheel_up || wheel_down) {
            self.throttle_wheel += dt * 1000.;

            {
                let mut gd_tile_component = self.get_tile_component();
                let mut tile_component = gd_tile_component.bind_mut();

                if wheel_up {
                    tile_component.rotate_ccw();
                } else {
                    tile_component.rotate_cw();
                }
            }

            self.refresh_display_state();
        } else if self.throttle_wheel >= 256. {
            self.throttle_wheel = 0.;
        } else if self.throttle_wheel > 0. {
            self.throttle_wheel += dt * 1000.;
        }

        let collision_id = self.active_collisions.first();

        if let Some(collision_id) = collision_id {
            // TODO: This isn't safe, but we can potentially make it safe(r) _by implementing a manager_
            let mut collision: Gd<CollisionOutline> = Gd::from_instance_id(*collision_id);

            let collision_side: Vec<CardinalDirection> =
                CardinalDirectionFlags::from_bits_truncate(collision.bind().side).into();

            let mut placement_is_legal = None;
            let mut placement_coordinates: (i32, i32) = (0, 0);

            let mut adjacent_tiles: Vec<Option<Gd<Tile>>> = vec![];
            let adjacent_direction_offset = calculate_direction_offset_for_side(&collision_side[0]);

            'placement_calculation: {
                let mut collision = collision.bind_mut();
                let gd_collided_tile = collision.get_tile();
                let collided_tile = gd_collided_tile.bind();

                let gd_board_component = BoardComponent::get(&self.base());
                let board_component = gd_board_component.bind();

                match board_component.get_tile_coordinates(collided_tile.id) {
                    Ok((x, y)) => {
                        let x = x as i32;
                        let y = y as i32;
                        placement_coordinates = (
                            x + adjacent_direction_offset.0,
                            y + adjacent_direction_offset.1,
                        );
                    }
                    Err(error) => {
                        Logger::error(&format!("{error:?}"));
                    }
                }

                if placement_coordinates.0 > 10
                    || placement_coordinates.1 > 10
                    || placement_coordinates.0 < 0
                    || placement_coordinates.1 < 0
                {
                    placement_is_legal = Some(false);

                    break 'placement_calculation;
                }

                let mut coordinate_offset = CardinalDirection::N.get_coordinate_offset();
                let mut adjacent_coordinates = (
                    (placement_coordinates.0 + coordinate_offset.0) as u8,
                    (placement_coordinates.1 + coordinate_offset.1) as u8,
                );

                if let Ok(t) =
                    board_component.get_tile_at(adjacent_coordinates.0, adjacent_coordinates.1)
                {
                    adjacent_tiles.push(Some(t))
                } else {
                    adjacent_tiles.push(None)
                }

                coordinate_offset = CardinalDirection::E.get_coordinate_offset();
                adjacent_coordinates = (
                    (placement_coordinates.0 + coordinate_offset.0) as u8,
                    (placement_coordinates.1 + coordinate_offset.1) as u8,
                );

                if let Ok(t) =
                    board_component.get_tile_at(adjacent_coordinates.0, adjacent_coordinates.1)
                {
                    adjacent_tiles.push(Some(t))
                } else {
                    adjacent_tiles.push(None)
                }

                coordinate_offset = CardinalDirection::S.get_coordinate_offset();
                adjacent_coordinates = (
                    (placement_coordinates.0 + coordinate_offset.0) as u8,
                    (placement_coordinates.1 + coordinate_offset.1) as u8,
                );

                if let Ok(t) =
                    board_component.get_tile_at(adjacent_coordinates.0, adjacent_coordinates.1)
                {
                    adjacent_tiles.push(Some(t))
                } else {
                    adjacent_tiles.push(None)
                }

                coordinate_offset = CardinalDirection::W.get_coordinate_offset();
                adjacent_coordinates = (
                    (placement_coordinates.0 + coordinate_offset.0) as u8,
                    (placement_coordinates.1 + coordinate_offset.1) as u8,
                );

                if let Ok(t) =
                    board_component.get_tile_at(adjacent_coordinates.0, adjacent_coordinates.1)
                {
                    adjacent_tiles.push(Some(t))
                } else {
                    adjacent_tiles.push(None)
                }

                let gd_tile_component = self.get_tile_component();
                let this_tile_component = gd_tile_component.bind();
                let this_oasis_directions: Vec<CardinalDirection> =
                    CardinalDirectionFlags::from(this_tile_component.oasis_layout.clone()).into();

                for (idx, adjacent_tile) in adjacent_tiles.iter().enumerate() {
                    let direction = CardinalDirection::from(idx).invert();

                    let is_oasis = if let Some(adjacent_tile) = adjacent_tile {
                        let gd_adjacent_tile_component = adjacent_tile.bind().get_tile_component();
                        let adjacent_tile_component = gd_adjacent_tile_component.bind();

                        let adjacent_oasis_directions: Vec<CardinalDirection> =
                            CardinalDirectionFlags::from(
                                adjacent_tile_component.oasis_layout.clone(),
                            )
                            .into();

                        adjacent_oasis_directions.contains(&direction)
                    } else {
                        placement_is_legal = Some(placement_is_legal.unwrap_or(true));

                        continue;
                    };

                    placement_is_legal = Some(
                        placement_is_legal.unwrap_or(true)
                            && if is_oasis {
                                this_oasis_directions.contains(&direction.invert())
                            } else {
                                !this_oasis_directions.contains(&direction.invert())
                            },
                    )
                }

                if !placement_is_legal.unwrap_or(true) {
                    collision.forbid_outline();
                } else {
                    collision.allow_outline();
                }
            }

            if placement_is_legal.unwrap_or(true) && pressed {
                let placement_coordinates =
                    (placement_coordinates.0 as u8, placement_coordinates.1 as u8);

                if self.active_collisions.len() != 1 {
                    godot_error!("Must have exactly 1 collision to place tile");
                } else {
                    self.is_active = false;

                    let position = collision.get_global_position();

                    collision.signals().submitted_at().emit();

                    target_position = Some(self.place_at(
                        collision_side[0].clone(),
                        position,
                        placement_coordinates,
                        adjacent_tiles,
                    ));

                    let mut base = self.base_mut();
                    base.set_scale(Vector2::from_tuple((0.2 * 0.9, 0.2 * 0.9)))
                }

                self.refresh_display_state();
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
