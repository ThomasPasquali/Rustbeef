use robotics_lib::{world::tile::Tile, interface::Direction};

use crate::{compass::{helpers::{Coordinate, TileWithCordinates}, MoveError, NLACompassParams}, probabilistic_model::helpers::{get_adjacent_tiles, get_opposite_direction, inverse_weighted_choice}};

use self::helpers::getting_closer_to_destination_coords;

mod helpers;

#[derive(Debug)]
pub(crate) struct PossibleDirection {
    direction: Direction,
    cost: f32,
    undiscovered: usize,
    tiles_until_undiscovered: usize
}

pub(crate) fn get_move(robot_map: &Vec<Vec<Option<Tile>>>, curr_pos: &Coordinate, destination_coords: Option<&Coordinate>, params: &NLACompassParams, last_move: &Option<Direction>) -> Result<Direction, MoveError> {

    let adj_tiles = get_adjacent_tiles(curr_pos, robot_map);
    // println!("Adjacent tiles: {:#?}", &adj_tiles);

    // Vector containing cost and number of undiscovered tiles that can be reached
    let mut possible_directions: Vec<PossibleDirection> = adj_tiles.iter().filter_map(|next| {
        if let Some(lm) = last_move {
            if next.0 == get_opposite_direction(lm) {
                return None;
            }
        }
        get_possible_direction(robot_map, curr_pos, params, next)
    }).collect();

    // println!("Costs + discover {:#?}", &possible_directions);

    let cost_tot: f32 = possible_directions.iter()
        .map(|poss_dir| poss_dir.cost)
        .sum();

    // Gather all costs estimation into `cost`
    for c in possible_directions.iter_mut() {
        // Add cost given by number of reachable undiscovered tiles
        // Cost inversly proportional to number of undiscovered tiles: more undiscovered -> smaller cost
        let cost_undiscovered = cost_tot / (c.undiscovered + params.cost_disc_tiles_proportion) as f32;
        c.cost = (c.cost + cost_undiscovered).powi(3);

        // Add cost given by how far is from undiscovered tiles
        c.cost += (c.tiles_until_undiscovered as f32).powf(params.dist_from_undiscovered_pow);

        // If going to coordinates consider if getting closer
        if let Some(dest) = destination_coords {
            if !getting_closer_to_destination_coords(curr_pos, dest, &c.direction) {
                // println!("Applied getting further multiplier!");
                c.cost = c.cost.powf(params.getting_further_to_coords_pow);
            }
        }

        // println!("Costs: {:?} {:?}", c.direction, c.cost);
    }

    inverse_weighted_choice(&possible_directions)
}

// Returns cost to go to that tile and number of undiscovered tiles that can be reached from that tile
fn get_possible_direction(robot_map: &Vec<Vec<Option<Tile>>>, curr_pos: &Coordinate, params: &NLACompassParams, tile_with_dir: &(Direction, Option<TileWithCordinates>)) -> Option<PossibleDirection> {
    let direction = tile_with_dir.0.to_owned();
    let tile_with_dir = tile_with_dir.1.as_ref()?;
    let tile = tile_with_dir.tile;
    let tile_pos = &tile_with_dir.pos;
    let curr = robot_map[curr_pos.row][curr_pos.col].as_ref()?;
    let tiles_until_undiscovered = helpers::get_tiles_count_until_undiscovered(curr_pos, robot_map, &direction);

    if !tile.tile_type.properties().walk() {
        None
    } else {
        Some(PossibleDirection {
            direction,
            cost: helpers::move_cost_estimation(curr, tile, params),
            undiscovered: helpers::get_undiscovered_tiles_count(tile_pos, robot_map),
            tiles_until_undiscovered
        })
    }    
}
