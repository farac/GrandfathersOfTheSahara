pub mod tile;

use std::fmt;
use tile::*;

#[derive(Default, Debug)]
pub struct Board(pub [[Option<Tile>; 11]; 11]);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().for_each(|row| {
            writeln!(f, "{:?}", row).unwrap_or(());
        });
        Ok(())
    }
}

pub trait Cross {
    fn fill(cross: Option<[Option<[Option<Tile>; 5]>; 4]>) -> Board {
        let mut board: Board = Default::default();

        if let Some(sides) = cross {
            for (side, tiles) in sides.into_iter().enumerate() {
                let side = Side::try_from(side);

                match side {
                    Ok(Side::North) => {
                        if let Some(tiles) = tiles {
                            for (index, tile) in tiles.into_iter().enumerate() {
                                board.0[index][5] = tile;
                            }
                        }
                    }
                    Ok(Side::East) => {
                        if let Some(tiles) = tiles {
                            for (index, tile) in tiles.into_iter().enumerate() {
                                board.0[5][6 + index] = tile;
                            }
                        }
                    }
                    Ok(Side::South) => {
                        if let Some(tiles) = tiles {
                            for (index, tile) in tiles.into_iter().enumerate() {
                                board.0[6 + index][5] = tile;
                            }
                        }
                    }
                    Ok(Side::West) => {
                        if let Some(tiles) = tiles {
                            for (index, tile) in tiles.into_iter().enumerate() {
                                board.0[5][index] = tile;
                            }
                        }
                    }
                    Err(()) => (),
                }
            }
        }

        board
    }
}

impl Cross for Board {}
