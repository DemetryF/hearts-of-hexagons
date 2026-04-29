use {
    crate::components::{Country, Owner},
    bevy::prelude::*,
};

pub const MONEY_PER_HOUR: usize = 1;

/// Every hour every province produces `MONEY_PER_HOUR` for its owner
pub fn gain_money(provs: Query<&Owner>, mut countries: Query<&mut Country>) {
    for &Owner(control) in provs {
        let Some(country) = control else {
            continue;
        };

        let mut country = countries.get_mut(country).unwrap();

        country.money += MONEY_PER_HOUR;
    }
}
