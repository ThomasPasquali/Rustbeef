use log::debug;
use robotics_lib::{world::tile::Tile, interface::Direction};

use crate::{compass::{MoveError, NLACompassParams, helpers::{TileWithCordinates, Coordinate}}, probabilistic_model::{helpers::get_adjacent_tiles, probabilistic_choice::ProbabilisticChoice}};

use self::helpers::TileWithDirection;

mod probabilistic_choice;
mod helpers;

#[derive(Debug)]
struct Choice {
    direction: Direction,
    cost: f32,
    next_undiscovered: usize
}

pub(crate) fn get_move(robot_map: &Vec<Vec<Option<Tile>>>, curr_pos: &Coordinate, params: &NLACompassParams) -> Result<Direction, MoveError> {

    let adj_tiles = get_adjacent_tiles(curr_pos, robot_map);
    debug!("Adjacent tiles: {:?}", &adj_tiles);

    // Vector containing cost and number of undiscovered tiles that can be reached
    let mut move_costs_and_tiles_to_discover: Vec<Choice> = adj_tiles.iter().filter_map(|next| {
        get_choice(robot_map, curr_pos, params, next)
    }).collect();

    debug!("Costs + discover {:?}", &move_costs_and_tiles_to_discover);

    let cost_tot: f32 = move_costs_and_tiles_to_discover.iter()
        .map(|Choice{direction: _, cost, next_undiscovered: _}| cost)
        .sum();

    for c in move_costs_and_tiles_to_discover.iter_mut() {
        // Add cost given by number of reachable undiscovered tiles
        c.cost = (c.cost + (cost_tot / (c.next_undiscovered + params.cost_disc_tiles_proportion) as f32)).powi(3);
    }

    let choice = ProbabilisticChoice::inverse_weighted_choice(&costs);
    match choice {
        Ok(direction_i) => {
            let mut i: usize = 0;
            let mut j: usize = 0;
            let mut directions = helpers::ordered_directions();
            // Remove impossible moves
            while i < directions.len() && j < move_costs_and_tiles_to_discover.len() {
                if move_costs_and_tiles_to_discover[j].is_none() {
                    directions.remove(i);
                } else {
                    i += 1;
                }
                j += 1;
            }
            let direction = directions[direction_i].clone();
            // println!("Choice: {:?}, idx {}  (estimated cost + discover: {:?})", &direction, direction_i, &move_costs_and_cells_to_discover[if j >= 4 {directions.len()} else {j}]);
            return Some(direction);
        },
        Err(e) => {
          println!("{e}");
          return None;
        }
    }
}

// Returns cost to go to that tile and number of undiscovered tiles that can be reached from that tile
fn get_choice(robot_map: &Vec<Vec<Option<Tile>>>, curr_pos: &Coordinate, params: &NLACompassParams, tile_with_coords: &Option<TileWithDirection>) -> Option<Choice> {
    let tile_with_coords = tile_with_coords.as_ref()?;
    let tile = tile_with_coords.tile;
    let tile_pos = tile_with_coords.pos;
    let curr = robot_map[curr_pos.row][curr_pos.col].as_ref()?;

    let undiscovered = helpers::get_undiscovered_tiles_count(tile_pos, robot_map);

    if undiscovered > 0 {
        return Some(Choice {
            direction: Direction::Up,
            cost: helpers::move_cost_estimation(curr, tile, params),
            next_undiscovered: undiscovered
        });
    }else{
        // If all adjacent tiles have been already discovered, assign a high cost for this direction
        return Some(Choice {
            direction: Direction::Up,
            cost: 20.0,
            next_undiscovered: 0
        });
    }
}