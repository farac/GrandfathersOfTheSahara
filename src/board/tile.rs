use std::{convert::TryFrom, fmt};

#[derive(Clone, Debug)]
pub enum Resource {
    Myrrh,
    Salt,
    Gem,
    Incense,
}

#[derive(Clone, Debug)]
pub enum Water {
    Single = 1,
    Double = 2,
}

#[derive(Clone, Debug)]
pub enum Bonus {
    Water(Water),
    Rumor,
    Camel,
}

#[derive(Clone, Debug)]
pub struct Oasis {
    pub position: [bool; 4],
    pub resources: Vec<Resource>,
    pub bonuses: Vec<Bonus>,
}

type OasisTile = Vec<Oasis>;

#[derive(Clone, Debug)]
pub enum Tile {
    NonDesert(OasisTile),
    Desert,
}
impl Tile {
    pub fn new(oases_param: Option<Vec<Oasis>>) -> Self {
        if let Some(oases) = oases_param {
            if !oases.iter().any(|oasis| oasis.position.contains(&true)) {
                return Self::Desert;
            }

            Self::NonDesert(oases)
        } else {
            Self::Desert
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonDesert(_tile) => {
                write!(
                    f,
                    "     |
     |
-----O-----
     |
     |",
                )?;
            }
            Self::Desert => {
                write!(
                    f,
                    "     |
     |
-----O-----
     |
     |",
                )?;
            }
        }
        Ok(())
    }
}

pub enum Side {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl TryFrom<usize> for Side {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == Side::North as usize => Ok(Side::North),
            x if x == Side::West as usize => Ok(Side::West),
            x if x == Side::South as usize => Ok(Side::South),
            x if x == Side::East as usize => Ok(Side::East),
            _ => Err(()),
        }
    }
}
