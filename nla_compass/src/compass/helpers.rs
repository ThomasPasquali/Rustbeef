use robotics_lib::world::tile::{Tile, Content};

use super::NLACompassParams;

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
    map.iter().enumerate().flat_map(|(row, el)| {
        el.iter().enumerate().map(move |(col, el)| {
            el.as_ref().and_then(|tile| {
                Some(TileWithCordinates {
                    pos: Coordinate::new(row, col),
                    tile: tile
                })
            })
        })
    }).filter_map(|el| {
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

/// Returns `true` if passed coords are inside world
pub(crate) fn in_bounds(map: &Vec<Vec<Option<Tile>>>, coord: &Coordinate) -> bool {
    coord.row < map.len() && coord.col < map[0].len()
}

pub(crate) fn cost_tile_entrance (tile: &Tile) -> usize {
    tile.tile_type.properties().cost()
}

pub(crate) fn cost_elevation_diff (curr: &Tile, next: &Tile, params: &NLACompassParams) -> f32 {
    let diff = (next.elevation as i32) - (curr.elevation as i32);
    let uphill = diff >= 0;
    if uphill {
        diff.pow(2) as f32
    } else {
        (diff as f32).powf(params.cost_neg_el_diff_pow)
    }
}