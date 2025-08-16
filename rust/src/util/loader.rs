use godot::global::godot_print;
use std::collections::HashMap;
use thiserror::Error;
use toml::map::Map;

use const_format::concatcp;
use godot::classes::file_access::ModeFlags;
use godot::classes::{FileAccess, Node};
use godot::obj::{Base, Gd};
use godot::prelude::GodotClass;
use toml::de::Error as TomlError;
use toml::{Table, Value};

use crate::game::entities::tile::Tile;

static GAME_CONFIGS_ROOT: &str = "res://config/";

#[derive(Error, Debug)]
pub enum LoadTomlError {
    #[error("Error reading file {0}")]
    FileReadError(String),
    #[error("Error parsing toml")]
    ParseError(TomlError),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum GameConfig {
    Tileset,
}

pub struct ConfigBool(bool);
pub struct ConfigString(String);

impl ConfigBool {
    fn from_key_in_table(key: &str, table: &Table) -> Option<Self> {
        table.get(key).and_then(|v| ConfigBool::try_from(v).ok())
    }
    fn from_key_in_table_as_bool(key: &str, table: &Table) -> Option<bool> {
        ConfigBool::from_key_in_table(key, table).map(|b| b.into())
    }
}

impl From<ConfigBool> for bool {
    fn from(value: ConfigBool) -> Self {
        value.0
    }
}

impl ConfigString {
    fn from_key_in_table(key: &str, table: &Table) -> Option<Self> {
        table.get(key).and_then(|v| ConfigString::try_from(v).ok())
    }
    fn from_key_in_table_as_string(key: &str, table: &Table) -> Option<String> {
        ConfigString::from_key_in_table(key, table).map(|s| s.into())
    }
}

impl From<ConfigString> for String {
    fn from(value: ConfigString) -> Self {
        value.0
    }
}

impl TryFrom<&Value> for ConfigBool {
    type Error = &'static str;

    fn try_from(value: &Value) -> Result<Self, &'static str> {
        match value {
            Value::Boolean(value) => Ok(ConfigBool(*value)),
            _ => Err("Couldn't parse value as boolean"),
        }
    }
}

impl TryFrom<&Value> for ConfigString {
    type Error = &'static str;

    fn try_from(value: &Value) -> Result<Self, &'static str> {
        match value {
            Value::String(value) => Ok(ConfigString(value.clone())),
            _ => Err("Couldn't parse value as string"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrossConfigArray([TileConfig; 5]);

impl CrossConfigArray {
    fn from_key_in_table(key: &str, table: &Table) -> Option<Self> {
        let val = table.get(key);
        // .and_then(|v| CrossConfigArray::try_from(v).ok())
        godot_print!("VAL {val:?}");
        let cross_config_array = CrossConfigArray::try_from(val.unwrap()).ok();
        // godot_print!("CCA {cross_config_array:?}");
        None
    }
    fn from_key_in_table_as_array(key: &str, table: &Table) -> Option<[TileConfig; 5]> {
        CrossConfigArray::from_key_in_table(key, table).map(|b| b.into())
    }
}

impl From<CrossConfigArray> for [TileConfig; 5] {
    fn from(value: CrossConfigArray) -> Self {
        value.0
    }
}

impl TryFrom<&Vec<Value>> for CrossConfigArray {
    type Error = &'static str;

    fn try_from(value: &Vec<Value>) -> Result<Self, &'static str> {
        if value.len() != 5 {
            return Err("Could not parse value as array of 5 elements");
        }

        let mut array: Option<[TileConfig; 5]> = None;

        for (idx, tile_value) in value.iter().enumerate() {
            let tile_config = TileConfig::try_from(tile_value)?;

            if let &Some(mut array) = &array {
                array[idx] = tile_config.clone();
            } else {
                array = Some(std::array::from_fn(|_| tile_config.clone()))
            }
        }

        let array = array.ok_or("Error parsing array")?;

        Ok(CrossConfigArray(array))
    }
}

impl TryFrom<&Value> for CrossConfigArray {
    type Error = &'static str;

    fn try_from(value: &Value) -> Result<Self, &'static str> {
        match value {
            Value::Array(value) => {
                godot_print!("CCA {value:?}");
                let array: Result<CrossConfigArray, &'static str> =
                    value.try_into().or(Err("Couldn't parse value as array"));

                // array
                Err("Test")
            }
            _ => Err("Couldn't parse value as array"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TileConfig {
    is_desert: Option<bool>,
    n: Option<bool>,
    e: Option<bool>,
    s: Option<bool>,
    w: Option<bool>,
    treasure_n: Option<String>,
    treasure_e: Option<String>,
    treasure_s: Option<String>,
    treasure_w: Option<String>,
}

impl TryFrom<&Value> for TileConfig {
    type Error = &'static str;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Table(table) => {
                let is_desert = ConfigBool::from_key_in_table_as_bool("is_desert", table);
                let n = ConfigBool::from_key_in_table_as_bool("n", table);
                let e = ConfigBool::from_key_in_table_as_bool("e", table);
                let s = ConfigBool::from_key_in_table_as_bool("s", table);
                let w = ConfigBool::from_key_in_table_as_bool("w", table);
                let treasure_n = ConfigString::from_key_in_table_as_string("treasure_n", table);
                let treasure_e = ConfigString::from_key_in_table_as_string("treasure_e", table);
                let treasure_s = ConfigString::from_key_in_table_as_string("treasure_s", table);
                let treasure_w = ConfigString::from_key_in_table_as_string("treasure_w", table);

                Ok(Self {
                    is_desert,
                    n,
                    e,
                    s,
                    w,
                    treasure_n,
                    treasure_e,
                    treasure_s,
                    treasure_w,
                })
            }
            _ => Err("Could not parse value as tile config table"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrossConfig {
    n: [TileConfig; 5],
    e: [TileConfig; 5],
    s: [TileConfig; 5],
    w: [TileConfig; 5],
}

impl TryFrom<&Option<&Value>> for CrossConfig {
    type Error = &'static str;

    fn try_from(value: &Option<&Value>) -> Result<Self, &'static str> {
        if let Some(value) = value {
            if let Value::Table(table) = value {
                let n = CrossConfigArray::from_key_in_table_as_array("n", table)
                    .ok_or("Missing entry in cross config")?;
                let e = CrossConfigArray::from_key_in_table_as_array("e", table)
                    .ok_or("Missing entry in cross config")?;
                let s = CrossConfigArray::from_key_in_table_as_array("s", table)
                    .ok_or("Missing entry in cross config")?;
                let w = CrossConfigArray::from_key_in_table_as_array("w", table)
                    .ok_or("Missing entry in cross config")?;

                Ok(Self { n, e, s, w })
            } else {
                Err("Cross config was not a table")
            }
        } else {
            Err("Could not find cross config")
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct DeckConfig([TileConfig; 17]);
//
// impl From<&Table> for DeckConfig {
//     fn from(table: &Table) -> Self {
//         todo!();
//     }
// }
//
// pub struct DeckConfigArray([DeckConfig; 5]);
//
// impl From<&Table> for DeckConfigArray {
//     fn from(table: &Table) -> Self {
//         todo!();
//     }
// }

#[derive(Debug, Clone)]
pub struct TilesetConfig {
    cross: CrossConfig,
    // deck: [DeckConfig; 5],
}

impl TryFrom<&Table> for TilesetConfig {
    type Error = &'static str;

    fn try_from(table: &Table) -> Result<Self, Self::Error> {
        let cross = CrossConfig::try_from(&table.get("cross"))?;
        // let deck = DeckConfigArray::from(table).0;

        Ok(Self {
            cross,
            // deck
        })
    }
}

#[derive(GodotClass, Debug)]
#[class(init, base=Node)]
pub struct TomlLoader {
    base: Base<Node>,
    configs: HashMap<GameConfig, Table>,
}

impl TomlLoader {
    pub fn get(node: Gd<Node>, config: GameConfig) -> Option<Map<String, Value>> {
        let tree = node
            .get_tree()
            .expect("Expected node to be part of a scene tree");
        let root = tree.get_root().expect("Expected scene tree to have a root");

        let mut gd_loader = root.get_node_as::<TomlLoader>("./GlobalTomlLoader");
        let mut loader = gd_loader.bind_mut();

        let table = loader.configs.get(&config);

        if let Some(table) = table {
            Some(table.clone())
        } else {
            loader.load(config).ok()
        }
    }
    fn load(&mut self, config: GameConfig) -> Result<Table, LoadTomlError> {
        let table_path = match config {
            GameConfig::Tileset => concatcp!(GAME_CONFIGS_ROOT, "tileset.toml"),
        };

        let raw_data = String::from(
            FileAccess::open(table_path, ModeFlags::READ)
                .ok_or(LoadTomlError::FileReadError(table_path.to_owned()))?
                .get_as_text(),
        );

        let table = toml::from_str::<Table>(&raw_data).map_err(LoadTomlError::ParseError)?;

        self.configs.insert(GameConfig::Tileset, table.clone());

        Ok(table)
    }
}
