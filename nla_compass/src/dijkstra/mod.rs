mod helpers;
use std::collections::HashMap;
use std::hash::Hasher;
use std::ops::Div;
use pathfinding::prelude::{build_path, dijkstra_all};
use robotics_lib::interface::Direction;
use robotics_lib::interface::robot_map;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Tile;
use core::hash::Hash;
use std::rc::Rc;
use self::helpers::get_cost;

struct Wrapper {
    world: Rc<Vec<Vec<Option<Tile>>>>,
    row: usize,
    col: usize
}
impl PartialEq for Wrapper {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}
impl Eq for Wrapper {}
impl Hash for Wrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.row, self.col).hash(state);
    }
}
impl Clone for Wrapper {
    fn clone(&self) -> Self {
        Self {
            world: Rc::clone(&self.world),
            row: self.row,
            col: self.col
        }
    }
}

fn successors(&node: &Wrapper) -> Vec<(Wrapper, usize)> {
    let mut successors = Vec::new();
    // Iterate over 3x3 adjacent tiles
    for row in [node.row-1, node.row+1] {
        for col in [node.col-1, node.col+1] {
            if let Some(_) = node.world.as_ref()[row][col] {
                successors.push((Wrapper {
                    world: node.world.clone(), row, col
                }, get_cost(&node.world.as_ref()[node.row][node.col].unwrap(),
                            &node.world.as_ref()[row][col].unwrap())))
            }
        };
    }
    successors
}