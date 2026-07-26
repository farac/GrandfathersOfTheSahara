use std::collections::HashMap;
use std::collections::HashSet;

use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Bfs;

use crate::game::entities::player::PlayerName;
use crate::util::flags::CardinalDirection;
use crate::util::flags::CardinalDirectionFlags;
use crate::util::flags::DIRECTIONS;

/// Board coordinates of the 4 arm-end tiles of the starting cross, and the
/// player whose caravan starts there.
pub const STARTING_POSITIONS: [((u8, u8), PlayerName); 4] = [
    ((5, 10), PlayerName::White),
    ((10, 5), PlayerName::Orange),
    ((5, 0), PlayerName::Red),
    ((0, 5), PlayerName::Blue),
];

// (x, y, crossed_white): the bool marks that an oasis line was crossed. A
// move may cross only one, so those nodes are dead ends.
type MoveNode = (u8, u8, bool);

#[derive(Debug, Default, Clone)]
pub struct BoardGraph {
    tiles: HashMap<(u8, u8), CardinalDirectionFlags>,
}

impl BoardGraph {
    pub fn insert_tile(&mut self, coordinates: (u8, u8), oasis_directions: CardinalDirectionFlags) {
        self.tiles.insert(coordinates, oasis_directions);
    }

    /// Tiles the caravan can reach: any distance over desert (brown) lines,
    /// plus one final step over an oasis (white) line. Occupied tiles can be
    /// passed through but are not valid destinations.
    pub fn reachable_tiles(&self, from: (u8, u8), occupied: &HashSet<(u8, u8)>) -> Vec<(u8, u8)> {
        let graph = self.build_move_graph();
        let start: MoveNode = (from.0, from.1, false);

        if !graph.contains_node(start) {
            return vec![];
        }

        let mut visited: HashSet<(u8, u8)> = HashSet::new();
        let mut bfs = Bfs::new(&graph, start);

        while let Some(node) = bfs.next(&graph) {
            if (node.0, node.1) == from {
                continue;
            }

            visited.insert((node.0, node.1));
        }

        visited
            .into_iter()
            .filter(|coordinates| !occupied.contains(coordinates))
            .collect()
    }

    fn build_move_graph(&self) -> DiGraphMap<MoveNode, ()> {
        let mut graph = DiGraphMap::new();

        for (coordinates, oasis_directions) in &self.tiles {
            for direction in DIRECTIONS {
                let Some(neighbor) = Self::neighbor_coordinates(*coordinates, &direction) else {
                    continue;
                };

                if !self.tiles.contains_key(&neighbor) {
                    continue;
                }

                let is_oasis_edge =
                    oasis_directions.contains(CardinalDirectionFlags::from(&direction));

                let source: MoveNode = (coordinates.0, coordinates.1, false);
                let target: MoveNode = (neighbor.0, neighbor.1, is_oasis_edge);

                graph.add_edge(source, target, ());
            }
        }

        graph
    }

    fn neighbor_coordinates(
        coordinates: (u8, u8),
        direction: &CardinalDirection,
    ) -> Option<(u8, u8)> {
        let (dx, dy) = direction.get_coordinate_offset();

        let x = coordinates.0 as i32 + dx;
        let y = coordinates.1 as i32 + dy;

        if x < 0 || y < 0 {
            return None;
        }

        Some((x as u8, y as u8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    /// Builds a graph from a west-to-east strip. The listed tiles get an oasis
    /// ("white") edge on their east side. All other edges are desert.
    fn strip(tile_count: u8, white_east_sides: &[u8]) -> BoardGraph {
        let mut graph = BoardGraph::default();

        for x in 0..tile_count {
            let oasis = if white_east_sides.contains(&x) {
                CardinalDirectionFlags::E
            } else {
                CardinalDirectionFlags::empty()
            };

            graph.insert_tile((x, 0), oasis);
        }

        graph
    }

    #[test]
    fn starting_positions_are_the_four_cross_arm_ends() {
        let coordinates: Vec<(u8, u8)> = STARTING_POSITIONS.iter().map(|(c, _)| *c).collect();

        assert_eq!(coordinates, vec![(5, 10), (10, 5), (5, 0), (0, 5)]);
    }

    #[test_case(4, &[], &[] => vec![(1, 0), (2, 0), (3, 0)] ; "brown lines chain arbitrarily far")]
    #[test_case(3, &[0], &[] => vec![(1, 0)] ; "crossing a white line ends the move")]
    #[test_case(4, &[1], &[] => vec![(1, 0), (2, 0)] ; "a white line is the final hop after brown ones")]
    #[test_case(3, &[], &[(1, 0)] => vec![(2, 0)] ; "occupied tiles are excluded but still passable")]
    #[test_case(1, &[], &[] => Vec::<(u8, u8)>::new() ; "an isolated tile has no reachable tiles")]
    fn reachable_tiles(
        tile_count: u8,
        white_east_sides: &[u8],
        occupied: &[(u8, u8)],
    ) -> Vec<(u8, u8)> {
        let graph = strip(tile_count, white_east_sides);
        let occupied: HashSet<(u8, u8)> = occupied.iter().copied().collect();

        let mut reachable = graph.reachable_tiles((0, 0), &occupied);
        reachable.sort();
        reachable
    }
}
