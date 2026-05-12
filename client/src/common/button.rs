use {
    crate::game::UiBuyDivisionButton,
    bevy::{input_focus::InputFocus, prelude::*},
};

const BUTTON_NORMAL_COLOR: Color = Color::linear_rgb(0.4, 0.4, 0.4);
const BUTTON_HOVERED_COLOR: Color = Color::linear_rgb(0.43, 0.43, 0.43);
const BUTTON_PRESSED_COLOR: Color = Color::linear_rgb(0.46, 0.46, 0.46);

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (insert_background_color, button_interaction));
    }
}

fn insert_background_color(
    buttons: Query<Entity, (With<Button>, Without<BackgroundColor>)>,
    mut commands: Commands,
) {
    for entity in buttons {
        commands
            .entity(entity)
            .insert(BackgroundColor(BUTTON_NORMAL_COLOR));
    }
}

fn button_interaction(
    mut input_focus: ResMut<InputFocus>,
    query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut Button),
        (Changed<Interaction>, With<UiBuyDivisionButton>),
    >,
) {
    for (id, &interaction, mut color, mut button) in query {
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
    }
}
