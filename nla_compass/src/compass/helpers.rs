use robotics_lib::world::tile::{Tile, Content, TileType};

#[derive(Debug)]
pub(crate) struct TileWithCordinates<'a> {
    pub(crate) tile: &'a Tile,
    pub(crate) pos: Coordinate
}

#[derive(Debug)]
pub(crate) struct Coordinate {
    // We need to implement again coordinate because implementation in robotic lib is private
    pub(crate) row: usize,
    pub(crate) col: usize
}

impl Coordinate {
    pub(crate) fn new(row: usize, col: usize) -> Self {
        Self {
            row, col
        }
    }

    /// Computes Euclidian distance between two coordinates on map
    fn distance(&self, other: &Coordinate) -> f32 {
        f32::sqrt(self.row.abs_diff(other.row).pow(2) as f32 + self.col.abs_diff(other.col).pow(2) as f32)
    }
}

/// Returns coordinate of closest tile with matching content
pub(crate) fn get_closest_content(map: &Vec<Vec<Option<Tile>>>, content: &Content, pos: &Coordinate) -> Option<Coordinate> {
    // Convert 2D vec to list with row and col index
    flatten_2d_map(map).filter_map(|el| {
        // Filter only specified content
        el.and_then(|tile| {
            if tile.tile.content == *content {Some(tile.pos)}
            else { None }
        })
    }).min_by(|el1, el2| {
        // Get content located at minimum distance
        pos.distance(el1).total_cmp(&pos.distance(el2))
    })
}

/// Returns coordinate of closest tile with matching tiletype
pub(crate) fn get_closest_tiletype(map: &Vec<Vec<Option<Tile>>>, tiletype: &TileType, pos: &Coordinate) -> Option<Coordinate> {
    // Convert 2D vec to list with row and col index
    flatten_2d_map(map).filter_map(|el| {
        // Filter only specified content
        el.and_then(|tile| {
            if tile.tile.tile_type == *tiletype {Some(tile.pos)}
            else { None }
        })
    }).min_by(|el1, el2| {
        // Get content located at minimum distance
        pos.distance(el1).total_cmp(&pos.distance(el2))
    })
}

// Converts 2D map to vector of tiles with coordinates field
fn flatten_2d_map(map: &Vec<Vec<Option<Tile>>>) -> impl Iterator<Item = Option<TileWithCordinates>> {
    return map.iter().enumerate().flat_map(|(row, el)| {
        el.iter().enumerate().map(move |(col, el)| {
            el.as_ref().and_then(|tile| {
                Some(TileWithCordinates {
                    pos: Coordinate::new(row, col),
                    tile
                })
            })
        })
    });
}

/// Returns `true` if passed coords are inside world
pub(crate) fn in_bounds(map: &Vec<Vec<Option<Tile>>>, coord: &Coordinate) -> bool {
    coord.row < map.len() && coord.col < map[0].len()
}