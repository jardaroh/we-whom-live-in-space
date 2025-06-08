use bevy::{
  input_focus::{
    tab_navigation::TabIndex,
    InputFocus,
  },
  prelude::*,
};
use crate::ui::ui_theme::Theme;

pub fn focus_system(
  mut commands: Commands,
  focus: Res<InputFocus>,
  mut query: Query<Entity, With<TabIndex>>,
  theme: Res<Theme>,
) {
  if focus.is_changed() {
    for element in query.iter_mut() {
      if focus.0 == Some(element) {
        commands.entity(element).insert(Outline {
          color: theme.color_primary_lightest.into(),
          width: theme.outline_width,
          offset: theme.outline_offset,
        });
      } else {
        commands.entity(element).remove::<Outline>();
      }
    }
  }
}
