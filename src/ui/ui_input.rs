use bevy::prelude::*;
use bevy::input::{
  ButtonState,
  keyboard::{
    Key,
    KeyboardInput,
  },
};
use bevy::input_focus::{
  InputFocus,
  tab_navigation::TabIndex,
};
use super::ui_theme::Theme;

#[derive(Component)]
pub struct TextInput {
  pub value: String,
}

pub fn text_input(
  theme: &Res<Theme>,
  value: &str,
) -> impl Bundle {
  (
    Node {
      width: Val::Percent(100.0),
      height: Val::Auto,
      padding: UiRect::all(theme.padding),
      ..default()
    },
    Text(value.to_string()),
    TextColor(theme.color.into()),
    BackgroundColor(theme.color_default.into()),
    Interaction::default(),
    TextInput { value: value.to_string() },
    TabIndex(0),
  )
}

pub fn text_input_click_system(
  mut query: Query<(Entity, &Interaction), (Changed<Interaction>, With<TextInput>)>,
  mut input_focus: ResMut<InputFocus>,
) {
  for (entity, interaction) in query.iter_mut() {
    if *interaction == Interaction::Pressed {
      println!("Text input clicked via Interaction: {:?}", entity);
      input_focus.set(entity);
    }
  }
}

pub fn text_input_system(
  mut query: Query<(Entity, &mut TextInput, &mut Text), With<TextInput>>,
  mut keyboard_events: EventReader<KeyboardInput>,
  input_focus: Res<InputFocus>,
) {
  for (entity, mut text_input, mut text) in query.iter_mut() {
    // Check if this specific entity is focused
    if input_focus.get() != Some(entity) {
      continue;
    }

    for event in keyboard_events.read() {
      if event.state == ButtonState::Released {
        continue;
      }

      match &event.logical_key {
        Key::Backspace => {
          if !text_input.value.is_empty() {
            text_input.value.pop();
            text.0 = text_input.value.clone();
          }
        },
        Key::Character(input) => {
          if input.chars().all(|c| !c.is_control()) {
            text_input.value.push_str(input);
            text.0 = text_input.value.clone();
          }
        }
        _ => {},
      }
    }
  }
}
