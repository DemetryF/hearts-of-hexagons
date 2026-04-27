use {
    crate::components::{Country, PlayingCountry},
    bevy::{input_focus::InputFocus, prelude::*},
};

const BACKGROUND_COLOR: Color = Color::linear_rgba(0.4, 0.4, 0.4, 0.5);

const BUTTON_NORMAL_COLOR: Color = Color::linear_rgb(0.4, 0.4, 0.4);
const BUTTON_HOVERED_COLOR: Color = Color::linear_rgb(0.43, 0.43, 0.43);
const BUTTON_PRESSED_COLOR: Color = Color::linear_rgb(0.46, 0.46, 0.46);

#[derive(Component)]
pub struct UiMoneyLabel;

#[derive(Component)]
pub struct UiBuyDivisionButton;

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
                UiMoneyLabel,
                Text::new(format!("{}\nmoney: {}\n", country.name, country.money)),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    margin: UiRect::bottom(px(10)),
                    ..Default::default()
                },
            ));

            parent.spawn((
                UiBuyDivisionButton,
                Button,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    border: UiRect::all(px(2)),
                    ..Default::default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::linear_rgb(0.5, 0.5, 0.5)),
                children![(
                    Text::new("Buy division"),
                    TextLayout::new_with_justify(Justify::Center),
                )],
            ));
        });
}

pub fn buy_division_button(
    mut input_focus: ResMut<InputFocus>,
    query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut Button,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<UiBuyDivisionButton>),
    >,
) {
    for (id, &interaction, mut color, mut button, mut border_color) in query {
        match interaction {
            Interaction::Pressed => {
                input_focus.set(id);
                color.0 = BUTTON_PRESSED_COLOR;
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(id);
                color.0 = BUTTON_HOVERED_COLOR;
                button.set_changed();
            }
            Interaction::None => {
                input_focus.set(id);
                color.0 = BUTTON_NORMAL_COLOR;
            }
        }

        *border_color = BorderColor::all(color.0);
    }
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
