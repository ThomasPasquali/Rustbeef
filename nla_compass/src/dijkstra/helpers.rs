use robotics_lib::world::tile::Tile;
use robotics_lib::interface::Direction;
use crate::compass::MoveError;
use super::Wrapper;

pub(crate) fn get_cost(current_tile: &Tile, next_tile: &Tile) -> usize {
    let mut cost: usize = next_tile.tile_type.properties().cost();
    if next_tile.elevation > current_tile.elevation {
        let elevation_diff = next_tile.elevation - current_tile.elevation;
        cost += elevation_diff.pow(2);
    }

    cost
}

pub(crate) fn get_direction(v: &Vec<Wrapper>) -> Result<Direction, MoveError> {
    if v.len() == 1 {
        Err(MoveError::NoAvailableMove)
    }
    else {
        if v[0].col != v[1].col {
            if v[1].col > v[0].col { Ok(Direction::Right) } else { Ok(Direction::Left) }
        }
        else {
            if v[1].row > v[0].row { Ok(Direction::Down) } else { Ok(Direction::Up) }
        }
    }

}