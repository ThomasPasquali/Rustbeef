use std::any::TypeId;

use robotics_lib::{interface::Tools, world::{World, worldgenerator::Generator, coordinates::Coordinate}, runner::{Runner, Runnable, backpack::BackPack, Robot}, event::events::Event, energy::Energy};
use endless_heights::height;

struct Tool;
impl Tools for Tool {}

struct MyRobot(Robot);

impl Runnable for MyRobot {
    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }
    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }
    fn handle_event(&mut self, event: Event) {
        
    }
    fn process_tick(&mut self, world: &mut World) {
        
    }
}

fn initialize_runner() -> Runner {
    let mut robot = MyRobot(Robot::new());
    let mut generator = endless_heights::WorldGenerator{};
    
    let tools = vec![Tool];

    Runner::new(Box::new(robot), &mut generator, tools).unwrap()
}