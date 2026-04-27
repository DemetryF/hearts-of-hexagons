use {
    bevy::{ecs::schedule::ScheduleLabel, prelude::*},
    std::time::Duration,
};

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tick;

#[derive(Resource)]
pub struct TickTimer(Timer);

pub fn setup_ticks(mut commands: Commands) {
    commands.insert_resource(TickTimer(Timer::new(
        Duration::from_secs(1),
        TimerMode::Repeating,
    )));
}

pub fn run_tick(mut commands: Commands, time: Res<Time>, mut timer: ResMut<TickTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.run_schedule(Tick);
    }
}
