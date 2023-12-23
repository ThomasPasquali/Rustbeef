use crate::dijkstra;
use robotics_lib::{interface::{Tools, Direction},
                   world::tile::{Content, Tile, TileType}};

use crate::{compass::helpers::in_bounds, probabilistic_model};

use self::helpers::{get_closest_content, Coordinate, get_closest_tiletype};

// Helpers for compass tool
pub(crate) mod helpers;

/// Defines the destination of the compass.
#[derive(Clone)]
pub enum Destination {
    /// Content (content, explore_new). Quantity field is ignored.
    Content(Content, bool),
    /// Tile type (tiletype, explore_new)
    TileType(TileType, bool),
    /// Coordinate (coordinate, explore_new)
    Coordinate((usize, usize), bool),
}

/// Errors returned by `getMove()`.
pub enum MoveError {
    /// The destination has not been set
    NoDestination,
    /// There is no matching content in the explored world
    NoContent,
    /// There is no matching tiletype in the explored world
    NoTileType,
    /// The current position does not point to an explored tile
    InvalidCurrPosition,
    /// The destination coordinate is invalid
    InvalidDestCoordinate,
    /// The algorithm could not find any move to make
    NoAvailableMove,
    /// The robot is at the destination already
    AlreadyAtDestination,
    /// The functionality has not been implemented yet :(
    NotImplemented
}

/// Advanced configuration options for tuning weigths.
#[derive(Clone)]
pub struct NLACompassParams {
    /// Cost assigned for going downhill. Used so that the robot avoids losing the elevation potential it gained. Defaults to 1.5.
    pub cost_neg_el_diff_pow: f32,
    /// Cost assigned to the number of undiscovered tiles for the next move. Defaults to 1. CANNOT be 0.
    pub cost_disc_tiles_proportion: usize
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
    /// Initilizes a new compass with the default parameters.
    pub fn new () -> Self {
        NLACompass { destination: None, params: NLACompassParams::default() }
    }

    /// Sets advanced configuration parameters.
    pub fn set_params (&mut self, params: NLACompassParams) {
        self.params = params;
    }

    /// Returns the advanced configuration parameters.
    pub fn get_params (&self) -> &NLACompassParams {
        &self.params
    }

    /// Sets the destination of the compass.
    pub fn set_destination (&mut self, destination: Destination) {
        self.destination = Some(destination);
    }

    /// Returns the destination of the compass.
    pub fn get_destination (&self) -> &Option<Destination> {
        &self.destination
    }

    /// Clears the destination of the compass (sets it to `None`).
    pub fn clear_destination(&mut self) {
        self.destination = None;
    }

    fn get_move_for_content (&mut self, map: &Vec<Vec<Option<Tile>>>, c: &Content, explore_new: bool, curr_pos: &Coordinate) -> Result<Direction, MoveError> {
        // Check if we have already reached the destination
        if map[curr_pos.row][curr_pos.col].as_ref().ok_or(MoveError::InvalidCurrPosition)?.content == *c {
            self.destination = None;
            Err(MoveError::AlreadyAtDestination)
        } else {
            if explore_new {
                probabilistic_model::get_move(map, curr_pos, &self.params)
            } else {
                let coordinate = get_closest_content(map, c, curr_pos).ok_or(MoveError::NoContent)?;
                self.get_move_for_coordinate(map, &coordinate, false, curr_pos)
            }
        }
    }

    fn get_move_for_tiletype (&mut self, map: &Vec<Vec<Option<Tile>>>, t: &TileType, explore_new: bool, curr_pos: &Coordinate) -> Result<Direction, MoveError> {
        if map[curr_pos.row][curr_pos.col].as_ref().ok_or(MoveError::InvalidCurrPosition)?.tile_type == *t {
            self.destination = None;
            Err(MoveError::AlreadyAtDestination)
        } else {
            if explore_new {
                probabilistic_model::get_move(map, curr_pos, &self.params)
            } else {
                let coordinate = get_closest_tiletype(map, t, curr_pos).ok_or(MoveError::NoTileType)?;
                self.get_move_for_coordinate(map, &coordinate, false, curr_pos)
            }
        }
    }

    fn get_move_for_coordinate (&mut self, map: &Vec<Vec<Option<Tile>>>, c: &Coordinate, _explore_new: bool, curr_pos: &Coordinate) -> Result<Direction, MoveError> {
        if !in_bounds(map, &c) {
            return Err(MoveError::InvalidDestCoordinate)
        }
        if curr_pos.row == c.row && curr_pos.col == c.col {
            self.destination = None;
            Err(MoveError::AlreadyAtDestination)
        } else {
            dijkstra::get_move_for_coordinate((curr_pos.row, curr_pos.col), (c.row, c.col), map)
        }
    }

    /// Returns best direction according to set destination and parameters.
    pub fn get_move(&mut self, map: &Vec<Vec<Option<Tile>>>, curr_pos: (usize, usize)) -> Result<Direction, MoveError> {
        let curr_pos = Coordinate::new(curr_pos.0, curr_pos.1);
        let destination = self.destination.clone().ok_or(MoveError::NoDestination)?;
        
        if !in_bounds(map, &curr_pos) {
            return Err(MoveError::InvalidCurrPosition)
        }

        match destination {
            Destination::Content(c, explore_new) => self.get_move_for_content(map, &c, explore_new, &curr_pos),
            Destination::TileType(t, explore_new) => self.get_move_for_tiletype(map, &t, explore_new, &curr_pos),
            Destination::Coordinate(c, explore_new) => self.get_move_for_coordinate(map, &Coordinate::new(c.0, c.1), explore_new, &curr_pos)
        }
    }
}