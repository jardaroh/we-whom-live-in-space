use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Theme {
  pub name: String,
  pub primary_color: Color,
  pub secondary_color: Color,
  pub background_color: Color,
  pub font: Handle<Font>,
  pub font_color: Color,
  pub font_size: f32,
  pub padding: Val,
  pub border: UiRect,
}

impl Default for Theme {
  fn default() -> Self {
    Theme {
      name: "Default".to_string(),
      primary_color: Color::srgb(0.2, 0.4, 0.8),
      secondary_color: Color::srgb(0.8, 0.8, 0.8),
      background_color: Color::srgb(0.1, 0.1, 0.1),
      font: Handle::default(),
      font_color: Color::WHITE,
      font_size: 16.0,
      padding: Val::Px(10.0),
      border: UiRect::all(Val::Px(2.0)),
    }
  }
}
