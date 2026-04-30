use {
    bevy::{ecs::schedule::ScheduleLabel, prelude::*},
    std::time::Duration,
};

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickTimer(Timer::new(
            Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        .add_systems(Update, run_tick);
    }
}

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tick;

#[derive(Resource)]
pub struct TickTimer(Timer);

fn run_tick(mut commands: Commands, time: Res<Time>, mut timer: ResMut<TickTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.run_schedule(Tick);
    }
}
