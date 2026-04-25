use bevy::prelude::*;

use crate::components::{Country, PlayingCountry};

const BACKGROUND_COLOR: Color = Color::linear_rgba(0.4, 0.4, 0.4, 0.5);

#[derive(Component)]
pub struct UiMoneyLabel;

pub fn display_country_info(
    country: Option<Single<&Country, With<PlayingCountry>>>,
    mut commands: Commands,
) {
    let Some(country) = country else {
        return;
    };

    println!("display country info");

    commands
        .spawn((
            Node {
                width: percent(20),
                height: percent(20),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::all(percent(1)),
                border_radius: BorderRadius::all(percent(5)),

                ..Default::default()
            },
            BackgroundColor(BACKGROUND_COLOR),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("{}\nmoney: {}\n", country.name, country.money)),
                UiMoneyLabel,
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    margin: UiRect::bottom(px(10)),
                    ..Default::default()
                },
            ));
        });
}

pub fn update_country_info(
    country: Option<Single<&Country, With<PlayingCountry>>>,
    money_label: Option<Single<&mut Text, With<UiMoneyLabel>>>,
) {
    let Some(country) = country else {
        return;
    };

    let mut money_label = money_label.unwrap();

    money_label.0 = format!("{}\nmoney: {}", country.name, country.money);
}
