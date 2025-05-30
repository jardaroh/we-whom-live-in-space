use bevy::prelude::*;

#[derive(Resource)]
pub struct Theme {
  #[allow(unused)]
  pub color: Color,
  #[allow(unused)]
  pub color_default: Color,
  #[allow(unused)]
  pub color_default_lighter: Color,
  #[allow(unused)]
  pub color_default_lightest: Color,
  #[allow(unused)]
  pub color_primary: Color,
  #[allow(unused)]
  pub color_primary_lighter: Color,
  #[allow(unused)]
  pub color_primary_lightest: Color,
  #[allow(unused)]
  pub color_secondary: Color,
  #[allow(unused)]
  pub color_secondary_lighter: Color,
  #[allow(unused)]
  pub color_secondary_lightest: Color,
  #[allow(unused)]
  pub font: Handle<Font>,

  // border
  #[allow(unused)]
  pub corner_radius: Val,
}

impl Default for Theme {
  fn default() -> Self {
    init_theme()
  }
}

fn init_theme() -> Theme {
  Theme {
    color: Color::WHITE,
    color_default: Color::srgb(0.1, 0.1, 0.1),
    color_default_lighter: Color::srgb(0.15, 0.15, 0.15),
    color_default_lightest: Color::srgb(0.2, 0.2, 0.2),
    color_primary: Color::srgb(0.0, 0.5, 1.0),
    color_primary_lighter: Color::srgb(0.0, 0.6, 1.0),
    color_primary_lightest: Color::srgb(0.0, 0.7, 1.0),
    color_secondary: Color::srgb(0.5, 0.5, 0.5),
    color_secondary_lighter: Color::srgb(0.6, 0.6, 0.6),
    color_secondary_lightest: Color::srgb(0.7, 0.7, 0.7),
    font: Handle::default(),
    corner_radius: Val::Px(2.0),
  }
}
