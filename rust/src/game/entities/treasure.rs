use godot::classes::{CompressedTexture2D, Control, Label, Sprite2D};
use godot::prelude::*;

use thiserror::Error;

use crate::game::components::hover_outline::ActionCollisionSquare;

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

impl TreasureKind {
    pub fn get_label_text(&self) -> &'static str {
        match self {
            TreasureKind::Water => "Water",
            TreasureKind::DoubleWater => "Water x2",
            TreasureKind::Camels => "Camels",
            TreasureKind::Rumors => "Rumors",
            TreasureKind::Goods(good) => match good {
                Good::Incense => "Incense",
                Good::Myrrh => "Myrrh",
                Good::Salt => "Salt",
                Good::Gems => "Gems",
            },
            TreasureKind::None => "No treasure",
        }
    }
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

    #[init(val = TreasureKind::None)]
    pub kind: TreasureKind,
    #[init(val = TreasureKind::None)]
    sprite_kind: TreasureKind,

    tooltip_visible: bool,
}

impl Treasure {
    pub fn get_hover_collision(&self) -> Gd<ActionCollisionSquare> {
        self.base().get_node_as("./ActionCollision")
    }
    pub fn get_sprites(&self) -> [Gd<Sprite2D>; 2] {
        [
            self.to_gd()
                .get_node_as::<Sprite2D>("./Control/SpriteContainer/TreasureSprite"),
            self.to_gd().get_node_as::<Sprite2D>(
                "./Tooltip/Panel/VBoxContainer/SpriteContainer/TreasureSprite",
            ),
        ]
    }
    pub fn get_label(&self) -> Gd<Label> {
        self.to_gd()
            .get_node_as::<Label>("./Tooltip/Panel/VBoxContainer/Panel/MarginContainer/Label")
    }
    fn hide_icon(&self) {
        self.get_sprites()[0].set_visible(false);
    }
    fn show_icon(&self) {
        self.get_sprites()[0].set_visible(true);
    }
    fn get_tooltip(&self) -> Gd<Control> {
        self.base().get_node_as("./Tooltip")
    }
    fn change_tooltip_visibility(&mut self, visible: bool) {
        if self.kind == TreasureKind::None {
            return;
        }

        let mut tooltip = self.get_tooltip();
        tooltip.set_visible(visible);
    }
}

#[godot_api]
impl INode2D for Treasure {
    fn ready(&mut self) {
        self.hide_icon();
        let gd_self = self.to_gd();

        let hover_collision = self.get_hover_collision();

        hover_collision
            .bind()
            .base()
            .signals()
            .mouse_entered()
            .connect_other(&gd_self, |this| this.tooltip_visible = true);

        hover_collision
            .bind()
            .base()
            .signals()
            .mouse_exited()
            .connect_other(&gd_self, |this| this.tooltip_visible = false);
    }
    fn process(&mut self, _dt: f64) {
        let is_tooltip_visible = self.get_tooltip().is_visible();

        if self.tooltip_visible != is_tooltip_visible {
            self.change_tooltip_visibility(self.tooltip_visible);
        }

        if self.tooltip_visible {
            let mut tooltip = self.get_tooltip();
            let mouse_position = self
                .base()
                .get_viewport()
                .expect("Expected node to have a viewport")
                .get_mouse_position();

            tooltip.set_global_position(mouse_position);
        }

        if self.sprite_kind == self.kind {
            return;
        }

        if self.kind == TreasureKind::None {
            self.sprite_kind = self.kind.clone();

            return self.hide_icon();
        }

        let mut sprites = self.get_sprites();

        let texture: Option<Gd<CompressedTexture2D>> = (&self.kind)
            .try_into()
            .expect("Provided invalid TreasureKind to Treasure.ready()");

        if let Some(texture) = texture {
            sprites[0].set_texture(&texture);
            sprites[1].set_texture(&texture);
        }

        let label = self.kind.get_label_text();
        let mut gd_label = self.get_label();

        gd_label.set_text(label);

        self.sprite_kind = self.kind.clone();

        if !sprites[0].is_visible() {
            self.show_icon();
        }
    }
}
