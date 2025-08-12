use std::{io::ErrorKind, str::FromStr};

use godot::classes::{ISprite2D, Image, ImageTexture, Sprite2D};
use godot::prelude::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TreasureKindParseError {
    #[error("TreasureKind expected {0} arguments, received {1}")]
    ArgumentCount(&'static str, usize),
    #[error("TreasureKind expected one of water, goods, camels, rumors, received {0}")]
    ParseTreasure(String),
    #[error("TreasureKinds expected goods to be one of , received {0}")]
    ParseGoods(String),
}

#[derive(Clone, Debug)]
enum Good {
    Incense,
    Myrrh,
    Salt,
    Gems,
}

#[derive(Clone, Debug)]
pub enum TreasureKind {
    Water,
    Goods(Good),
    Camels,
    Rumors,
    None,
}

impl TreasureKind {
    fn icon_texture(&self) -> Result<Gd<ImageTexture>, ErrorKind> {
        let treasure = self;

        let icon_name: &str = treasure.into();

        let path = format!("res://assets/icons/treasure/{icon_name}.png");

        let image = Image::load_from_file(&path);

        if let Some(image) = image {
            ImageTexture::create_from_image(&image).ok_or(ErrorKind::NotFound)
        } else {
            Err(ErrorKind::NotFound)
        }
    }
}

impl From<&TreasureKind> for &str {
    fn from(val: &TreasureKind) -> Self {
        match val {
            TreasureKind::Water => "water",
            TreasureKind::Camels => "camels",
            TreasureKind::Rumors => "rumors",
            TreasureKind::Goods(_) => {
                todo!();
            }
            TreasureKind::None => "none",
        }
    }
}

impl FromStr for TreasureKind {
    type Err = TreasureKindParseError;

    fn from_str(value: &str) -> Result<Self, TreasureKindParseError> {
        match value {
            "water" => Ok(TreasureKind::Water),
            "camels" => Ok(TreasureKind::Camels),
            "rumors" => Ok(TreasureKind::Rumors),
            "goods" => {
                todo!();
            }
            _ => Err(TreasureKindParseError::ParseTreasure(value.to_owned())),
        }
    }
}

impl TryFrom<&str> for TreasureKind {
    type Error = TreasureKindParseError;

    fn try_from(value: &str) -> Result<Self, TreasureKindParseError> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for TreasureKind {
    type Error = TreasureKindParseError;

    fn try_from(value: String) -> Result<Self, TreasureKindParseError> {
        Self::from_str(&value)
    }
}

impl TryFrom<&GString> for TreasureKind {
    type Error = TreasureKindParseError;

    fn try_from(value: &GString) -> Result<Self, TreasureKindParseError> {
        let value: String = value.to_string();
        let substrings: Vec<String> = value
            .split(',')
            .map(|substring| substring.to_lowercase())
            .collect();

        if substrings.is_empty() || substrings.len() > 3 {
            return Err(TreasureKindParseError::ArgumentCount(
                "1 to 3",
                substrings.len(),
            ));
        }

        match substrings[0].as_str() {
            "water" => Ok(TreasureKind::Water),
            "goods" => {
                if substrings.len() != 3 {
                    Err(TreasureKindParseError::ArgumentCount("3", substrings.len()))
                } else {
                    Ok(TreasureKind::from_str(&substrings[2])?)
                }
            }
            "camels" => Ok(TreasureKind::Camels),
            "rumors" => Ok(TreasureKind::Rumors),
            _ => Err(TreasureKindParseError::ParseTreasure(
                substrings
                    .into_iter()
                    .next()
                    .expect("Substrings should have length greater than 0."),
            )),
        }
    }
}

#[derive(GodotClass, Debug)]
#[class(init, base=Node2D)]
pub struct Treasure {
    base: Base<Node2D>,
    #[export]
    initial_treasure: GString,
    pub kind: Option<TreasureKind>,
}

impl Treasure {
    pub fn get_sprite(&self) -> Gd<Sprite2D> {
        self.to_gd()
            .get_node_as::<Sprite2D>("./Control/SpriteContainer/TreasureSprite")
    }
}

#[godot_api]
impl INode2D for Treasure {
    fn ready(&mut self) {
        if self.initial_treasure.is_empty() {
            return;
        }

        let parse_result = TreasureKind::try_from(&self.initial_treasure);

        if parse_result.is_err() {
            let error = parse_result
                .as_ref()
                .expect_err("Expected parse result to be an error.");
            godot_error!("{error}",);
        }

        let parsed_kind = parse_result.ok();

        let mut sprite = self.get_sprite();

        self.kind = parsed_kind.clone();

        if sprite.get_texture().is_none() {
            match parsed_kind {
                Some(TreasureKind::Water) => {
                    let texture = parsed_kind.unwrap().icon_texture().unwrap();

                    sprite.set_texture(&texture);
                }
                _ => {
                    todo!();
                }
            }
        }
    }
}
