use crate::map::Map;
use bevy::prelude::*;
use shared::*;

pub struct ProvincePlugin;

impl Plugin for ProvincePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(capture);
    }
}

fn capture(
    event: On<DivisionMoved>,
    divisions: Query<&Division>,
    mut provs: Query<&mut Owner>,
    map: ResMut<Map>,
) {
    let country = divisions.get(event.event_target()).unwrap().country;

    let captured_id = map.provs[&event.to];
    let mut captured_owner = provs.get_mut(captured_id).unwrap();

    if captured_owner.0 != Some(country) {
        captured_owner.0 = Some(country);
    }
}
