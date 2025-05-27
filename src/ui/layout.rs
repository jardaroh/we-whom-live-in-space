use bevy::{
  prelude::*,
};
use crate::resources::theme::Theme;
use crate::constants::SizingMode;

pub fn grid(
  asset_server: &AssetServer,
  theme: &Theme,
  sizing_mode: SizingMode,
  cols: usize,
  rows: usize,
  gap: f32,
) -> impl Bundle {
  let (outer_node_width, outer_node_height) = match sizing_mode {
    SizingMode::Fixed { width, height } => (width, height),
    SizingMode::Fill => (Val::Percent(100.0), Val::Percent(100.0)),
    SizingMode::FitContent => (Val::Auto, Val::Auto),
  };

  let grid_template_columns = vec![GridTrack::flex(1.0); cols];
  let grid_template_rows = vec![GridTrack::flex(1.0); rows];

  let (column_gap, row_gap) = if gap > 0.0 {
    (Val::Px(gap), Val::Px(gap))
  } else {
    (Val::Auto, Val::Auto)
  };

  (
    Node {
      width: outer_node_width,
      height: outer_node_height,
      display: Display::Grid,
      grid_template_columns,
      grid_template_rows,
      row_gap,
      column_gap,
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
  )
}
