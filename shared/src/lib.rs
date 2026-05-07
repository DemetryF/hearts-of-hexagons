mod components;
mod events;
mod hexagon_pos;

pub use components::*;
pub use events::*;
pub use hexagon_pos::*;

use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<Country>()
            .replicate::<Province>()
            .replicate::<Owner>()
            .replicate::<Division>()
            .replicate::<CombatStats>()
            .replicate::<Path>()
            .replicate::<MovementBlock>()
            .replicate::<AttacksOn>()
            .replicate::<DefendsFrom>()
            .add_mapped_client_event::<MovingOrder>(Channel::Ordered)
            .add_client_event::<CountryAssignment>(Channel::Ordered);
    }
}
