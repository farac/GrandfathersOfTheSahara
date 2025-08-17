use std::collections::HashMap;
use thiserror::Error;
use toml::map::Map;

use const_format::concatcp;
use godot::classes::file_access::ModeFlags;
use godot::classes::{FileAccess, INode, Node, PackedScene};
use godot::obj::{Base, Gd};
use godot::prelude::load;
use godot::prelude::{godot_api, GodotClass};
use toml::de::Error as TomlError;
use toml::{Table, Value};

use crate::game::components::tile_component::TileComponent;
use crate::game::entities::tile::Tile;

const GAME_CONFIGS_ROOT: &str = "res://config/";
const GAME_OBJECTS_ROOT: &str = "res://game/objects/";

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
        let val = table
            .get(key)
            .and_then(|v| CrossConfigArray::try_from(v).ok());

        val
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

        let tile_configs: Vec<TileConfig> = value
            .iter()
            .map(TileConfig::try_from)
            .collect::<Result<Vec<TileConfig>, &str>>()?;

        let array = std::array::from_fn(|idx| tile_configs[idx].clone());

        Ok(CrossConfigArray(array))
    }
}

impl TryFrom<&Value> for CrossConfigArray {
    type Error = &'static str;

    fn try_from(value: &Value) -> Result<Self, &'static str> {
        match value {
            Value::Array(value) => {
                let tile_configs: Vec<TileConfig> = value
                    .iter()
                    .map(TileConfig::try_from_for_cross)
                    .collect::<Result<Vec<TileConfig>, &'static str>>()?;

                let array: [TileConfig; 5] = tile_configs.try_into().or(Err(
                    "Couldn't coerce TileConfig for center cross into array of 5 tiles",
                ))?;

                Ok(CrossConfigArray(array))
            }
            _ => Err("Couldn't parse value as array"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TileConfig {
    pub is_cross: Option<bool>,
    pub is_desert: Option<bool>,
    pub n: Option<bool>,
    pub e: Option<bool>,
    pub s: Option<bool>,
    pub w: Option<bool>,
    pub treasure_n: Option<String>,
    pub treasure_e: Option<String>,
    pub treasure_s: Option<String>,
    pub treasure_w: Option<String>,
}

impl TileConfig {
    fn try_from_for_cross(value: &Value) -> Result<Self, &'static str> {
        let mut base = Self::try_from(value)?;

        base.is_cross = Some(true);

        Ok(base)
    }
}

impl TryFrom<&Value> for TileConfig {
    type Error = &'static str;

    fn try_from(value: &Value) -> Result<Self, &'static str> {
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
                    is_cross: Some(false),
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
    c: TileConfig,
    n: [TileConfig; 5],
    e: [TileConfig; 5],
    s: [TileConfig; 5],
    w: [TileConfig; 5],
}

impl CrossConfig {
    pub fn get_side(&self, side: &str) -> Result<[TileConfig; 5], &'static str> {
        match side {
            "cross_n" => Ok(self.n.clone()),
            "cross_e" => Ok(self.e.clone()),
            "cross_s" => Ok(self.s.clone()),
            "cross_w" => Ok(self.w.clone()),
            _ => Err("Requested invalid side for cross config"),
        }
    }
    pub fn get_center(&self) -> TileConfig {
        self.c.clone()
    }
}

impl TryFrom<&Option<&Value>> for CrossConfig {
    type Error = &'static str;

    fn try_from(value: &Option<&Value>) -> Result<Self, &'static str> {
        if let Some(value) = value {
            if let Value::Table(table) = value {
                let center_value = value
                    .get("c")
                    .ok_or("Missing center entry in cross config")?;
                let c = TileConfig::try_from_for_cross(center_value)
                    .map_err(|_| "Missing center entry in cross config")?;
                let n = CrossConfigArray::from_key_in_table_as_array("n", table)
                    .ok_or("Missing north entry in cross config")?;
                let e = CrossConfigArray::from_key_in_table_as_array("e", table)
                    .ok_or("Missing east entry in cross config")?;
                let s = CrossConfigArray::from_key_in_table_as_array("s", table)
                    .ok_or("Missing south entry in cross config")?;
                let w = CrossConfigArray::from_key_in_table_as_array("w", table)
                    .ok_or("Missing west entry in cross config")?;

                Ok(Self { c, n, e, s, w })
            } else {
                Err("Cross config was not a table")
            }
        } else {
            Err("Could not find cross config")
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeckConfig(pub [TileConfig; 17]);

impl TryFrom<&Value> for DeckConfig {
    type Error = &'static str;

    fn try_from(value: &Value) -> Result<Self, &'static str> {
        match value {
            Value::Array(value) => {
                let value: Vec<Result<TileConfig, &str>> =
                    value.iter().map(TileConfig::try_from).collect();

                if value.len() != 17 {
                    return Err("Deck requires exactly 17 tiles");
                }

                let array: [TileConfig; 17] =
                    std::array::from_fn(|idx| value[idx].clone().unwrap());

                Ok(DeckConfig(array))
            }
            _ => Err("Couldn't parse value as array"),
        }
    }
}

pub struct DeckConfigArray([DeckConfig; 5]);

impl TryFrom<&Table> for DeckConfigArray {
    type Error = &'static str;

    fn try_from(table: &Table) -> Result<Self, &'static str> {
        let value = table
            .get("decks")
            .ok_or("Could not find list of decks in config")?;

        match value {
            Value::Array(value) => {
                let vec_of_nested_decks: Result<Vec<&Value>, &'static str> = value
                    .iter()
                    .map(|t| {
                        t.get("deck")
                            .ok_or("Missing property 'deck' in decks array")
                    })
                    .collect();

                let value: Vec<Result<DeckConfig, &str>> = vec_of_nested_decks?
                    .iter()
                    .map(|e| DeckConfig::try_from(*e))
                    .collect();

                if value.len() != 5 {
                    return Err("Config requires exactly 5 tile decks");
                }

                let array: [DeckConfig; 5] = std::array::from_fn(|idx| value[idx].clone().unwrap());

                Ok(DeckConfigArray(array))
            }
            _ => Err("Couldn't parse value as array"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TilesetConfig {
    pub cross: CrossConfig,
    pub deck: [DeckConfig; 5],
}

impl TryFrom<&Table> for TilesetConfig {
    type Error = &'static str;

    fn try_from(table: &Table) -> Result<Self, Self::Error> {
        let cross = CrossConfig::try_from(&table.get("cross"))?;
        let deck = DeckConfigArray::try_from(table)?.0;

        Ok(Self { cross, deck })
    }
}

#[derive(GodotClass, Debug)]
#[class(init, base=Node)]
pub struct TomlLoader {
    base: Base<Node>,
    configs: HashMap<GameConfig, Table>,
}

impl TomlLoader {
    pub fn get(node: &Node, config: GameConfig) -> Option<Map<String, Value>> {
        let tree = node
            .get_tree()
            .expect("Expected node to be part of a scene tree");
        let root = tree.get_root().expect("Expected scene tree to have a root");

        {
            let gd_loader = root.get_node_as::<TomlLoader>("./GlobalTomlLoader");
            let loader = gd_loader.bind();

            let table = loader.configs.get(&config);

            if let Some(table) = table {
                return Some(table.clone());
            }
        }

        let mut gd_loader = root.get_node_as::<TomlLoader>("./GlobalTomlLoader");
        let mut loader = gd_loader.bind_mut();
        loader.load(config).ok()
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

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use toml::Table;

    use crate::util::loader::{CrossConfig, TilesetConfig};

    #[test]
    fn test_parse_tileset_config() {
        let input = String::from(
            "
# Configuration for the cross center
[cross.c]
is_desert = false

# 4 lists describing each part of the starting cross
# Read the directions like you would read a map
# Numbered going outwards from the center
# N1
[[cross.n]]
is_desert = false
w = true
# N2
[[cross.n]]
is_desert = false
e = true
# N3
[[cross.n]]
is_desert = false
w = true
# N4
[[cross.n]]
is_desert = false
e = true
# N5
[[cross.n]]
is_desert = false

# E1
[[cross.e]]
is_desert = false
# E2
[[cross.e]]
is_desert = false
s = true
# E3
[[cross.e]]
is_desert = false
n = true
# E4
[[cross.e]]
s = true
is_desert = false
# E5
[[cross.e]]
is_desert = false

# S1
[[cross.s]]
is_desert = false
# S2
[[cross.s]]
is_desert = false
# S3
[[cross.s]]
is_desert = false
e = true
w = true
# S4
[[cross.s]]
is_desert = false
# S5
[[cross.s]]
is_desert = false

# W1
[[cross.w]]
is_desert = false
# W2
[[cross.w]]
is_desert = false
n = true
s = true
# W3
[[cross.w]]
is_desert = false
# W4
[[cross.w]]
is_desert = false
n = true
s = true
# W5
[[cross.w]]
is_desert = false

# Lists describing each of the 'decks' used in the game
# Deck 1 
[[decks]]
# 1
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 2
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 3
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 4
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 5
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 6
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 7
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 8
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 9
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 10
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 11
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 12
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 13
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 14
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 15
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 16
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 17
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"


# Deck 2 
[[decks]]
# 1
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 2
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 3
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 4
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 5
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 6
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 7
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 8
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 9
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 10
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 11
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 12
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 13
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 14
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 15
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 16
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 17
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"


# Deck 3
[[decks]]
# 1
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 2
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 3
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 4
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 5
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 6
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 7
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 8
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 9
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 10
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 11
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 12
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 13
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 14
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 15
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 16
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 17
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"


# Deck 4
[[decks]]
# 1
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 2
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 3
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 4
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 5
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 6
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 7
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 8
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 9
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 10
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 11
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 12
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 13
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 14
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 15
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 16
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 17
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"


# Deck 5
[[decks]]
# 1
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 2
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 3
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 4
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 5
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 6
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 7
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 8
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 9
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 10
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 11
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 12
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 13
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 14
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 15
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 16
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"

# 17
[[decks.deck]]
is_desert = false
n = false
e = false
s = false
w = false
treasure_n = \"none\"
treasure_e = \"none\"
treasure_s = \"none\"
treasure_w = \"none\"
",
        );

        let table = toml::from_str::<Table>(&input).unwrap();

        let parsed_config = TilesetConfig::try_from(&table);

        println!("{parsed_config:?}");

        assert_matches!(parsed_config, Ok(_));

        let tileset_config = parsed_config.unwrap();

        assert_matches!(
            tileset_config,
            TilesetConfig {
                deck: [_, _, _, _, _],
                cross: CrossConfig { c, n, e:_, w, s:_ }
            } => {
                assert_eq!(c.is_desert, Some(false));
                assert_matches!(n, array => {
                    assert_eq!(array[0].is_desert, Some(false));
                    assert_matches!(array[0].n, Some(false) | None);
                    assert_matches!(array[0].e, Some(false) | None);
                    assert_matches!(array[0].s, Some(false) | None);
                    assert_eq!(array[0].w, Some(true));
                });

                assert_matches!(w, array => {
                    assert_eq!(array[1].is_desert, Some(false));
                    assert_eq!(array[1].n, Some(true));
                    assert_matches!(array[1].e, Some(false) | None);
                    assert_eq!(array[1].s, Some(true));
                    assert_matches!(array[1].w, Some(false) | None);
                });
            }
        );
    }
}

#[derive(GodotClass, Debug)]
#[class(init, base=Node)]
pub struct SceneLoader {
    base: Base<Node>,
    tile_scene: Gd<PackedScene>,
}

impl SceneLoader {
    pub fn get(node: &Node) -> Gd<SceneLoader> {
        let tree = node
            .get_tree()
            .expect("Expected node to be part of a scene tree");
        let root = tree.get_root().expect("Expected scene tree to have a root");

        root.get_node_as::<SceneLoader>("./GlobalSceneLoader")
    }
    pub fn instantiate_tile_scene_from_tile_component(
        &self,
        tile_component: &TileComponent,
    ) -> Gd<Tile> {
        let mut gd_tile = self.tile_scene.instantiate_as::<Tile>();

        {
            let mut tile = gd_tile.bind_mut();
            tile.set_from_tile_component(tile_component);
        }

        gd_tile
    }
}

#[godot_api]
impl INode for SceneLoader {
    fn ready(&mut self) {
        self.tile_scene = load(concatcp!(GAME_OBJECTS_ROOT, "tile.tscn"));
    }
}
