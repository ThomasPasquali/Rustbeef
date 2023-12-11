use pathfinding::num_traits::Pow;
use crate::probabilistic_choice::ProbabilisticChoice;
// FIXME use crate::dijkstra::dijkstra_path;
use robotics_lib::{interface::{Tools, Direction},
                   world::{tile::{Content, Tile, TileType}, coordinates::Coordinate}};


/// Compass destination
/// 
/// (at least seen)
/// # Usage
/// ```rust
/// ```
///
/// # Examples
/// ```rust
/// 
/// ```
#[derive(Clone)]
pub enum Destination {
    /// Content (content, min_r, new)
    CONTENT(Content, Option<usize>, Option<bool>),
    /// Tile type (tiletype, min_r, new)
    TILE_TYPE(TileType, Option<usize>, Option<bool>),
    /// Coordinate (coordinate)
    COORDINATE(Coordinate),
}

#[derive(Debug)]
struct TileCordinates<'a> {
    tile: Option<&'a Tile>,
    pos: (usize, usize)
}

#[derive(Clone)]
pub struct NLACompassParams {
    COST_NEG_EL_DIFF_POW: f32,
    COST_NEXT_NEXT_POW: f32,
    COST_DISC_TILES_PROPORTION: usize
}
impl Default for NLACompassParams {
    fn default() -> Self {
        NLACompassParams {
            COST_NEG_EL_DIFF_POW: 3.0 / 2.0, // 1.5
            COST_NEXT_NEXT_POW: 1.0 / 2.0,   // 0.5 (sqrt)
            COST_DISC_TILES_PROPORTION: 1    // CANNNOT be 0
        }
    }
}


#[derive(Clone)]
pub struct NLACompass {
    params: NLACompassParams,
    destination: Option<Destination>
}

impl Tools for NLACompass { }

impl NLACompass {
    pub fn new () -> Self {
        NLACompass { destination: None, params: NLACompassParams::default() }
    }

    pub fn set_params (&mut self, params: NLACompassParams) {
        self.params = params;
    }

    fn cost_elevation_diff (&self, curr: &Tile, next: &Tile) -> usize {
        let diff = (next.elevation as i32) - (curr.elevation as i32);
        let uphill = diff >= 0;
        if uphill {
            diff.pow(2) as usize
        } else {
            (diff as f32).pow(self.params.COST_NEG_EL_DIFF_POW) as usize
        }
    }

    fn cost_tile_entrance (&self, tile: &Tile) -> usize {
        tile.tile_type.properties().cost()
    }

    pub fn move_cost_estimation (&self, curr: &Tile, next: &Tile) -> usize {
        println!("Cost estimation: next_in {}, el_diff {}", self.cost_tile_entrance(next), self.cost_elevation_diff(curr, next));
        self.cost_tile_entrance(next)       // Cost of entering the tile
        + self.cost_elevation_diff(curr, next)    // Cost of elevation difference (both positive and negative)
        // TODO check if can add   next_type_cost
        // TODO cost_next_next
    }

    fn get_adjacent_tile<'a> (curr: (usize, usize), map: &'a Vec<Vec<Option<Tile>>>, direction: &Direction) -> Result<Option<TileCordinates<'a>>, String> {
        if curr.0 >= map.len() || curr.1 >= map[0].len() {
            return Err("Invalud curr parameter".to_string());
        }
        match direction {
            Direction::Left => {
                let x = curr.0;
                let y = curr.1.checked_sub(1);
                match y {
                    Some(y) => Ok(Some(TileCordinates { tile: map[x][y].as_ref(), pos: (x, y) })),
                    None => Err("y underflow".to_string()),
                }
            },
            Direction::Down => {
                let x = curr.0 + 1;
                let y = curr.1;
                if x < map.len() { 
                    Ok(Some(TileCordinates { tile: map[x][y].as_ref(), pos: (x, y) }))
                } else {
                    Err("x overflow".to_string())
                }
            },
            Direction::Right => {
                let x = curr.0;
                let y = curr.1 + 1;
                if y < map[x].len() {
                    Ok(Some(TileCordinates { tile: map[x][y].as_ref(), pos: (x, y) }))
                } else {
                    Err("y overflow".to_string())
                }
            },
            Direction::Up => {
                let x = curr.0.checked_sub(1);
                let y = curr.1;
                match x {
                    Some(x) => Ok(Some(TileCordinates { tile: map[x][y].as_ref(), pos: (x, y) })),
                    None => Err("x underflow".to_string()),
                }
            },
        }
        
    }

    fn ordered_directions () -> Vec<Direction> {
        vec![Direction::Left, Direction::Down, Direction::Right, Direction::Up]
    }

    fn get_adjacent_tiles<'a> (curr: (usize, usize), map: &'a Vec<Vec<Option<Tile>>>) -> Vec<Result<Option<TileCordinates<'a>>, String>> {
        NLACompass::ordered_directions().iter().map(|dir| {
            NLACompass::get_adjacent_tile(curr, &map, dir)     
        }).collect()
    }

    fn pos_in_bounds (pos: (i32, i32), x_bound: usize, y_bound: usize) -> bool {
        pos.0 >= 0 && pos.1 >=0 && pos.0 < x_bound as i32 && pos.1 < y_bound as i32
    }

    fn get_move_discover_tiles_count<'a> (pos: (usize, usize), map: &'a Vec<Vec<Option<Tile>>>) -> usize {
        // .iter().filter(|x| x.is_ok() && (x.as_ref().unwrap().is_none() || x.as_ref().unwrap().as_ref().unwrap().tile.is_none())).count()
        let mut dicovered = 0;
        for x_off in -1..=1 {
            for y_off in -1..=1 {
                let disc_pos: (i32, i32) = (pos.0 as i32 + x_off, pos.1 as i32 + y_off);
                if NLACompass::pos_in_bounds(disc_pos, map.len(), map[0].len()) // Only works with square maps
                    && map[disc_pos.0 as usize][disc_pos.1 as usize].is_none() { 
                    dicovered += 1;
                }
            }
        }
        dicovered
    }

    fn get_move_for_content (&self, c: &Content, min_r: &Option<usize>, new: &Option<bool>) -> Option<Direction> {
        // TODO
        Some(Direction::Up)
    }

    fn get_move_for_tiletype (&self, t: &TileType, min_r: &Option<usize>, new: &Option<bool>) -> Option<Direction> {
        // TODO
        Some(Direction::Up)
    }

    fn get_move_for_coordinate (&self, c: &Coordinate) -> Option<Direction> {
        // TODO
        Some(Direction::Up)
    }

    pub fn get_move(&self, map: &Option<Vec<Vec<Option<Tile>>>>, surroundings: &Vec<Vec<Option<Tile>>>, curr_pos: (usize, usize)) -> Option<Direction> {
        if self.destination.is_none() {
            return None;
        }
        if map.is_none() {
            return None;
        }
        let robot_map = map.as_ref().unwrap();
        // FIXME this is just a test for destination unknown moves

        // Adjacent tiles
        let next_tiles = NLACompass::get_adjacent_tiles(curr_pos, robot_map);
        println!("Next tile: {:?}", &next_tiles);

        // Directions base costs
        let move_costs_and_cells_to_discover: Vec<Option<(usize, usize)>> = next_tiles.iter().map(|next| {
            if next.is_err() { return None; }
            let next = next.as_ref().unwrap().as_ref()?;
            let next_tile = next.tile?;
            let next_pos = next.pos;
            let curr = robot_map[curr_pos.0][curr_pos.1].as_ref()?;
            println!("Discover {:?}: {:?} ({})", next_pos, NLACompass::get_adjacent_tiles(next_pos, robot_map), NLACompass::get_adjacent_tiles(next_pos, robot_map).iter().filter(|x| x.is_ok() && (x.as_ref().unwrap().is_none() || x.as_ref().unwrap().as_ref().unwrap().tile.is_none())).count());
            println!("\n");
            let discover = NLACompass::get_move_discover_tiles_count(next_pos, robot_map);
            if discover > 0 { // Removing move that do not discover
                Some((
                self.move_cost_estimation(curr, next_tile),
                discover 
            )) } else { None }
        }).collect();
        println!("Costs + discover {:?}", &move_costs_and_cells_to_discover);

        let mut costs: Vec<Option<usize>> = vec![];
        let cost_tot: usize = move_costs_and_cells_to_discover.iter()
            .filter_map(|c| c.map(|(cost, _)| cost))
            .sum();
        for c in move_costs_and_cells_to_discover.iter() {
            let mut cost = None;
            if c.is_some() {
                let c = c.unwrap();
                cost = Some(c.0 + (cost_tot / (c.1 + self.params.COST_DISC_TILES_PROPORTION)));
            }
            costs.push(cost);
        }
        println!("Costs {:?}", &costs);

        // Change costs proportionality
        costs = costs.into_iter().map(|c| match c {
            Some(cost) => Some(cost.pow(3)),
            None => None
        }).collect();

        let choice = ProbabilisticChoice::inverse_wheighted_choice(&costs);
        match choice {
            Ok(direction_i) => {
                let mut i: usize = 0;
                let mut j: usize = 0;
                let mut directions = NLACompass::ordered_directions();
                // Remove impossible moves
                while i < directions.len() && j < move_costs_and_cells_to_discover.len() {
                    if move_costs_and_cells_to_discover[j].is_none() {
                        directions.remove(i);
                    } else {
                        i += 1;
                    }
                    j += 1;
                }
                let direction = directions[direction_i].clone();
                println!("Choice: {:?}, idx {}  (estimated cost + discover: {:?})", &direction, direction_i, &move_costs_and_cells_to_discover[if j >= 4 {directions.len()} else {j}]);
                return Some(direction);
            },
            Err(e) => {
              println!("{e}");
              return None;
            }
        }

        // match self.destination.as_ref().unwrap() {
        //     Destination::CONTENT(c, min_r, new) => self.get_move_for_content(c, min_r, new),
        //     Destination::TILE_TYPE(c, min_r, new) => self.get_move_for_tiletype(c, min_r, new),
        //     Destination::COORDINATE(c) => self.get_move_for_coordinate(c)
        // }
    }

    pub fn set_destination(&mut self, destination: Destination) {
        self.destination = Some(destination);
    }
}