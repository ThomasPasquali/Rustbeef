use pathfinding::prelude::{build_path, dijkstra_all};


fn successors(&n: &u32) -> Vec<(u32, usize)> {
    let world_size: u32 = 5_u32;
    let locked: Vec<u32> = vec![2, 3, 4, 5, 7, 8, 13, 15, 16, 20, 21, 22, 25];
    //let price: usize = rand::thread_rng().gen_range(0..1000);
    let price = 10;
    let mut result: Vec<(u32, usize)> = Vec::new();

    // qui faccio i collegamenti "a griglia" tra le celle, tenendo conto degli angoli e delle colonne
    // tenendo conto delle caselle locked (not walkable o not discovered)

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

fn closest_multiple(n: u32, base: u32) -> u32 {
    (n).div_ceil(base) * base
}

fn convert_to_directions(cells: Vec<u32>, map_size: u32) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for i in 1..cells.len() {
        if cells.get(i) > cells.get(i-1) {
            if closest_multiple(*cells.get(i).unwrap(), map_size) == closest_multiple(*cells.get(i-1).unwrap(), map_size) {
                commands.push("Right".to_string());
            }
            else { commands.push("Down".to_string()); }
        }
        else {
            if closest_multiple(*cells.get(i).unwrap(), map_size) == closest_multiple(*cells.get(i-1).unwrap(), map_size) {
                commands.push("Left".to_string());
            }
            else { commands.push("Up".to_string()); }
        }
    }
    commands
}

fn main() {
    let reachables_from_6 = dijkstra_all(&6, successors);
    let path_6_to_10 = build_path(&10, &reachables_from_6);
    println!("{:?}", path_6_to_10);
    let commands = convert_to_directions(path_6_to_10, 5);
    println!("{:?}", commands);
}
