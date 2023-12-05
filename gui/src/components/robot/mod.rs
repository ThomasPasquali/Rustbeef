use bevy::{
    app::{Plugin, Startup},
    prelude as bv,
};

use bevy_extern_events::{queue_event, ExternEventsPlugin};
use rand::{thread_rng, Rng};
use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, robot_view, Tools},
    runner::{backpack::BackPack, Robot, Runnable, Runner},
    world::{coordinates::Coordinate, tile::Tile, worldgenerator::Generator, World},
};

use super::terraingen::DISCOVERED_WORLD;

#[derive(bv::Resource)]
pub struct WorldData {
    pub runner: Runner,
}

#[derive(bv::Resource)]
pub struct TickTimer(pub bv::Timer);

struct Tool;
impl Tools for Tool {}

pub struct MyRobot {
    robot: Robot,
}

#[derive(Default)]
pub struct Update {
    changed_tiles: Vec<Vec<Option<Tile>>>,
    coordinates: Option<(usize, usize)>,
    position: (usize, usize),
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
        let tiles = robot_view(self, world);
        // Inform world that map changed
        queue_event(Update {
            changed_tiles: tiles,
            coordinates: Some((
                self.get_coordinate().get_col(),
                self.get_coordinate().get_row(),
            )),
            position: (
                self.get_coordinate().get_row(),
                self.get_coordinate().get_col(),
            ),
        });
        // Go in random direction
        let mut rng = thread_rng();
        let _ = match rng.gen_range(0..=3) {
            0 => go(self, world, robotics_lib::interface::Direction::Left),
            1 => go(self, world, robotics_lib::interface::Direction::Up),
            2 => go(self, world, robotics_lib::interface::Direction::Down),
            3 => go(self, world, robotics_lib::interface::Direction::Right),
            _ => go(self, world, robotics_lib::interface::Direction::Right),
        };
    }
}

pub fn initialize_runner(mut commands: bv::Commands) {
    let robot = MyRobot {
        robot: Robot::new(),
    };
    let mut generator = endless_heights::WorldGenerator {};
    let tools = vec![Tool];
    DISCOVERED_WORLD.write().unwrap().world = generator.gen().0;
    commands.insert_resource(WorldData {
        runner: Runner::new(Box::new(robot), &mut generator, tools).unwrap(),
    });
    commands.insert_resource(TickTimer(bv::Timer::from_seconds(
        0.5,
        bv::TimerMode::Repeating,
    )))
}

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, initialize_runner)
            .add_plugins((ExternEventsPlugin::<Update>::default(),))
            .add_systems(crate::bv::Update, tick);
    }
}
