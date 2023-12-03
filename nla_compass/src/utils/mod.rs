use pathfinding::prelude::{build_path, dijkstra_all};
use rand::Rng;

fn successors(&n: &u32) -> Vec<(u32, usize)> {
    let world_size: u32 = 5_u32;
    let price: usize = rand::thread_rng().gen_range(0..1000);

    // qui faccio i collegamenti "a griglia" tra le celle, tenendo conto degli angoli e delle colonne
    // nell'implementazione reale bisogna tenere conto delle celle "discovered" e "walkable" e
    // fare collegamenti appropriati

    if n <= world_size.pow(2) {
        if n <= world_size {
            if n == 1 {
                vec! [(n + world_size, price), (n + 1, price)]
            }
            else if n == world_size {
                vec! [(n + world_size, price), (n - 1, price)]
            }
            else {
                vec! [(n + world_size, price), (n - 1, price), (n + 1, price)]
            }
        }
        else if n >= (world_size.pow(2) - (world_size - 1)) && n <= world_size.pow(2) {
            if n == world_size.pow(2) - (world_size - 1) {
                vec! [(n - world_size, 10), (n + 1, 10)]
            }
            else if n == world_size.pow(2) {
                vec! [(n - world_size, 10), (n - 1, 10)]
            }
            else {
                vec! [(n + 1, 10), (n - world_size, 10), (n - 1, 10)]
            }
        }
        else {
            if n % world_size == 1 {
                vec! [(n + 1, 10), (n - world_size, 10), (n + world_size, 10)]
            }
            else if n % world_size == 0 {
                vec! [(n - 1, 10), (n - world_size, 10), (n + world_size, 10)]
            }
            else {
                vec! [(n - world_size, 10), (n + 1, 10), (n + world_size, 10), (n - 1, 10)]
            }
        }
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
    let reachables_from_20 = dijkstra_all(&20, successors);
    let path_20_to_1 = build_path(&1, &reachables_from_20);
    println!("{:?}", successors(&1));
    println!("{:?}", path_20_to_1);
    let commands = convert_to_directions(path_20_to_1, 5);
    println!("{:?}", commands);
}
