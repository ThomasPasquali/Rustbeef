mod helpers;
use std::hash::Hasher;
use std::ops::Div;
use core::hash::Hash;
use self::helpers::get_cost;
use self::helpers::Wrapper;

fn successors(&node: &Wrapper) -> Vec<(Wrapper, usize)> {
    let mut successors = Vec::new();
    // Iterate over 3x3 adjacent tiles
    for row in [node.row-1, node.row+1] {
        for col in [node.col-1, node.col+1] {
            if let Some(_) = node.world.as_ref()[row][col] {
                successors.push((Wrapper {
                    world: node.world.clone(), row, col
                }, get_cost(&node.world.as_ref()[node.row][node.col].unwrap(),
                            &node.world.as_ref()[row][col].unwrap())))
            }
        };
    }
    successors
}