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

#[derive(Debug)]
enum Good {
    Incense,
    Myrrh,
    Salt,
    Gems,
}

#[derive(Debug)]
enum TreasureKind {
    Water,
    Goods(Good),
    Camels,
    Rumors,
    None,
}

impl TreasureKind {
    fn icon_texure(&self) -> Result<Gd<ImageTexture>, ErrorKind> {
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
            TreasureKind::None => "",
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

#[derive(GodotClass)]
#[class(init, base=Sprite2D)]
pub struct Treasure {
    base: Base<Sprite2D>,

    #[export]
    initial_treasure: GString,

    kind: Option<TreasureKind>,
}

#[godot_api]
impl ISprite2D for Treasure {
    fn ready(&mut self) {
        let mut this_sprite = self.base().clone();

        dbg!(&this_sprite);

        if self.initial_treasure.is_empty() {
            return;
        }

        dbg!(&self.initial_treasure);

        let parse_result = TreasureKind::try_from(&self.initial_treasure);

        dbg!(&parse_result);

        if let Ok(kind) = parse_result {
            self.kind = Some(kind);
        } else {
            let error = parse_result.expect_err("Expected parse result to be an error.");
            godot_error!("{error}",);
        };

        if this_sprite.get_texture().is_none() {
            match self.kind {
                Some(TreasureKind::Water) => {
                    let texture_image = self.kind.as_ref().unwrap().icon_texure().unwrap();
                    dbg!(&texture_image);

                    this_sprite.set_texture(&texture_image)
                }
                _ => {
                    todo!();
                }
                None => {}
            }
        }
    }
}
