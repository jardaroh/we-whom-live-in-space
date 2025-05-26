use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GrayScale {
  pub bright: Color,
  pub one: Color,
  pub two: Color,
  pub three: Color,
  pub four: Color,
  pub five: Color,
  pub six: Color,
  pub seven: Color,
  pub eight: Color,
  pub nine: Color,
  pub full: Color,
}

impl Default for GrayScale {
  fn default() -> Self {
    GrayScale {
      bright: Color::srgb(0.95, 0.95, 0.95),
      one: Color::srgb(0.9, 0.9, 0.9),
      two: Color::srgb(0.8, 0.8, 0.8),
      three: Color::srgb(0.7, 0.7, 0.7),
      four: Color::srgb(0.6, 0.6, 0.6),
      five: Color::srgb(0.5, 0.5, 0.5),
      six: Color::srgb(0.4, 0.4, 0.4),
      seven: Color::srgb(0.3, 0.3, 0.3),
      eight: Color::srgb(0.2, 0.2, 0.2),
      nine: Color::srgb(0.1, 0.1, 0.1),
      full: Color::srgb(0.05, 0.05, 0.05),
    }
  }
}

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
  pub gray: GrayScale,
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
      gray: GrayScale::default(),
    }
  }
}
