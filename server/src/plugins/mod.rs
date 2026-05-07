mod battle;
mod division_movement;
mod economy;
mod province;
mod tick;

pub use {battle::*, division_movement::*, economy::*, province::*, tick::*};

use bevy::prelude::*;

pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BattlePlugin,
            DivisionMovementPlugin,
            ProvincePlugin,
            EconomyPlugin,
            TickPlugin,
        ));
    }
}
