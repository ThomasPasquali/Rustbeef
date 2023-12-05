use bevy::{
    app::{Plugin, Startup},
    prelude as bv,
};

use bevy_extern_events::{queue_event, ExternEventsPlugin};
use nla_compass::compass::{NLACompass, Destination};
use rand::{thread_rng, Rng};
use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, robot_view, Tools, where_am_i, robot_map, Direction},
    runner::{backpack::BackPack, Robot, Runnable, Runner},
    world::{coordinates::Coordinate, tile::Tile, worldgenerator::Generator, World},
};

use crate::{LEFT_ARROW, DOWN_ARROW, RIGHT_ARROW, UP_ARROW};

use super::terraingen::DISCOVERED_WORLD;
#[derive(bv::Resource)]
pub struct WorldData {
    pub runner: Runner,
}

#[derive(bv::Resource)]
pub struct TickTimer(pub bv::Timer);

pub struct MyRobot {
    robot: Robot,
    compass: NLACompass
}

#[derive(Default, Debug)]
pub struct WorldTick {
    pub changed_tiles: Vec<Vec<Option<Tile>>>,
    pub coordinates: Option<(usize, usize)>,
    pub direction: char
}

pub fn tick(
    time: bv::Res<bv::Time>,
    mut timer: bv::ResMut<TickTimer>,
    mut world_data: bv::ResMut<WorldData>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let _ = world_data.runner.game_tick();
    }
}

impl Runnable for MyRobot {
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    fn handle_event(&mut self, _event: Event) {}
    fn process_tick(&mut self, world: &mut World) {
        let (surrounding, pos) = where_am_i(self, world);
        let map = robot_map(world);
        // let direction = self.compass.get_move(&map, &surrounding, pos);

        println!("Current position: {:#?}", pos);
        println!("Surroundings: {:#?}", surrounding);
        // println!("Map: {:?}", map);

        // Go in random direction
        let mut rng = thread_rng();
        let d = rng.gen_range(0..=1);
        let direction = match d {
            0 => Direction::Down,
            _ => Direction::Right
        };
        let _ = go(self, world, direction.clone());

        println!("Going: {:#?}\n\n\n", direction.clone());

        // match direction.clone() {
        //     Some(d) => { go(self, world, match d {
        //         Direction::Up => Direction::Down,
        //         Direction::Right => Direction::Right,
        //         Direction::Down => Direction::Up,
        //         Direction::Left => Direction::Left
        //     }); },
        //     None => { println!("No direction from compass!"); }
        // }
        
        // Inform world that map changed
        queue_event(WorldTick {
            changed_tiles: surrounding,
            coordinates: Some((
                self.get_coordinate().get_col(),
                self.get_coordinate().get_row(),
            )),
            direction: match Some(direction.clone()) {
                Some(d) => match d {
                    Direction::Up => UP_ARROW,
                    Direction::Right => RIGHT_ARROW,
                    Direction::Down => DOWN_ARROW,
                    Direction::Left => LEFT_ARROW
                },
                None => '-'
            }
        });
    }
}

pub fn initialize_runner(mut commands: bv::Commands) {
    let mut robot = MyRobot {
        robot: Robot::new(),
        compass: NLACompass::new()
    };
    robot.compass.set_destination(Destination::COORDINATE(Coordinate::new(40, 40)));

    let mut generator = endless_heights::WorldGenerator {};
    
    DISCOVERED_WORLD.write().unwrap().world = generator.gen().0;
    commands.insert_resource(WorldData {
        runner: Runner::new(Box::new(robot), &mut generator, vec![NLACompass::new()]).unwrap(),
    });
    commands.insert_resource(TickTimer(bv::Timer::from_seconds(
        10.0,
        bv::TimerMode::Repeating,
    )))
}

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, initialize_runner)
            .add_plugins((ExternEventsPlugin::<WorldTick>::default(),))
            .add_systems(crate::bv::Update, tick);
    }
}