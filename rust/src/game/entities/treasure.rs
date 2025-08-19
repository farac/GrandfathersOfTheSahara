use godot::classes::{CompressedTexture2D, Sprite2D};
use godot::prelude::*;

use thiserror::Error;

const MAX_TREASURE_SUBSTRINGS: u8 = 2;

#[derive(Error, Debug)]
pub enum TreasureKindParseError {
    #[error("TreasureKind expected {0} arguments, received {1}")]
    ArgumentCount(String, usize),
    #[error("TreasureKind expected one of water, double_water, goods:[gems, myrrh, salt, incense], camels, rumors, received {0}")]
    ParseTreasure(String),
    #[error("TreasureKinds expected goods to be one of goods:[gems, myrrh, salt, incense], received {0}")]
    ParseGoods(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Good {
    Incense,
    Myrrh,
    Salt,
    Gems,
}

impl TryFrom<&str> for Good {
    type Error = TreasureKindParseError;

    fn try_from(value: &str) -> Result<Self, TreasureKindParseError> {
        match value {
            "incense" => Ok(Good::Incense),
            "myrrh" => Ok(Good::Myrrh),
            "salt" => Ok(Good::Salt),
            "gems" => Ok(Good::Gems),
            _ => Err(TreasureKindParseError::ParseGoods(String::from(value))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TreasureKind {
    Water,
    DoubleWater,
    Goods(Good),
    Camels,
    Rumors,
    None,
}

impl From<&TreasureKind> for &str {
    fn from(val: &TreasureKind) -> Self {
        match val {
            TreasureKind::Water => "water",
            TreasureKind::DoubleWater => "double_water",
            TreasureKind::Camels => "camels",
            TreasureKind::Rumors => "rumors",
            TreasureKind::Goods(good) => match good {
                Good::Incense => "goods/incense",
                Good::Myrrh => "goods/myrrh",
                Good::Salt => "goods/salt",
                Good::Gems => "goods/gems",
            },
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

impl TryFrom<&str> for TreasureKind {
    type Error = TreasureKindParseError;

    fn try_from(value: &str) -> Result<Self, TreasureKindParseError> {
        let substrings: Vec<String> = value
            .split(':')
            .map(|substring| substring.to_lowercase())
            .collect();

        if substrings.is_empty() || substrings.len() > MAX_TREASURE_SUBSTRINGS as usize {
            return Err(TreasureKindParseError::ArgumentCount(
                "1 to 2".to_owned(),
                substrings.len(),
            ));
        }

        match substrings[0].as_str() {
            "double_water" => Ok(TreasureKind::DoubleWater),
            "water" => Ok(TreasureKind::Water),
            "goods" => {
                if substrings.len() != MAX_TREASURE_SUBSTRINGS as usize {
                    Err(TreasureKindParseError::ArgumentCount(
                        "2".to_owned(),
                        substrings.len(),
                    ))
                } else {
                    Ok(TreasureKind::Goods(Good::try_from(substrings[1].as_str())?))
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

impl TryFrom<&String> for TreasureKind {
    type Error = TreasureKindParseError;

    fn try_from(value: &String) -> Result<Self, TreasureKindParseError> {
        TreasureKind::try_from(value.as_str())
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

impl TryFrom<&TreasureKind> for Option<Gd<CompressedTexture2D>> {
    type Error = IoError;

    fn try_from(kind: &TreasureKind) -> Result<Option<Gd<CompressedTexture2D>>, IoError> {
        if *kind == TreasureKind::None {
            return Ok(None);
        }

        let icon_name: &str = kind.into();

        let path = format!("res://assets/icons/treasure/{icon_name}.png");

        try_load::<CompressedTexture2D>(&path).map(Some)
    }
}

impl TryFrom<TreasureKind> for Option<Gd<CompressedTexture2D>> {
    type Error = IoError;

    fn try_from(kind: TreasureKind) -> Result<Option<Gd<CompressedTexture2D>>, IoError> {
        let image_texture: Option<Gd<CompressedTexture2D>> = (&kind).try_into()?;

        Ok(image_texture)
    }
}

#[derive(GodotClass, Debug)]
#[class(init, base=Node2D)]
pub struct Treasure {
    base: Base<Node2D>,

    #[export]
    #[init(val = GString::from("none"))]
    initial_treasure: GString,
    #[init(val = Some(TreasureKind::None))]
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

        let texture: Option<Gd<CompressedTexture2D>> = (&kind)
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
