use std::collections::HashMap;
use std::num::TryFromIntError;

use godot::classes::Node;
use godot::obj::{Gd, InstanceId};
use godot::prelude::GodotClass;
use godot::{obj::Base, prelude::godot_api};
use thiserror::Error;

use crate::game::entities::tile::Tile;

#[derive(Error, Debug)]
pub enum TileAddError {
    #[error("Tile already exists at attempted position - x:{0}, y:{1}")]
    TileExistsError(u8, u8),
    #[error("{0}")]
    IntegerConversionError(#[from] TryFromIntError),
}

#[derive(Error, Debug)]
pub enum TileGetError {
    #[error("Tile does not exist at requested position - x:{0}, y:{1}")]
    TileCoordinateNotFoundError(u8, u8),
    #[error("Tile does not exist with request id: {0}")]
    TileIdNotFoundError(i64),
    #[error("{0}")]
    IntegerConversionError(#[from] TryFromIntError),
}

#[derive(GodotClass, Debug)]
#[class(init,base=Node)]
pub struct BoardComponent {
    base: Base<Node>,

    #[init(val = [[0; 11]; 11])]
    placed_tiles: [[u64; 11]; 11],
    #[init(val=HashMap::new())]
    tile_coordinates: HashMap<u64, (usize, usize)>,
}

// #[godot_api]
// impl INode for BoardComponent {
//     fn process(&mut self, _dt: f64) {}
// }

#[godot_api]
impl BoardComponent {
    pub fn get(node: &Node) -> Gd<BoardComponent> {
        let tree = node
            .get_tree()
            .expect("Expected node to be part of a scene tree");
        let root = tree.get_root().expect("Expected scene tree to have a root");

        root.get_node_as::<BoardComponent>("./GlobalBoardComponent")
    }
    pub fn add_tile_at(&mut self, id: InstanceId, x: u8, y: u8) -> Result<(), TileAddError> {
        if self.placed_tiles[x as usize][y as usize] != 0 {
            return Err(TileAddError::TileExistsError(x, y));
        }

        let id = id.to_i64().try_into()?;

        self.placed_tiles[x as usize][y as usize] = id;
        self.tile_coordinates.insert(id, (x as usize, y as usize));

        Ok(())
    }
    pub fn get_tile_at(&self, x: u8, y: u8) -> Result<Gd<Tile>, TileGetError> {
        let id = self.placed_tiles[x as usize][y as usize];

        if id == 0 {
            return Err(TileGetError::TileCoordinateNotFoundError(x, y));
        }

        let id = InstanceId::from_i64(id.try_into()?);
        // TODO: This isn't safe, but we can potentially make it safe(r) _by implementing a manager_
        Ok(Gd::from_instance_id(id))
    }
    pub fn get_tile_coordinates(&self, id: InstanceId) -> Result<(u8, u8), TileGetError> {
        let (x, y) = self
            .tile_coordinates
            .get(&id.to_i64().try_into()?)
            .ok_or(TileGetError::TileIdNotFoundError(id.to_i64()))?;

        Ok(((*x).try_into()?, (*y).try_into()?))
    }
}
