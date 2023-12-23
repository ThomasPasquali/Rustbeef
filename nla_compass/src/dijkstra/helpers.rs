use robotics_lib::world::tile::Tile;
use robotics_lib::interface::Direction;
use super::Wrapper;

pub(crate) fn get_cost(current_tile: &Tile, next_tile: &Tile) -> usize {
    let mut cost: usize = next_tile.tile_type.properties().cost();
    let elevation_diff = next_tile.elevation - current_tile.elevation;
    if elevation_diff > 0 {
        cost += elevation_diff.pow(2);
    }
    cost
}

pub(crate) fn get_direction(v: &Vec<Wrapper>) -> Direction {
    if v[0].col != v[1].col {
        if v[1].col > v[0].col { Direction::Right } else { Direction::Left }
    }
    else {
        if v[1].row > v[0].row { Direction::Down } else { Direction::Up }
    }
}