use std::ops::Div;
use pathfinding::prelude::{build_path, dijkstra_all};

fn successors(&n: &u32) -> Vec<(u32, usize)> {
    let world_size: u32 = 50_u32;
    let locked: Vec<u32> = vec![];
    //let price: usize = rand::thread_rng().gen_range(0..1000);
    let price = 10;
    let mut result: Vec<(u32, usize)> = Vec::new();

    if n <= world_size.pow(2) {
        if !locked.contains(&n) {
            if n <= world_size {
                if n == 1 {
                    if !locked.contains(&(n + world_size)) {
                        result.push((n + world_size, price));
                    }
                    if !locked.contains(&(n + 1)) {
                        result.push((n + 1, price));
                    }
                    result
                }
                else if n == world_size {
                    if !locked.contains(&(n + world_size)) {
                        result.push((n + world_size, price));
                    }
                    if !locked.contains(&(n - 1)) {
                        result.push((n - 1, price));
                    }
                    result
                }
                else {
                    if !locked.contains(&(n + world_size)) {
                        result.push((n + world_size, price));
                    }
                    if !locked.contains(&(n - 1)) {
                        result.push((n - 1, price));
                    }
                    if !locked.contains(&(n + 1)) {
                        result.push((n + 1, price));
                    }
                    result
                }
            }
            else if n >= (world_size.pow(2) - (world_size - 1)) && n <= world_size.pow(2) {
                if n == world_size.pow(2) - (world_size - 1) {
                    if !locked.contains(&(n - world_size)) {
                        result.push((n - world_size, price));
                    }
                    if !locked.contains(&(n + 1)) {
                        result.push((n + 1, price));
                    }
                    result
                }
                else if n == world_size.pow(2) {
                    if !locked.contains(&(n - world_size)) {
                        result.push((n - world_size, price));
                    }
                    if !locked.contains(&(n - 1)) {
                        result.push((n - 1, price));
                    }
                    result
                }
                else {
                    if !locked.contains(&(n - world_size)) {
                        result.push((n - world_size, price));
                    }
                    if !locked.contains(&(n - 1)) {
                        result.push((n - 1, price));
                    }
                    if !locked.contains(&(n + 1)) {
                        result.push((n + 1, price));
                    }
                    result
                }
            }
            else {
                if n % world_size == 1 {
                    if !locked.contains(&(n - world_size)) {
                        result.push((n - world_size, price));
                    }
                    if !locked.contains(&(n + world_size)) {
                        result.push((n + world_size, price));
                    }
                    if !locked.contains(&(n + 1)) {
                        result.push((n + 1, price));
                    }
                    result
                }
                else if n % world_size == 0 {
                    if !locked.contains(&(n - world_size)) {
                        result.push((n - world_size, price));
                    }
                    if !locked.contains(&(n + world_size)) {
                        result.push((n + world_size, price));
                    }
                    if !locked.contains(&(n - 1)) {
                        result.push((n - 1, price));
                    }
                    result
                }
                else {
                    if !locked.contains(&(n - world_size)) {
                        result.push((n - world_size, price));
                    }
                    if !locked.contains(&(n + world_size)) {
                        result.push((n + world_size, price));
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
        }
        else { vec![] }
    }

    else { vec![] }
}

fn bigger_multiple(n: u32, base: u32) -> u32 {
    (n).div_ceil(base) * base
}
fn smaller_multiple(n: u32, base: u32) -> u32 {
    if n <= base {
        return  0;
    }
    (n - base).div_ceil(base) * base
}

fn convert_to_tuple(n: u32, map_size: u32) -> (u32, u32) {
    (bigger_multiple(n, map_size).div(map_size), (n - smaller_multiple(n, map_size)))
}

fn convert_to_number(cell: (u32, u32), map_size: u32) -> u32 {
    (cell.0 - 1)*map_size + cell.1
}

fn convert_to_directions(cells: Vec<u32>, map_size: u32) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for i in 1..cells.len() {
        if cells.get(i) > cells.get(i-1) {
            if bigger_multiple(*cells.get(i).unwrap(), map_size) == bigger_multiple(*cells.get(i-1).unwrap(), map_size) {
                commands.push("Right".to_string());
            }
            else { commands.push("Down".to_string()); }
        }
        else {
            if bigger_multiple(*cells.get(i).unwrap(), map_size) == bigger_multiple(*cells.get(i-1).unwrap(), map_size) {
                commands.push("Left".to_string());
            }
            else { commands.push("Up".to_string()); }
        }
    }
    commands
}

fn main() {
    let reachables_from_40 = dijkstra_all(&40, successors);
    let path_40_to_50 = build_path(&50, &reachables_from_40);
    let cell_path: Vec<_> = path_40_to_50.iter().map(|n| convert_to_tuple(*n, 50)).collect();
    println!("{:?}", cell_path);
    let converted_cell_path: Vec<_> = cell_path.iter().map(|n| convert_to_number(*n, 50)).collect();
    println!("{:?}", converted_cell_path);
    let commands = convert_to_directions(path_40_to_50, 50);
    println!("{:?}", commands);
}
