pub mod helpers;
use std::hash::Hasher;
use core::hash::Hash;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use self::helpers::get_cost;
use pathfinding::prelude::{build_path, dijkstra_all};
use robotics_lib::interface::Direction;
use robotics_lib::world::tile::Tile;
use crate::compass::MoveError;
use crate::dijkstra::helpers::get_direction;

pub(crate) struct Wrapper {
    pub(crate) world: Rc<Vec<Vec<Option<Tile>>>>,
    pub(crate) row: usize,
    pub(crate) col: usize
}

impl Debug for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.row, self.col)
    }
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

fn successors(node: &Wrapper) -> Vec<(Wrapper, usize)> {
    let mut result: Vec<(Wrapper, usize)> = Vec::new();

    // continue only if start_tile is some
    if node.world.as_ref()[node.row][node.col].as_ref().is_none() {
        return result;
    }


    // left tile
    if node.col > 0 {
        if let Some(_) = node.world.as_ref()[node.row][node.col-1] {
            if node.world.as_ref()[node.row][node.col-1].clone().unwrap().tile_type.properties().walk() {
                result.push((Wrapper {
                    world: node.world.clone(), row: node.row, col: node.col-1
                }, get_cost(&node.world.as_ref()[node.row][node.col].as_ref().unwrap(),
                            &node.world.as_ref()[node.row][node.col-1].as_ref().unwrap())));
            }
        }
    }

    // right tile
    if node.col < node.world.len()-1 {
        if let Some(_) = node.world.as_ref()[node.row][node.col+1] {
            if node.world.as_ref()[node.row][node.col+1].clone().unwrap().tile_type.properties().walk() {
                result.push((Wrapper {
                    world: node.world.clone(), row: node.row, col: node.col+1
                }, get_cost(&node.world.as_ref()[node.row][node.col].as_ref().unwrap(),
                            &node.world.as_ref()[node.row][node.col+1].as_ref().unwrap())));
            }
        }
    }

    // up tile
    if node.row > 0 {
        if let Some(_) = node.world.as_ref()[node.row-1][node.col] {
            if node.world.as_ref()[node.row-1][node.col].clone().unwrap().tile_type.properties().walk() {
                result.push((Wrapper {
                    world: node.world.clone(), row: node.row-1, col: node.col
                }, get_cost(&node.world.as_ref()[node.row][node.col].as_ref().unwrap(),
                            &node.world.as_ref()[node.row-1][node.col].as_ref().unwrap())));
            }
        }
    }

    // down tile
    if node.row < node.world.len()-1 {
        if let Some(_) = node.world.as_ref()[node.row+1][node.col] {
            if node.world.as_ref()[node.row+1][node.col].clone().unwrap().tile_type.properties().walk() {
                result.push((Wrapper {
                    world: node.world.clone(), row: node.row+1, col: node.col
                }, get_cost(&node.world.as_ref()[node.row][node.col].as_ref().unwrap(),
                            &node.world.as_ref()[node.row+1][node.col].as_ref().unwrap())));
            }
        }
    }

    return result;
}

pub(crate) fn get_path_vector(start: (usize, usize), destination: (usize, usize), map: &Vec<Vec<Option<Tile>>>) -> Vec<Wrapper> {
    let start_wrapper = Wrapper {
        world: Rc::new(map.clone()),
        row: start.0,
        col: start.1,
    };
    let destination_wrapper = Wrapper {
        world: Rc::new(map.clone()),
        row: destination.0,
        col: destination.1,
    };
    let reachables_from_start = dijkstra_all(&start_wrapper, successors);
    let path_start_to_dest = build_path(&destination_wrapper, &reachables_from_start);
    path_start_to_dest
    //get_direction(&path_start_to_dest)
}