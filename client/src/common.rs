mod button;
mod controls;
mod province;

pub use {button::*, controls::*, province::*};

use bevy::prelude::*;

pub struct CommonPlugins;

impl Plugin for CommonPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((ProvincePlugin, ControlsPlugin, ButtonPlugin));
    }
}
