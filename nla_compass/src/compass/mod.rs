use std::any::TypeId;

use pathfinding::num_traits::Pow;
use crate::probabilistic_choice::ProbabilisticChoice;
use crate::dijkstra;
use robotics_lib::{interface::{Tools, Direction, robot_view, robot_map, where_am_i},
                   world::{World, tile::{Content, Tile, TileType}, coordinates::Coordinate}, runner::Runnable};


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

impl Tools for NLACompass {
    fn check(&self, world: &mut World) -> Result<(), robotics_lib::utils::LibError> {
        Ok(())
    }
    fn id(&self) -> TypeId {
        TypeId::of::<NLACompass>()
    }
}

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

    fn move_cost (&self, curr: &Tile, next: &Tile) -> usize {
        next.tile_type.properties().cost()              // Cost of entering the tile
        + self.cost_elevation_diff(curr, next)          // Cost of elevation difference (both positive and negative)
        // TODO check if can add next_type_cost
        // TODO cost_next_next
    }

    fn get_adjacent_tile<'a> (curr: (usize, usize), view: &'a Vec<Vec<Option<Tile>>>, direction: &Direction) -> Option<TileCordinates<'a>> {
        match direction {
            Direction::Up => {
                let x = curr.0;
                let y = curr.1.checked_sub(1)?;
                Some(TileCordinates { tile: view[x][y].as_ref(), pos: (x, y) })
            },
            Direction::Right => {
                let x = curr.0 + 1;
                let y = curr.1;
                if x < view.len() { Some(TileCordinates { tile: view[x][y].as_ref(), pos: (x, y) }) } else { None }
            },
            Direction::Down => {
                let x = curr.0;
                let y = curr.1 + 1;
                if y < view[x].len() { Some(TileCordinates { tile: view[curr.0][y].as_ref(), pos: (x, y) }) } else { None }
            },
            Direction::Left => {
                let x = curr.0.checked_sub(1)?;
                let y = curr.1;
                Some(TileCordinates { tile: view[x][y].as_ref(), pos: (x, y) })
            },
        }
        
    }

    fn get_adjacent_tiles<'a> (curr: (usize, usize), view: &'a Vec<Vec<Option<Tile>>>) -> Vec<Option<TileCordinates<'a>>> {
        vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left].iter().map(|dir| {
            NLACompass::get_adjacent_tile(curr, &view, dir)     
        }).collect()
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

        // Adjacent tiles
        let next_tiles = NLACompass::get_adjacent_tiles(curr_pos, surroundings);

        // Directions base costs
        let move_costs_and_cells_to_discover: Vec<Option<(usize, usize)>> = next_tiles.iter().map(|next| {
            let next = next.as_ref()?;
            let next_tile = next.tile?;
            let next_pos = next.pos;
            let curr = surroundings[curr_pos.0][curr_pos.1].as_ref()?;
            Some((
                self.move_cost(curr, next_tile),
                NLACompass::get_adjacent_tiles(next_pos, &surroundings).iter().filter(|x| x.is_some()).count()
            ))
        }).collect();

        let mut costs: Vec<Option<usize>> = vec![];
        let cost_tot: usize = move_costs_and_cells_to_discover.iter()
            .filter_map(|c| c.map(|(cost, _)| cost))
            .sum();

        for c in move_costs_and_cells_to_discover {
            let mut cost = None;
            if c.is_some() {
                let c = c.unwrap();
                cost = Some(c.0 + (cost_tot / (c.1 + self.params.COST_DISC_TILES_PROPORTION)));
            }
            costs.push(cost);
        }

        // TODO eventually change costs proportionality

        let choice = ProbabilisticChoice::inverse_wheighted_choice(&costs);
        match choice {
            Ok(c) => {
                println!("{:?}", &costs);
                println!("Choice: {}", c);
            },
            Err(e) => {
              println!("{e}")
            }
        }

        match self.destination.as_ref().unwrap() {
            Destination::CONTENT(c, min_r, new) => self.get_move_for_content(c, min_r, new),
            Destination::TILE_TYPE(c, min_r, new) => self.get_move_for_tiletype(c, min_r, new),
            Destination::COORDINATE(c) => self.get_move_for_coordinate(c)
        }
    }

    pub fn set_destination(&mut self, destination: Destination) {
        self.destination = Some(destination);
    }
}