use std::collections::HashMap;
use std::collections::HashSet;
use std::num::TryFromIntError;

use godot::classes::INode;
use godot::classes::Node2D;
use godot::obj::WithBaseField;
use godot::obj::WithUserSignals;
use godot::prelude::godot_api;
use thiserror::Error;

use crate::game::entities::movement::BoardGraph;
use crate::game::entities::player::PlayerName;
use crate::game::entities::player_token::PlayerToken;
use crate::game::entities::tile::Tile;
use crate::game::entities::turn::TurnState;
use crate::game::RunningGameScene;
use crate::util::flags::CardinalDirectionFlags;
use crate::util::flags::DIRECTIONS;
use crate::util::Logger;
use godot::classes::Node;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::InstanceId;
use godot::prelude::GodotClass;

use crate::util::RootWindow;

pub mod deck;
pub mod movement;
pub mod player;
pub mod player_token;
pub mod tile;
pub mod treasure;
pub mod turn;

trait Entity
where
    Self: GodotClass,
{
    fn register(&mut self);
}

pub enum EntityScope {
    Global,
    Running,
}

#[derive(Debug, GodotClass)]
#[class(init, base=Node)]
pub struct EntityManager {
    base: Base<Node>,

    // Lists are scoped to enable freeing when the lifetime of a scope is over
    // E.g. on game exit/reset, the "running" scope can be cleared without worry
    #[init(val=vec![])]
    _global: Vec<u64>,
    #[init(val=vec![])]
    running: Vec<Option<InstanceId>>,
}

impl EntityManager {
    fn get_manager(node: &Node) -> Gd<EntityManager> {
        let root = node.get_tree_root();

        root.get_node_as("./GlobalEntityManager")
    }
    fn register(&mut self, instance_id: InstanceId, scope: EntityScope) -> u64 {
        match scope {
            EntityScope::Global => todo!(),
            EntityScope::Running => {
                self.running.push(Some(instance_id));
                Logger::debug(&format!(
                    "Registered instance id {instance_id:?} with entity id {}",
                    self.running.len()
                ));
                self.running.len() as u64
            }
        }
    }
    fn _get(&self, id: u64, scope: EntityScope) -> Option<InstanceId> {
        match scope {
            EntityScope::Global => todo!(),
            EntityScope::Running => {
                Logger::debug(&format!(
                    "Got instance id {:?} for entity id {id}",
                    self.running[(id - 1) as usize],
                ));

                self.running[(id - 1) as usize]
            }
        }
    }
    fn get_instance<T>(&self, id: u64, scope: EntityScope) -> Option<Gd<T>>
    where
        T: GodotClass,
    {
        let instance_id = match scope {
            EntityScope::Global => todo!(),
            EntityScope::Running => self.running[(id - 1) as usize],
        }?;

        Gd::try_from_instance_id(instance_id).ok()
    }
    fn _remove(&mut self, id: u64, scope: EntityScope) -> Result<(), &'static str> {
        match scope {
            EntityScope::Global => todo!(),
            EntityScope::Running => {
                if self.running[(id - 1) as usize].is_some() {
                    self.running[(id - 1) as usize] = None;

                    Ok(())
                } else {
                    Err("Attempted to remove Entity that is already set to None")
                }
            }
        }
    }
    fn _clear_scope(&mut self, scope: EntityScope) {
        match scope {
            EntityScope::Global => todo!(),
            EntityScope::Running => self.running.clear(),
        }
    }
    fn _free(&mut self, scope: EntityScope) {
        match scope {
            EntityScope::Global => todo!(),
            EntityScope::Running => {
                self._clear_scope(EntityScope::Running);
                self.running.shrink_to_fit()
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum TileAddError {
    #[error("Tile already exists at attempted position - x:{0}, y:{1}")]
    TileExistsError(u8, u8),
    #[error("{0}")]
    IntegerConversionError(#[from] TryFromIntError),
}

#[derive(Error, Debug)]
pub enum TileGetError {
    #[error("Requested coordinate is out of bounds")]
    TileCoordinateOutOfBoundsError,
    #[error("Tile does not exist at requested position - x:{0}, y:{1}")]
    TileCoordinateNotFoundError(u8, u8),
    #[error("Tile does not exist with requested id: {0}")]
    TileIdNotFoundError(u64),
    #[error("Tile instance was not found with requested id: {0}\nThis is an entity manager ID; not a Godot built-in Instance Id")]
    TileInstanceNotFoundError(i64),
    #[error("{0}")]
    IntegerConversionError(#[from] TryFromIntError),
}

// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// pub enum GameState {
//     Pre,
//     Running,
//     Post,
// }

#[derive(GodotClass, Debug)]
#[class(init,base=Node)]
pub struct BoardComponent {
    base: Base<Node>,

    #[init(val = [[0; 11]; 11])]
    placed_tiles: [[u64; 11]; 11],
    #[init(val=HashMap::new())]
    tile_coordinates: HashMap<u64, (usize, usize)>,
    active_tile_deck: u8,

    #[init(val = 4)]
    player_count: u8,
    turn: TurnState,

    #[init(val = HashMap::new())]
    player_positions: HashMap<PlayerName, (u8, u8)>,

    // A Tile queues these while it's borrowed by its own callback. Applying
    // the change now would borrow it again, so it runs later via `call_deferred`.
    #[init(val = None)]
    pending_move: Option<(u8, u8)>,
    #[init(val = None)]
    pending_placement: Option<((u8, u8), bool)>,
}

#[godot_api]
impl INode for BoardComponent {
    fn ready(&mut self) {
        self.signals()
            .tile_placed()
            .connect_self(Self::on_tile_placed);
    }
}

#[godot_api]
impl BoardComponent {
    pub fn get(node: &Node) -> Gd<BoardComponent> {
        let running_scene = RunningGameScene::get_running_game(node);

        running_scene.get_node_as::<BoardComponent>("./BoardComponent")
    }
    pub fn add_tile_at(&mut self, id: u64, x: u8, y: u8) -> Result<(), TileAddError> {
        if self.placed_tiles[x as usize][y as usize] != 0 {
            return Err(TileAddError::TileExistsError(x, y));
        }

        self.placed_tiles[x as usize][y as usize] = id;
        self.tile_coordinates.insert(id, (x as usize, y as usize));

        Logger::debug(&format!("Placed tile {id} at {x}, {y}"));

        Ok(())
    }
    pub fn get_tile_at(&self, x: u8, y: u8) -> Result<Gd<Tile>, TileGetError> {
        if x > 10 || y > 10 {
            return Err(TileGetError::TileCoordinateOutOfBoundsError);
        }

        let id = self.placed_tiles[x as usize][y as usize];

        if id == 0 {
            return Err(TileGetError::TileCoordinateNotFoundError(x, y));
        }

        EntityManager::get_manager(&self.base())
            .bind()
            .get_instance(id, EntityScope::Running)
            .ok_or(TileGetError::TileIdNotFoundError(id))
    }
    pub fn get_tile_coordinates(&self, id: u64) -> Result<(u8, u8), TileGetError> {
        let (x, y) = self
            .tile_coordinates
            .get(&id)
            .ok_or(TileGetError::TileIdNotFoundError(id))?;

        Ok(((*x).try_into()?, (*y).try_into()?))
    }
    pub fn set_player_position(&mut self, player: PlayerName, coordinates: (u8, u8)) {
        self.player_positions.insert(player, coordinates);
    }
    pub fn get_player_position(&self, player: PlayerName) -> Option<(u8, u8)> {
        self.player_positions.get(&player).copied()
    }
    pub fn active_player(&self) -> PlayerName {
        self.turn.active_player()
    }
    pub fn can_move(&self) -> bool {
        self.turn.can_move()
    }
    fn placed_coordinates(&self) -> Vec<(u8, u8)> {
        self.tile_coordinates
            .values()
            .map(|&(x, y)| (x as u8, y as u8))
            .collect()
    }
    fn build_board_graph(&self) -> BoardGraph {
        let mut graph = BoardGraph::default();

        for coordinates in self.placed_coordinates() {
            if let Ok(tile) = self.get_tile_at(coordinates.0, coordinates.1) {
                graph.insert_tile(coordinates, tile.bind().oasis_directions());
            }
        }

        graph
    }
    fn occupied_by_others(&self, active: PlayerName) -> HashSet<(u8, u8)> {
        self.player_positions
            .iter()
            .filter(|(player, _)| **player != active)
            .map(|(_, coordinates)| *coordinates)
            .collect()
    }
    /// Sides of the tile at `coordinates` a caravan may explore along. Sides
    /// facing the board border are excluded: no tile may be placed off-board.
    fn explorable_edges(&self, coordinates: (u8, u8)) -> CardinalDirectionFlags {
        let mut explorable = CardinalDirectionFlags::empty();

        for direction in DIRECTIONS {
            let (dx, dy) = direction.get_coordinate_offset();
            let x = coordinates.0 as i32 + dx;
            let y = coordinates.1 as i32 + dy;

            if x < 0 || y < 0 || x > 10 || y > 10 {
                continue;
            }

            if self.placed_tiles[x as usize][y as usize] == 0 {
                explorable |= CardinalDirectionFlags::from(&direction);
            }
        }

        explorable
    }
    /// Highlights every tile the active caravan may legally move to.
    pub fn enter_move_phase(&mut self) {
        let active = self.active_player();

        let Some(from) = self.get_player_position(active) else {
            return;
        };

        let graph = self.build_board_graph();
        let occupied = self.occupied_by_others(active);
        let reachable: HashSet<(u8, u8)> =
            graph.reachable_tiles(from, &occupied).into_iter().collect();

        Logger::info(&format!(
            "{active:?} to move from {from:?}: {} reachable tile(s)",
            reachable.len()
        ));
        Logger::debug(&format!("{active:?} reachable tiles: {reachable:?}"));

        for coordinates in self.placed_coordinates() {
            if let Ok(mut gd_tile) = self.get_tile_at(coordinates.0, coordinates.1) {
                let mut tile = gd_tile.bind_mut();

                tile.set_move_destination(reachable.contains(&coordinates));
                tile.set_explorable_edges(CardinalDirectionFlags::empty());
                tile.set_active_caravan(coordinates == from);
            }
        }
    }
    /// Clears movement highlights and confines tile placement to the
    /// explorable edges of the active caravan's current tile.
    pub fn enter_explore_phase(&mut self) {
        let active = self.active_player();
        let active_position = self.get_player_position(active);

        Logger::debug(&format!("{active:?} to explore from {active_position:?}"));

        for coordinates in self.placed_coordinates() {
            let explorable = self.explorable_edges(coordinates);

            if let Ok(mut gd_tile) = self.get_tile_at(coordinates.0, coordinates.1) {
                let mut tile = gd_tile.bind_mut();

                tile.set_move_destination(false);

                if Some(coordinates) == active_position {
                    tile.set_explorable_edges(explorable);
                } else {
                    tile.set_explorable_edges(CardinalDirectionFlags::empty());
                }

                tile.set_active_caravan(Some(coordinates) == active_position);
            }
        }
    }
    /// Moves the player's token onto the center of the tile at `coordinates`.
    fn move_token_to_tile(&self, player: PlayerName, coordinates: (u8, u8)) {
        let target = match self.get_tile_at(coordinates.0, coordinates.1) {
            Ok(tile) => tile.bind().center(),
            Err(_) => return,
        };

        let container = RunningGameScene::get_running_game(&self.base())
            .get_node_as::<Node2D>("./PlayerTokens");

        for child in container.get_children().iter_shared() {
            if let Ok(mut token) = child.try_cast::<PlayerToken>() {
                if token.bind().player == player {
                    token.set_global_position(target);
                    break;
                }
            }
        }
    }
    /// Snaps every token onto its tile. Runs deferred after the initial
    /// placement so the board layout is resolved and the pixel positions are
    /// correct.
    #[func]
    fn reposition_tokens(&mut self) {
        for (player, coordinates) in self.player_positions.clone() {
            self.move_token_to_tile(player, coordinates);
        }
    }
    fn advance_caravan_to(&mut self, player: PlayerName, coordinates: (u8, u8)) {
        self.set_player_position(player, coordinates);
        self.move_token_to_tile(player, coordinates);
    }
    fn move_active_player_to(&mut self, coordinates: (u8, u8)) {
        let player = self.active_player();

        Logger::info(&format!("{player:?} moved caravan to {coordinates:?}"));

        self.advance_caravan_to(player, coordinates);
        self.turn.advance_to_explore();
        self.enter_explore_phase();
    }
    /// Applies a queued caravan move.
    ///
    /// Godot invokes this by name (the `"apply_pending_move"` string) from
    /// [`Tile::try_move_here`] through `call_deferred`. Renaming the method
    /// without updating that string breaks the deferred call at runtime.
    #[func]
    fn apply_pending_move(&mut self) {
        if let Some(coordinates) = self.pending_move.take() {
            self.move_active_player_to(coordinates);
        }
    }
    /// Resolves the turn once a tile has been placed.
    ///
    /// Godot invokes this by name (the `"apply_pending_placement"` string)
    /// from [`Self::on_tile_placed`] through `call_deferred`. Renaming the
    /// method without updating that string breaks the deferred call at runtime.
    #[func]
    fn apply_pending_placement(&mut self) {
        let Some((coordinates, was_desert_tile)) = self.pending_placement.take() else {
            return;
        };

        // Rules: the caravan advances onto the tile it just explored, and a
        // desert tile grants the same player another move instead of passing.
        let player = self.active_player();

        Logger::info(&format!(
            "{player:?} explored {coordinates:?}{}",
            if was_desert_tile {
                " (desert: same player moves again)"
            } else {
                ""
            }
        ));

        self.advance_caravan_to(player, coordinates);
        self.turn.advance_turn(was_desert_tile);
        self.enter_move_phase();
    }
    /// Called when the active player draws a tile to explore. Uses up their
    /// move (if unused) and limits placement to their current tile's edges.
    pub fn begin_exploration(&mut self) {
        if self.turn.can_move() {
            self.turn.advance_to_explore();
        }

        self.enter_explore_phase();
    }
    pub fn queue_move(&mut self, coordinates: (u8, u8)) {
        self.pending_move = Some(coordinates);
    }
    pub fn queue_placement(&mut self, coordinates: (u8, u8), was_desert_tile: bool) {
        self.pending_placement = Some((coordinates, was_desert_tile));
    }
    /// Defers turn resolution to [`Self::apply_pending_placement`], which runs
    /// once the just-placed Tile is no longer borrowed.
    fn on_tile_placed(&mut self) {
        self.to_gd().call_deferred("apply_pending_placement", &[]);
    }
    #[signal]
    fn tile_placed();
}
