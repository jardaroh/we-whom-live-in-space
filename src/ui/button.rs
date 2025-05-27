use bevy::{
  prelude::*,
};
use crate::resources::theme::Theme;
use crate::constants::SizingMode;

pub fn button_system(
  theme: Res<Theme>,
  mut interaction_query: Query<(
    &Interaction,
    &mut BackgroundColor,
    &mut BorderColor,
    &Children,
  ),
  (Changed<Interaction>, With<Button>)
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, mut border_color, children) in &mut interaction_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
      Interaction::Hovered => {
        **text = "Hover".to_string();
        *color = theme.gray.six.into();
        border_color.0 = theme.gray.one.into();
      }
      Interaction::Pressed => {
        **text = "Pressed".to_string();
        *color = theme.gray.five.into();
        border_color.0 = theme.gray.one.into();
      }
      Interaction::None => {
        **text = "Click Me".to_string();
        *color = theme.gray.seven.into();
        border_color.0 = theme.gray.one.into();
      }
    }
  }
}

pub fn button(asset_server: &AssetServer, theme: &Theme, sizing_mode: SizingMode) -> impl Bundle {
  let (outer_node_width, outer_node_height) = match sizing_mode {
    SizingMode::Fixed { width, height } => (width, height),
    SizingMode::Fill => (Val::Percent(100.0), Val::Percent(100.0)),
    SizingMode::FitContent => (Val::Auto, Val::Auto),
  };

  let (inner_node_width, inner_node_height) = match sizing_mode {
    SizingMode::FitContent => (Val::Auto, Val::Auto),
    _ => (Val::Percent(100.0), Val::Percent(100.0)),
  };

  (
    Node {
      width: outer_node_width,
      height: outer_node_height,
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    children![(
      Button,
      Node {
        width: inner_node_width,
        height: inner_node_height,
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
      },
      BorderColor(theme.gray.one.into()),
      BorderRadius::all(Val::Px(2.0)),
      BackgroundColor(theme.gray.seven.into()),
      children![(
        Text::new("Button"),
        TextFont {
          font: theme.font.clone(),
          font_size: 24.0,
          ..default()
        },
        TextColor(theme.gray.one.into()),
      )]
    )],
  )
}
