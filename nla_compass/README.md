<!-- PROJECT LOGO -->
# Never lost again compass by Rustbeef 🧭

This tool suggests directions to reach a set destination.

If the tiles to reach the destination have been already discovered before, the tool will suggest the least expensive path using Dijkstra's algorithm.

Otherwise, the tool will weight the possible options according to the configured coefficients and suggest a direction.

The tool uses no energy :)

## Example
```rust
pub struct MyRobot {
    robot: Robot,
    compass: NLACompass
}

impl Runnable for MyRobot {
    fn process_tick (&mut self, world: &mut robotics_lib::world::World) {
        // Asks the compass for a move
        let suggested_move = self.compass.get_move (
            &robot_map(world).unwrap(), self.get_coordinate()
        );
        // Your robot logic
    }
}

// Initialization of the Runner with the compass tool
fn init_runner() {
    let mut robot = MyRobot{
        robot: Robot::new(),
        compass: NLACompass::new()
    };
    // Configure a tiletype as destination. The robot will explore new tiles while doing so (last field set to `true`)
    let destination = Destination::TileType(TileType::Sand, true);
    robot.compass.set_destination(destination);

    let run = Runner::new(Box::new(robot), &mut WorldGenerator::init()).unwrap();
    run.game_tick();
}
```

## Usage
Note: complete description for all functions and structs can be found in the docs inside the `compass` module.
#### Configuring the destination

The destination can be set by calling the `set_destination(destination: Destination)` function of the `NLACompass` object. Once the destination has been reached, the destination will be reset.

Available `Destination` options:
- `Content(Content, bool)`: if `bool` is true, the tool will suggest random paths (but preferring the least expensive directions) until it finds the given content. If `bool` is false, the robot will scan the known world and find the closest matching content. Then it will suggest the least expensive path.
- `TileType(TileType, bool)`: if `bool` is true, the tool will suggest random paths (but preferring the least expensive directions) until it finds the given tile type. If `bool` is false, the robot will scan the known world and find the closest matching tile type. Then it will suggest the least expensive path.
- `Coordinate((usize, usize), bool)`: if `bool` is true, the tool will suggest a path going to the destination coordinate (but preferring the least expensive directions). If `bool` is false, the robot will suggest the least expensive path known a-priori. (note: the coordinate follows the general convention used in `RoboticLib`: the first `usize` is the row and second is the `col`).

The destination can be changed at any moment, since the path computation is done at every step. The destination can be reset manually by calling `clear_destination()`.

#### Asking for directions
The suggested direction can be retrieved by calling the `get_move(map: &Vec<Vec<Option<Tile>>>, curr_pos: (usize, usize))` function of the `NLACompass` object. If everything is configured correctly, the function should return a `Result` with a `Direction`. Otherwise it will return one of the errors defined in `MoveError`. Refer to the docs for additional details.

### Advanced configuration
Usually the default coefficients should work well with any world. Manual tuning can be done by creating a `NLACompassParams` object and passing it to the `set_params(params: NLACompassParams)` function of the `NLACompass` object. Refer to the docs for additional details.

## Group members

- [Thomas Pasquali](mailto:thomas.pasquali@studenti.unitn.it) [\[Telegram\]](https://t.me/thom_pasqui)  (group leader)
- [Salvatore Andaloro](mailto:salvatore.andaloro@studenti.unitn.it)
- [Claudio Foroncelli](mailto:claudio.foroncelli@studenti.unitn.it)
- [Florian Kandra](mailto:florian.kandra@studenti.unitn.it)
