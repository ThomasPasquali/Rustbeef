use std::collections::HashMap;
use std::hash::Hasher;
use std::ops::Div;
use pathfinding::prelude::{build_path, dijkstra_all};
use robotics_lib::interface::Direction;
use robotics_lib::interface::robot_map;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::World;
use endless_heights::MAP_SIZE;
use core::hash::Hash;

struct Wrapper<'a> {
    world: &'a Vec<Vec<Option<Tile>>>,
    row: usize,
    col: usize
}
impl<'a> PartialEq for Wrapper<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}
impl<'a> Eq for Wrapper<'a> {}
impl<'a> Hash for Wrapper<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.row, self.col).hash(state);
    }
}
impl<'a> Clone for Wrapper<'a> {
    fn clone(&self) -> Self {
        Self {
            world: self.world,
            row: self.row,
            col: self.col
        }
    }
}

pub fn dijkstra_path(start: &(u32, u32), destination: &(u32, u32), mut world: &World) -> Vec<Direction>{
    let robot_world=robot_map(&mut world).unwrap();
    fn robot_map_to_numbers(robot_world: Vec<Vec<Option<Tile>>>) -> Vec<u32> {
        let mut cell_list: Vec<(u32, u32)> = Vec::new();
        for x in 0..robot_world.len() {
            for y in 0..robot_world[x].len() {
                if robot_world[x][y].is_some() {
                    cell_list.push((x as u32, y as u32))
                }
            }
        }
        let number_list: Vec<u32> = cell_list.iter().map(|cell| convert_to_number(*cell)).collect();
        number_list
    }
    fn get_locked(robot_world: Vec<Vec<Option<Tile>>>) -> Vec<u32> {
        let mut unknown_tiles: Vec<u32> = Vec::new();
        let known_tiles = robot_map_to_numbers(robot_world);
        for n in 0..(MAP_SIZE as u32).pow(2) {
            if !known_tiles.contains(&(n as u32)) {
                unknown_tiles.push(n as u32);
            }
        }
        unknown_tiles
    }
    // TODO: THIS FUNCTION HAS NOT BEEN TESTED!!!
    fn successors<'a>(&node: &'a Wrapper) -> Vec<(Wrapper<'a>, i32)>
    {
        let mut successors = Vec::new();
        // Iterate over 3x3 adjacent tiles
        for row in [node.row-1, node.row+1] {
            for col in [node.col-1, node.col+1] {
                if let Some(_) = node.world[row][col] {
                    successors.push((Wrapper {
                        world: node.world, row, col
                    }, 10)) //TODO: Swap 10 with actual cost
                }
            };
        }
        successors
        /*
        let price = 10;
        // let locked = get_locked(robot_world);
        let locked = vec![];
        let mut result: Vec<(u32, usize)> = Vec::new();

        if n <= (MAP_SIZE as u32).pow(2) {
            if !locked.contains(&n) {
                if n <= (MAP_SIZE as u32) {
                    if n == 1 {
                        if !locked.contains(&(n + (MAP_SIZE as u32))) {
                            result.push((n + (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n + 1)) {
                            result.push((n + 1, price));
                        }
                        result
                    } else if n == (MAP_SIZE as u32) {
                        if !locked.contains(&(n + (MAP_SIZE as u32))) {
                            result.push((n + (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n - 1)) {
                            result.push((n - 1, price));
                        }
                        result
                    } else {
                        if !locked.contains(&(n + (MAP_SIZE as u32))) {
                            result.push((n + (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n - 1)) {
                            result.push((n - 1, price));
                        }
                        if !locked.contains(&(n + 1)) {
                            result.push((n + 1, price));
                        }
                        result
                    }
                } else if n >= ((MAP_SIZE as u32).pow(2) - ((MAP_SIZE as u32) - 1)) && n <= (MAP_SIZE as u32).pow(2) {
                    if n == (MAP_SIZE as u32).pow(2) - ((MAP_SIZE as u32) - 1) {
                        if !locked.contains(&(n - (MAP_SIZE as u32))) {
                            result.push((n - (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n + 1)) {
                            result.push((n + 1, price));
                        }
                        result
                    } else if n == (MAP_SIZE as u32).pow(2) {
                        if !locked.contains(&(n - (MAP_SIZE as u32))) {
                            result.push((n - (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n - 1)) {
                            result.push((n - 1, price));
                        }
                        result
                    } else {
                        if !locked.contains(&(n - (MAP_SIZE as u32))) {
                            result.push((n - (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n - 1)) {
                            result.push((n - 1, price));
                        }
                        if !locked.contains(&(n + 1)) {
                            result.push((n + 1, price));
                        }
                        result
                    }
                } else {
                    if n % (MAP_SIZE as u32) == 1 {
                        if !locked.contains(&(n - (MAP_SIZE as u32))) {
                            result.push((n - (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n + (MAP_SIZE as u32))) {
                            result.push((n + (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n + 1)) {
                            result.push((n + 1, price));
                        }
                        result
                    } else if n % (MAP_SIZE as u32) == 0 {
                        if !locked.contains(&(n - (MAP_SIZE as u32))) {
                            result.push((n - (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n + (MAP_SIZE as u32))) {
                            result.push((n + (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n - 1)) {
                            result.push((n - 1, price));
                        }
                        result
                    } else {
                        if !locked.contains(&(n - (MAP_SIZE as u32))) {
                            result.push((n - (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n + (MAP_SIZE as u32))) {
                            result.push((n + (MAP_SIZE as u32), price));
                        }
                        if !locked.contains(&(n + 1)) {
                            result.push((n + 1, price));
                        }
                        if !locked.contains(&(n - 1)) {
                            result.push((n - 1, price));
                        }
                        result
                    }
                }
            } else { vec![] }
        } else { vec![] }*/
    }
    let start_n = convert_to_number(*start);
    let destination_n = convert_to_number(*destination);

    // Rewritten to take the wrapper as input
    let reachables_from_start = dijkstra_all(
        &Wrapper { world: &robot_world, row: start.0 as usize, col: start.1 as usize }, 
        successors
    );

    let path = build_path(
        &Wrapper { world: &robot_world, row: destination.0 as usize, col: destination.1 as usize }, 
        &reachables_from_start
    );
    let direction_list = convert_to_directions(path);
    return direction_list;
    
}

fn bigger_multiple(n: u32) -> u32 {
    (n).div_ceil(MAP_SIZE as u32) * (MAP_SIZE as u32)
}
fn smaller_multiple(n: u32) -> u32 {
    if n <= (MAP_SIZE as u32) {
        return  0;
    }
    (n - (MAP_SIZE as u32)).div_ceil(MAP_SIZE as u32) * (MAP_SIZE as u32)
}

fn convert_to_tuple(n: u32) -> (u32, u32) {
    (bigger_multiple(n).div(MAP_SIZE as u32), (n - smaller_multiple(n)))
}

fn convert_to_number(cell: (u32, u32)) -> u32 {
    (cell.0 - 1) * (MAP_SIZE as u32) + cell.1
}

fn convert_to_directions<'a>(cells: Vec<Wrapper<'a>>) -> Vec<Direction> {
    let mut commands: Vec<Direction> = Vec::new();
    for step in cells.windows(2) {
        // TODO: Check that this mapping to directions is right
        if step[1].row > step[0].row {
            commands.push(Direction::Right);
        } else if step[1].row < step[0].row {
            commands.push(Direction::Left);
        } else if step[1].col > step[0].col {
            commands.push(Direction::Up);
        } else if step[1].col > step[0].col {
            commands.push(Direction::Down);
        }
        /*
        if cells.get(i) > cells.get(i-1) {
            if bigger_multiple(*cells.get(i).unwrap()) == bigger_multiple(*cells.get(i-1).unwrap()) {
                commands.push(Direction::Right);
            }
            else { commands.push(Direction::Down); }
        }
        else {
            if bigger_multiple(*cells.get(i).unwrap()) == bigger_multiple(*cells.get(i-1).unwrap()) {
                commands.push(Direction::Left);
            }
            else { commands.push(Direction::Up); }
        }
        */
    }
    commands
}
