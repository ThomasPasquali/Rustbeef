use std::any::TypeId;

use robotics_lib::{interface::{Tools, Direction, robot_view, robot_map, where_am_i},
                   world::{World, tile::{Content, Tile, TileType}, coordinates::Coordinate},
                   runner::{Robot, Runnable}};


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
pub enum Destination {
    /// Content (content, min_r, new)
    CONTENT(Content, Option<usize>, Option<bool>),
    /// Tile type (tiletype, min_r, new)
    TILE_TYPE(TileType, Option<usize>, Option<bool>),
    /// Coordinate (coordinate)
    COORDINATE(Coordinate),
}

struct TileCost {
    tile: Tile,
    cost: usize
}

pub struct NLACompass {
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
    pub fn new (world: Option<Vec<Vec<Option<Tile>>>>) -> Self {
        NLACompass { destination: None }
    }

    fn get_move_for_content (&self, c: &Content, min_r: &Option<usize>, new: &Option<bool>) -> Option<Direction> {
        // TODO
        Some(Direction::Up)
    }

    fn get_move_for_tiletype (&self, t: &TileType, min_r: &Option<usize>, new: &Option<bool>) -> Option<Direction> {
        Some(Direction::Up)
    }

    fn get_move_for_coordinate (&self, c: &Coordinate) -> Option<Direction> {
        Some(Direction::Up)
    }

    fn costs_around_me(robot: &impl Runnable, world: &World) -> Option<Vec<Vec<Option<Tile>>>> {
        //let (view, curr) = where_am_i(robot, world);
        //for tiles in view
        return None;
    }

    pub fn get_move(&self, robot: &impl Runnable, world: &World) -> Option<Direction> {
        if self.destination.is_none() {
            return None;
        }

        let map = robot_map(world);
        if map.is_none() {
            return None;
        }

        let map = map.unwrap();
        // let view = robot_view(robot, world);
        let (view, curr) = where_am_i(robot, world);

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