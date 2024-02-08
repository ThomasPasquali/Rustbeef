use robotics_lib::{world::tile::Tile, interface::Direction};

use crate::{compass::{MoveError, NLACompassParams, helpers::{Coordinate, TileWithCordinates}}, probabilistic_model::helpers::{get_adjacent_tiles, inverse_weighted_choice}};

mod helpers;

#[derive(Debug)]
pub struct PossibleDirection {
    direction: Direction,
    cost: f32,
    undiscovered: usize
}

pub(crate) fn get_move(robot_map: &Vec<Vec<Option<Tile>>>, curr_pos: &Coordinate, params: &NLACompassParams) -> Result<Direction, MoveError> {

    let adj_tiles = get_adjacent_tiles(curr_pos, robot_map);
    println!("Adjacent tiles: {:?}", &adj_tiles);

    // Vector containing cost and number of undiscovered tiles that can be reached
    let mut possible_directions: Vec<PossibleDirection> = adj_tiles.iter().filter_map(|next| {
        get_possible_direction(robot_map, curr_pos, params, next)
    }).collect();

    println!("Costs + discover {:?}", &possible_directions);

    let cost_tot: f32 = possible_directions.iter()
        .map(|PossibleDirection{direction: _, cost, undiscovered: _}| cost)
        .sum();

    for c in possible_directions.iter_mut() {
        // Add cost given by number of reachable undiscovered tiles
        // Cost inversly proportional to number of undiscovered tiles: more undiscovered -> smaller cost
        let cost_undiscovered = cost_tot / (c.undiscovered + params.cost_disc_tiles_proportion) as f32;
        c.cost = (c.cost + cost_undiscovered).powi(3);
    }

    inverse_weighted_choice(&possible_directions)
}

// Returns cost to go to that tile and number of undiscovered tiles that can be reached from that tile
fn get_possible_direction(robot_map: &Vec<Vec<Option<Tile>>>, curr_pos: &Coordinate, params: &NLACompassParams, tile_with_dir: &Option<TileWithCordinates>) -> Option<PossibleDirection> {
    let tile_with_dir = tile_with_dir.as_ref()?;
    let tile = tile_with_dir.tile;
    let tile_pos = &tile_with_dir.pos;
    let curr = robot_map[curr_pos.row][curr_pos.col].as_ref()?;

    let undiscovered = helpers::get_undiscovered_tiles_count(tile_pos, robot_map);

    if undiscovered > 0 {
        return Some(PossibleDirection {
            direction: Direction::Up,
            cost: helpers::move_cost_estimation(curr, tile, params),
            undiscovered
        });
    }else{
        // If all adjacent tiles have been already discovered, assign a high cost for this direction
        return Some(PossibleDirection {
            direction: Direction::Up,
            cost: 20.0,
            undiscovered: 0
        });
    }
}