mod controls;
mod province;

pub use province::*;

use {
    crate::common::{controls::ControlsPlugin, province::ProvincePlugin},
    bevy::prelude::*,
};

pub struct CommonPlugins;

impl Plugin for CommonPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((ProvincePlugin, ControlsPlugin));
    }
}
