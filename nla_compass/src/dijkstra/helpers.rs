use std::hash::{Hash, Hasher};
use std::rc::Rc;
use robotics_lib::world::tile::Tile;
use robotics_lib::interface::Direction;

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

pub(crate) fn get_cost(current_tile: &Tile, next_tile: &Tile) -> usize {
    let mut cost: usize = next_tile.tile_type.properties().cost();
    let elevation_diff = next_tile.elevation - current_tile.elevation;
    if elevation_diff > 0 {
        cost += elevation_diff.pow(2);
    }
    cost
}

pub(crate) fn to_directions(v: &Vec<Wrapper>) -> Vec<Direction> {
    let mut result: Vec<Direction> = Vec::new();
    for i in 1..v.len() {
        if v[i-1].col != v[i].col {
            if v[i].col > v[i-1].col {
                result.push(Direction::Right)
            }
            else { result.push(Direction::Left) }
        }
        else {
            if v[i].row > v[i-1].row {
                result.push(Direction::Down)
            }
            else { result.push(Direction::Up) }
        }
    }
    result
}