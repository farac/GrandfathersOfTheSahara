use std::collections::HashMap;
use std::num::TryFromIntError;

use godot::{obj::WithBaseField, prelude::godot_api};
use thiserror::Error;

use crate::{game::entities::tile::Tile, util::Logger};
use godot::{
    classes::Node,
    obj::{Base, Gd, InstanceId},
    prelude::GodotClass,
};

use crate::util::RootWindow;

pub mod deck;
pub mod tile;
pub mod treasure;

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
    // #[init(val=vec![])]
    // global: Vec<u64>,
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
                Logger::debug(format!(
                    "Registered instance id {instance_id:?} with entity id {}",
                    self.running.len()
                ));
                self.running.len() as u64
            }
        }
    }
    fn get(&self, id: u64, scope: EntityScope) -> Option<InstanceId> {
        match scope {
            EntityScope::Global => todo!(),
            EntityScope::Running => {
                Logger::debug(format!(
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
    fn remove(&mut self, id: u64, scope: EntityScope) -> Result<(), &'static str> {
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
    fn clear(&mut self, scope: EntityScope) {
        match scope {
            EntityScope::Global => todo!(),
            EntityScope::Running => self.running.clear(),
        }
    }
    fn free(&mut self, scope: EntityScope) {
        match scope {
            EntityScope::Global => todo!(),
            EntityScope::Running => {
                self.clear(EntityScope::Running);
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
    #[error("Tile does not exist at requested position - x:{0}, y:{1}")]
    TileCoordinateNotFoundError(u8, u8),
    #[error("Tile does not exist with requested id: {0}")]
    TileIdNotFoundError(u64),
    #[error("Tile instance was not found with requested id: {0}\nThis is an entity manager ID; not a Godot built-in Instance Id")]
    TileInstanceNotFoundError(i64),
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
    pub fn add_tile_at(&mut self, id: u64, x: u8, y: u8) -> Result<(), TileAddError> {
        if self.placed_tiles[x as usize][y as usize] != 0 {
            return Err(TileAddError::TileExistsError(x, y));
        }

        self.placed_tiles[x as usize][y as usize] = id;
        self.tile_coordinates.insert(id, (x as usize, y as usize));

        Logger::debug(format!("Placed tile {id} at {x}, {y}"));

        Ok(())
    }
    pub fn get_tile_at(&self, x: u8, y: u8) -> Result<Gd<Tile>, TileGetError> {
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
}
