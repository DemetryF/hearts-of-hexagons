use {
    crate::components::{Country, Map},
    bevy::prelude::*,
};

pub const MONEY_PER_HOUR: usize = 1;

/// Every hour every province produces `MONEY_PER_HOUR` for its owner
pub fn gain_money(map: Res<Map>, mut countries: Query<&mut Country>) {
    for (_, province) in &map.provinces {
        let Some(country) = province.control else {
            continue;
        };

        let mut country = countries.get_mut(country).unwrap();

        country.money += MONEY_PER_HOUR;
    }
}
