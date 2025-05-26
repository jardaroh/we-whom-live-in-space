use bevy::{
  prelude::*,
  color::palettes::basic::*,
};
use crate::resources::theme::Theme;

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

pub fn button(asset_server: &AssetServer, theme: &Theme) -> impl Bundle + use<> {
  (
    Node {
      width: Val::Px(200.0),
      height: Val::Px(50.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    children![(
      Button,
      Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
      },
      BorderColor(Color::srgb(0.1, 0.1, 0.5)),
      BorderRadius::all(Val::Px(2.0)),
      BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
      children![(
        Text::new("Button"),
        TextFont {
          font: theme.font.clone(),
          font_size: 24.0,
          ..default()
        },
        TextColor(Color::WHITE),
      )]
    )],
  )
}
