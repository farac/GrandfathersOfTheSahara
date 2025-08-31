use godot::{
    builtin::Color,
    classes::{ColorRect, Control, IControl, INode2D, Node2D},
    obj::{Gd, WithBaseField},
    prelude::{godot_api, Base, GodotClass},
};

#[derive(Default, Clone, Copy, Debug)]
pub enum PlayerName {
    #[default]
    White,
    Orange,
    Red,
    Blue,
}

impl PlayerName {
    pub fn cycle(player: &Self) -> Self {
        match player {
            Self::White => Self::Orange,
            Self::Orange => Self::Red,
            Self::Red => Self::Blue,
            Self::Blue => Self::White,
        }
    }
    pub fn color(&self) -> Color {
        match self {
            PlayerName::White => Color::from_rgb(255., 255., 255.),
            PlayerName::Orange => Color::from_rgb(255., 128., 0.),
            PlayerName::Red => Color::from_rgb(255., 0., 0.),
            PlayerName::Blue => Color::from_rgb(0., 0., 255.),
        }
    }
}

impl From<u8> for PlayerName {
    fn from(value: u8) -> Self {
        match value % 4 {
            0 => Self::White,
            1 => Self::Orange,
            2 => Self::Red,
            3 => Self::Blue,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum BuildingType {
    #[default]
    Shortest = 0,
    Short = 1,
    Tall = 2,
    Tallest = 3,
}

impl From<u8> for BuildingType {
    fn from(value: u8) -> Self {
        match value % 4 {
            0 => Self::Shortest,
            1 => Self::Short,
            2 => Self::Tall,
            3 => Self::Tallest,
            _ => unreachable!(),
        }
    }
}

impl BuildingType {
    fn get_height(&self, player: &PlayerName) -> f32 {
        match player {
            PlayerName::White => match self {
                Self::Shortest => 1.,
                Self::Short => 5.,
                Self::Tall => 7.,
                Self::Tallest => 7.,
            },
            PlayerName::Orange => match self {
                Self::Shortest => 1.5,
                Self::Short => 4.,
                Self::Tall => 6.,
                Self::Tallest => 8.,
            },
            PlayerName::Red => match self {
                Self::Shortest => 2.,
                Self::Short => 3.5,
                Self::Tall => 5.5,
                Self::Tallest => 8.5,
            },
            PlayerName::Blue => match self {
                BuildingType::Shortest => 2.5,
                BuildingType::Short => 3.,
                BuildingType::Tall => 5.,
                BuildingType::Tallest => 9.,
            },
        }
    }
    fn get_height_in_px(&self, player: &PlayerName) -> f32 {
        let unit_height = self.get_height(player);

        let unit = 256 / 20;

        unit_height * unit as f32
    }
}

#[derive(GodotClass, Debug)]
#[class(init, base=Node2D)]
pub struct PlayerBuilding {
    base: Base<Node2D>,

    height: BuildingType,
    player: PlayerName,
}

impl PlayerBuilding {
    fn get_sprite(&self) -> Gd<ColorRect> {
        let base = self.base();
        base.get_node_as("./ColorRect")
    }
    fn set_height(&mut self, height: BuildingType, player: PlayerName) {
        let mut sprite = self.get_sprite();
        let mut initial_size = sprite.get_size();

        self.height = height;

        initial_size.y = self.height.get_height_in_px(&player);

        sprite.set_size(initial_size);

        sprite.set_color(player.color());
    }
}

#[godot_api]
impl INode2D for PlayerBuilding {
    fn ready(&mut self) {
        self.set_height(self.height, self.player);
    }
}

#[derive(GodotClass, Debug)]
#[class(init, base=Control)]
pub struct PlayerFigures {
    base: Base<Control>,

    #[export]
    player_number: u8,
}

impl PlayerFigures {
    fn get_building(&self, building: BuildingType) -> Gd<PlayerBuilding> {
        let base = self.base();
        let building = building as u8 + 1;

        base.get_node_as(&format!(
            "./Panel/HBoxContainer/MarginContainer{}/CenterContainer{}/PlayerBuilding{}",
            building, building, building
        ))
    }
}

const BUILDINGS: [BuildingType; 4] = [
    BuildingType::Shortest,
    BuildingType::Short,
    BuildingType::Tall,
    BuildingType::Tallest,
];

#[godot_api]
impl IControl for PlayerFigures {
    fn ready(&mut self) {
        BUILDINGS.iter().for_each(|b| {
            let mut gd_building = self.get_building(*b);
            let mut building = gd_building.bind_mut();

            building.set_height(*b, PlayerName::from(self.player_number));
        })
    }
}
