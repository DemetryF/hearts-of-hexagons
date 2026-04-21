use crate::components::{Country, Map};
use bevy::prelude::*;

pub fn gain_money(map: Res<Map>, mut countries: Query<&mut Country>) {
    for (_, province) in &map.provinces {
        let Some(country) = province.control else {
            continue;
        };

        let mut country = countries.get_mut(country).unwrap();

        country.money += 1;
    }
}
