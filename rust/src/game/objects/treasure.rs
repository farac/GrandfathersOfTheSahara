use std::{io::ErrorKind, str::FromStr};

use godot::classes::{Image, ImageTexture, Sprite2D};
use godot::prelude::*;

use thiserror::Error;

use crate::godot_dbg;

#[derive(Error, Debug)]
pub enum TreasureKindParseError {
    #[error("TreasureKind expected {0} arguments, received {1}")]
    ArgumentCount(&'static str, usize),
    #[error("TreasureKind expected one of water, goods, camels, rumors, received {0}")]
    ParseTreasure(String),
    #[error("TreasureKinds expected goods to be one of , received {0}")]
    ParseGoods(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Good {
    Incense,
    Myrrh,
    Salt,
    Gems,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TreasureKind {
    Water,
    Goods(Good),
    Camels,
    Rumors,
    None,
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

impl From<&TreasureKind> for GString {
    fn from(value: &TreasureKind) -> GString {
        let kind_string: &str = value.into();

        GString::from(kind_string)
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
            "none" => Ok(TreasureKind::None),
            "" => Ok(TreasureKind::None),
            _ => Err(TreasureKindParseError::ParseTreasure(value.to_owned())),
        }
    }
}

impl TryFrom<&String> for TreasureKind {
    type Error = TreasureKindParseError;

    fn try_from(value: &String) -> Result<Self, TreasureKindParseError> {
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
            "none" => Ok(TreasureKind::None),
            "" => Ok(TreasureKind::None),
            _ => Err(TreasureKindParseError::ParseTreasure(
                substrings
                    .into_iter()
                    .next()
                    .expect("Substrings should have length greater than 0."),
            )),
        }
    }
}

impl TryFrom<String> for TreasureKind {
    type Error = TreasureKindParseError;

    fn try_from(value: String) -> Result<Self, TreasureKindParseError> {
        TreasureKind::try_from(&value)
    }
}

impl TryFrom<GString> for TreasureKind {
    type Error = TreasureKindParseError;

    fn try_from(value: GString) -> Result<Self, TreasureKindParseError> {
        TreasureKind::try_from(value.to_string())
    }
}

impl TryFrom<&GString> for TreasureKind {
    type Error = TreasureKindParseError;

    fn try_from(value: &GString) -> Result<Self, TreasureKindParseError> {
        TreasureKind::try_from(value.to_string())
    }
}

impl TryFrom<&TreasureKind> for Option<Gd<ImageTexture>> {
    type Error = ErrorKind;

    fn try_from(kind: &TreasureKind) -> Result<Option<Gd<ImageTexture>>, ErrorKind> {
        if *kind == TreasureKind::None {
            return Ok(None);
        }

        let icon_name: &str = kind.into();

        let path = format!("res://assets/icons/treasure/{icon_name}.png");

        let image = Image::load_from_file(&path);

        if let Some(image) = image {
            let texture = ImageTexture::create_from_image(&image);

            Ok(texture)
        } else {
            godot_dbg!(path);

            Err(ErrorKind::NotFound)
        }
    }
}

impl TryFrom<TreasureKind> for Option<Gd<ImageTexture>> {
    type Error = ErrorKind;

    fn try_from(kind: TreasureKind) -> Result<Option<Gd<ImageTexture>>, ErrorKind> {
        let image_texture: Option<Gd<ImageTexture>> = (&kind).try_into()?;

        Ok(image_texture)
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
    fn set_icon_hidden(&self) {
        self.get_sprite().set_visible(false);
    }
    pub fn set_kind(&mut self, kind: TreasureKind) {
        let mut sprite = self.get_sprite();

        if kind == TreasureKind::None {
            self.kind = Some(kind.clone());

            return self.set_icon_hidden();
        }

        let texture: Option<Gd<ImageTexture>> = (&kind)
            .try_into()
            .expect("Provided invalid TreasureKind to Treasure.ready()");

        if let Some(texture) = texture {
            sprite.set_texture(&texture);
        }

        self.kind = Some(kind);
    }
}

#[godot_api]
impl INode2D for Treasure {
    fn ready(&mut self) {
        if self.kind.is_none() {
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

        if parsed_kind.is_none() {
            return;
        }

        self.set_kind(parsed_kind.unwrap_or(TreasureKind::None));
    }
}
