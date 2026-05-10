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
            .replicate::<ProvinceOwner>()
            .replicate::<Division>()
            .replicate::<DivisionPos>()
            .replicate::<DivisionOwner>()
            .replicate::<CombatStats>()
            .replicate::<Path>()
            .replicate::<MovementBlock>()
            .replicate::<AttacksOn>()
            .replicate::<DefendsFrom>()
            .add_visibility_filter::<DivisionVisibility>()
            .add_mapped_client_event::<MovingOrderEvent>(Channel::Ordered)
            .add_mapped_client_event::<CountryAssignmentRequest>(Channel::Ordered)
            .add_mapped_server_event::<CountryAssignmentResponse>(Channel::Ordered);
    }
}
