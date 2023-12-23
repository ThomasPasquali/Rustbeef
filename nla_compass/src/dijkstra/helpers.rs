use robotics_lib::world::tile::Tile;

pub(crate) fn get_cost(current_tile: &Tile, next_tile: &Tile) -> usize {
    let mut cost: usize = next_tile.tile_type.properties().cost();
    let elevation_diff = next_tile.elevation - current_tile.elevation;
    if elevation_diff > 0 {
        cost += elevation_diff.pow(2);
    }
    cost
}