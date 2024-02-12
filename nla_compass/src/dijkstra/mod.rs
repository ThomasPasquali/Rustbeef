mod helpers;
use std::hash::Hasher;
use core::hash::Hash;
use std::rc::Rc;
use self::helpers::get_cost;
use pathfinding::prelude::{build_path, dijkstra_all};
use robotics_lib::interface::Direction;
use robotics_lib::world::tile::Tile;
use crate::compass::MoveError;
use crate::dijkstra::helpers::get_direction;

#[derive(Debug)]
pub(crate) struct Wrapper {
    pub(crate) world: Rc<Vec<Vec<Option<Tile>>>>,
    pub(crate) row: usize,
    pub(crate) col: usize
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
    //println!("\nHere at the start");
    let mut row_start = node.row;
    let mut col_start = node.col;
    let mut row_end = node.row + 1;
    let mut col_end = node.col + 1;
    if row_start > 0 {
        row_start -= 1;
    }
    if col_start > 0 {
        col_start -= 1;
    }
    if row_end < (node.world.len()) {
        row_end += 1;
    }
    if col_end < (node.world.len()) {
        col_end += 1;
    }
    let mut result: Vec<(Wrapper, usize)> = Vec::new();

    // continue only if start_tile is some
    if &node.world.as_ref()[node.row][node.col].as_ref().is_none() {
        return result;
    }

    // Iterate over 3x3 adjacent tiles

    for row in row_start..row_end {
        for col in col_start..col_end {
            //println!("{}{}: range {}{} - {}{}", row, col, row_start, col_start, row_end, col_end);
            if let Some(_) = node.world.as_ref()[row][col] {
                //println!("{}{}, is some", row, col);
                result.push((Wrapper {
                    world: node.world.clone(), row, col
                }, get_cost(&node.world.as_ref()[node.row][node.col].as_ref().unwrap(),
                            &node.world.as_ref()[row][col].as_ref().unwrap())));
                //println!("pushed\n");
            }
        };
    }
    //println!("loop ended");
    return result;
}

pub(crate) fn get_move_for_coordinate(start: (usize, usize), destination: (usize, usize), map: &Vec<Vec<Option<Tile>>>) -> Result<Direction, MoveError> {
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
    get_direction(&path_start_to_dest)
}