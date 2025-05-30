use bevy::prelude::*;
use bevy::input_focus::tab_navigation::TabIndex;
use super::ui_theme::Theme;

#[derive(Component)]
pub struct Checkbox {
  pub checked: bool,
}

pub fn checkbox(
  theme: &Res<Theme>,
  checked: bool,
) -> impl Bundle {
  (
    Button,
    Node {
      width: Val::Px(24.0),
      height: Val::Px(24.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      padding: UiRect::all(Val::Px(4.0)),
      ..default()
    },
    Checkbox {checked},
    TabIndex(0),
    children![(
      Node {
        width: Val::Px(16.0),
        height: Val::Px(16.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
      },
      BackgroundColor(if checked {
        theme.color_primary.into()
      } else {
        theme.color_default.into()
      }),
      BorderRadius::all(theme.corner_radius),
    )],
  )
}

pub fn checkbox_system(
  mut interaction_query: Query<(&Interaction, &mut Checkbox, &Children), Changed<Interaction>>,
  mut child_query: Query<&mut BackgroundColor>,
  theme: Res<Theme>,
) {
  for (interaction, mut checkbox, children) in &mut interaction_query {
    match *interaction {
      Interaction::Pressed => {
        checkbox.checked = !checkbox.checked;
        println!("Checkbox toggled: {}", checkbox.checked);

        // Update the first child's BackgroundColor
        if let Some(child) = children.iter().next() {
          if let Ok(mut background_color) = child_query.get_mut(child) {
            background_color.0 = if checkbox.checked {
              theme.color_primary.into()
            } else {
              theme.color_default.into()
            };
          }
        }
      }
      Interaction::Hovered => {
        // Optionally handle hover state
      }
      Interaction::None => {
        // Optionally handle none state
      }
    }
  }
}
