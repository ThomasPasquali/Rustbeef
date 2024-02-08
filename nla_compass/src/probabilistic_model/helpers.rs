use pathfinding::num_traits::Pow;
use rand::{thread_rng, seq::SliceRandom};
use robotics_lib::{world::tile::Tile, interface::Direction};

use crate::compass::{NLACompassParams, helpers::{in_bounds, Coordinate, TileWithCordinates}, MoveError};

use super::PossibleDirection;

fn cost_tile_entrance (tile: &Tile) -> usize {
    tile.tile_type.properties().cost()
}

fn cost_elevation_diff (curr: &Tile, next: &Tile, params: &NLACompassParams) -> f32 {
    let diff = (next.elevation as i32) - (curr.elevation as i32);
    let uphill = diff >= 0;
    if uphill {
        diff.pow(2) as f32
    } else {
        (diff as f32).pow(params.cost_neg_el_diff_pow)
    }
}

/// Cost associated with moving from tile to another
pub(crate) fn move_cost_estimation (curr: &Tile, next: &Tile, params: &NLACompassParams) -> f32 {
    cost_tile_entrance(next) as f32         // Cost of entering the tile
    + cost_elevation_diff(curr, next, params)     // Cost of elevation difference (both positive and negative)
}

fn get_adjacent_tile<'a> (curr: &Coordinate, map: &'a Vec<Vec<Option<Tile>>>, direction: &Direction) -> Option<TileWithCordinates<'a>> {
    match direction {
        Direction::Left => {
            let row = curr.row;
            let col = curr.col.checked_sub(1);

            col.and_then(|col| {
                map[row][col].as_ref().and_then(|tile| {
                    Some(TileWithCordinates { tile, pos: Coordinate{row, col} })
                })
            })
        },
        Direction::Down => {
            let row = curr.row + 1;
            let col = curr.col;
            if row < map.len() {
                map[row][col].as_ref().and_then(|tile| {
                    Some(TileWithCordinates { tile, pos: Coordinate{row, col} })
                })
            } else {
                None
            }
        },
        Direction::Right => {
            let row = curr.row;
            let col = curr.col + 1;
            if col < map[row].len() {
                map[row][col].as_ref().and_then(|tile| {
                    Some(TileWithCordinates { tile, pos: Coordinate{row, col} })
                })
            } else {
                None
            }
        },
        Direction::Up => {
            let row = curr.row.checked_sub(1);
            let col = curr.col;

            row.and_then(|row| {
                map[row][col].as_ref().and_then(|tile| {
                    Some(TileWithCordinates { tile, pos: Coordinate{row, col} })
                })
            })
        },
    }
}

pub(crate) fn ordered_directions () -> Vec<Direction> {
    vec![Direction::Left, Direction::Down, Direction::Right, Direction::Up]
}

/// Returns left, right, top and bottom adjacent tiles
pub(crate) fn get_adjacent_tiles<'a> (curr: &Coordinate, map: &'a Vec<Vec<Option<Tile>>>) -> Vec<(Direction, Option<TileWithCordinates<'a>>)> {
    ordered_directions().iter().map(|dir| {
        (dir.to_owned(), get_adjacent_tile(curr, &map, dir))  
    }).collect()
}

// Returns number of undiscovered tiles around the tile at position `pos`
pub(crate) fn get_undiscovered_tiles_count (pos: &Coordinate, map: &Vec<Vec<Option<Tile>>>) -> usize {
    let mut discovered = 0;
    for row_off in -1..=1 {
        for col_off in -1..=1 {
            let Some(row) = pos.row.checked_add_signed(row_off) else {
                break;
            };
            let Some(col) = pos.row.checked_add_signed(col_off) else {
                break;
            };
            if in_bounds(map, &Coordinate::new(row, col))
                && map[row][col].is_none() { 
                discovered += 1;
            }
        }
    }
    discovered
}

pub(crate) fn inverse_weighted_choice (directions: &Vec<PossibleDirection>) -> Result<Direction, MoveError> {
    let tot: f32 = directions.iter().map(|PossibleDirection{direction: _, cost, undiscovered: _}| cost).sum();

    let mut rng = thread_rng();
    match directions.choose_weighted(&mut rng, |el| tot/el.cost) {
        Ok(choice) => Ok(choice.direction.clone()),
        Err(_) => Err(MoveError::NoAvailableMove)
    }
  }