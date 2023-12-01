use bevy::prelude as bv;
use robotics_lib::runner::Runner;

#[derive(bv::Resource)]
struct WorldData { runner: Runner }

#[derive(bv::Resource)]
pub struct TickTimer(pub bv::Timer);

fn tick(time: bv::Res<bv::Time>, mut timer: bv::ResMut<TickTimer>, world_data: &mut WorldData) {
    if timer.0.tick(time.delta()).just_finished() {
        let _ = world_data.runner.game_tick();
    }
}