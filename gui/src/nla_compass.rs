use std::any::TypeId;

use robotics_lib::{interface::{Tools, Direction}, world::{World, tile::{Content, Tile}, coordinates::Coordinate}};

pub enum Destination {
    CONTENT(Content),
    COORDINATE(Coordinate),
    // TILE_TYPE(TileType)
}

pub struct NLACompass {
    world: Option<Vec<Vec<Option<Tile>>>>,
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
    fn new (world: Option<Vec<Vec<Option<Tile>>>>) -> Self {
        NLACompass { world, destination: None }
    }
    fn get_move(&self) -> Option<Direction> {
        if self.destination.is_none() {
            return None;
        }

        Some(Direction::Up)
    }
    fn set_destination(&mut self, destination: Destination) {
        self.destination = Some(destination);
    }
}