// FIXME use crate::dijkstra::dijkstra_path;
use robotics_lib::{interface::{Tools, Direction},
                   world::{tile::{Content, Tile, TileType}, coordinates::Coordinate as RoboticCoord}};

use crate::{compass::helpers::in_bounds, probabilistic_model};

use self::helpers::{get_closest_content, Coordinate, get_closest_tiletype};

// Helpers for compass tool
pub(crate) mod helpers;

/// Defines the destination of the compass.
/// If the last field of the variant is set to `true`, the compass will also explore new tiles
/// it hasn't alredy been to.
/// Otherwise, it will always stick to tiles it has already explored (if the destination is
/// unreachable you'll get an error when you call `get_move()`).
pub enum Destination {
    /// Content (content, min_amount, explore_new)
    /// DISCUSSION NEEDED: content already contains information about number of items
    Content(Content, bool),
    /// Tile type (tiletype, explore_new)
    TileType(TileType, bool),
    /// Coordinate (coordinate, explore_new)
    Coordinate((usize, usize), bool),
}

pub enum MoveError {
    // The destination has not been set
    NoDestination,
    // There is no matching content in the explored world
    NoContent,
    // There is no matching tiletype in the explored world
    NoTileType,
    // The current position does not point to an explored tile
    InvalidCurrPosition,
    // The destination coordinate is invalid
    InvalidDestCoordinate,
    // The algorithm could not find any move to make
    NoAvailableMove,
    // The functionality has not been implemented yet :(
    NotImplemented
}

#[derive(Clone)]
pub struct NLACompassParams {
    // Cost assigned for going downhill. Used so that the robot avoids losing the elevation potential it gained
    pub(crate) cost_neg_el_diff_pow: f32,
    // Cost assigned to the importance of the tiles after this move
    pub(crate) cost_disc_tiles_proportion: usize
}
impl Default for NLACompassParams {
    fn default() -> Self {
        NLACompassParams {
            cost_neg_el_diff_pow: 3.0 / 2.0, // 1.5
            cost_disc_tiles_proportion: 1    // CANNNOT be 0
        }
    }
}

/// Represents the compass tool.
/// See the `Destination` enum for information on how to configure the destination.
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
    pub fn get_params (&self) -> &NLACompassParams {
        &self.params
    }
    pub fn set_destination (&mut self, destination: Destination) {
        self.destination = Some(destination);
    }
    pub fn get_destination (&self) -> &Option<Destination> {
        &self.destination
    }
    pub fn clear_destination(&mut self) {
        self.destination = None;
    }

    fn get_move_for_content (&self, map: &Vec<Vec<Option<Tile>>>, c: &Content, explore_new: bool, dst: &Destination, curr_pos: &Coordinate) -> Result<Direction, MoveError> {
        if explore_new {
            probabilistic_model::get_move(map, curr_pos, &self.params)
        } else {
            let coordinate = get_closest_content(map, c, curr_pos).ok_or(MoveError::NoContent);
            // TODO Dijkstra
            Err(MoveError::NotImplemented)
        }
    }

    fn get_move_for_tiletype (&self, map: &Vec<Vec<Option<Tile>>>, t: &TileType, explore_new: bool, dst: &Destination, curr_pos: &Coordinate) -> Result<Direction, MoveError> {
        if explore_new {
            probabilistic_model::get_move(map, curr_pos, &self.params)
        } else {
            let coordinate = get_closest_tiletype(map, t, curr_pos).ok_or(MoveError::NoTileType);
            // TODO Dijkstra
            Err(MoveError::NotImplemented)
        }
    }

    fn get_move_for_coordinate (&self, map: &Vec<Vec<Option<Tile>>>, c: &(usize, usize), explore_new: bool, dst: &Destination) -> Result<Direction, MoveError> {
        let c = Coordinate::new(c.0, c.1);
        if !in_bounds(map, &c) {
            return Err(MoveError::InvalidDestCoordinate)
        }
        // TODO Dijkstra
        Err(MoveError::NotImplemented)
    }

    /// Returns best direction according to set destination and parameters
    pub fn get_move(&self, map: &Vec<Vec<Option<Tile>>>, curr_pos: &RoboticCoord) -> Result<Direction, MoveError> {
        let curr_pos = Coordinate::new(curr_pos.get_row(), curr_pos.get_col());
        let destination = self.destination.as_ref().ok_or(MoveError::NoDestination)?;
        
        if !in_bounds(map, &curr_pos) || map[curr_pos.row][curr_pos.col].is_none() {
            return Err(MoveError::InvalidCurrPosition)
        }

        match destination {
            Destination::Content(c, explore_new) => self.get_move_for_content(map, &c, *explore_new, &destination, &curr_pos),
            Destination::TileType(t, explore_new) => self.get_move_for_tiletype(map, &t, *explore_new, &destination, &curr_pos),
            Destination::Coordinate(c, explore_new) => self.get_move_for_coordinate(map, &c, *explore_new, &destination)
        }
    }
}