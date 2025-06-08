use bevy::prelude::*;
use bevy::input_focus::tab_navigation::TabIndex;
use super::ui_theme::Theme;

pub fn button(
  theme: &Res<Theme>,
  text: &str,
  size: (Val, Val),
) -> impl Bundle {
  (
    Button,
    Node {
      width: size.0,
      height: size.1,
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    BackgroundColor(theme.color_default),
    TabIndex(0),
    children![(
      Text(text.to_string()),
      TextColor(theme.color_primary.into()),
    )],
  )
}

pub fn button_system(
  mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
  theme: Res<Theme>,
) {
  for (interaction, mut color) in &mut interaction_query {
    *color = match *interaction {
      Interaction::Pressed => BackgroundColor(theme.color_default_lightest.into()),
      Interaction::Hovered => BackgroundColor(theme.color_default_lighter.into()),
      Interaction::None => BackgroundColor(theme.color_default.into()),
    }
  }
}
