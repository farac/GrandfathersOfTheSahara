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
                let array: Result<CrossConfigArray, &'static str> =
                    value.try_into().or(Err("Couldn't parse value as array"));

                array
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
    c: TileConfig,
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
                let center_value = value
                    .get("c")
                    .ok_or("Missing center entry in cross config")?;
                let c = TileConfig::try_from(center_value)
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

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use toml::Table;

    use crate::util::loader::{CrossConfig, TileConfig, TilesetConfig};

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
[[deck]]
[[deck]]
[[deck]]
[[deck]]
[[deck]]
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
