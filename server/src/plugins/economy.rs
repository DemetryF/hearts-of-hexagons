use {
    crate::plugins::tick::Tick,
    bevy::prelude::*,
    shared::{Country, Owner},
};

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Tick, gain_money);
    }
}

const MONEY_PER_HOUR: usize = 1;

/// Every hour every province produces `MONEY_PER_HOUR` for its owner
fn gain_money(provs: Query<&Owner>, mut countries: Query<&mut Country>) {
    for &Owner(control) in provs {
        let Some(country) = control else {
            continue;
        };

        let mut country = countries.get_mut(country).unwrap();

        country.money += MONEY_PER_HOUR;
    }
}
