use bevy::{
  prelude::*,
};
use crate::resources::theme::Theme;
use crate::constants::SizingMode;
use crate::ui::{
  button::button,
  layout::grid,
};

pub fn main_menu(
  asset_server: &AssetServer,
  theme: &Theme
) -> impl Bundle {
  (
    Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      padding: UiRect::all(Val::Px(10.0)),
      ..default()
    },
    children![(
      grid(
        asset_server,
        theme,
        SizingMode::Fill,
        vec![GridTrack::fr(1.0)],
        vec![GridTrack::px(32.0), GridTrack::auto()],
        4.0 // gap
      ),
      children![(
        grid(
          asset_server,
          theme,
          SizingMode::Fill,
          vec![
            GridTrack::percent(15.0),
            GridTrack::percent(15.0),
            GridTrack::percent(15.0),
            GridTrack::percent(15.0),
            GridTrack::percent(15.0),
          ],
          vec![GridTrack::fr(1.0)],
          4.0 // gap
        ),
        children![(
          button(
            "Main Menu",
            asset_server,
            theme,
            SizingMode::Fill
          )
        ),
        (
          button(
            "Video",
            asset_server,
            theme,
            SizingMode::Fill
          )
        ),
        (
          button(
            "Audio",
            asset_server,
            theme,
            SizingMode::Fill
          )
        ),
        (
          button(
            "Graphics",
            asset_server,
            theme,
            SizingMode::Fill
          )
        ),
        (
          button(
            "Quit",
            asset_server,
            theme,
            SizingMode::Fill
          )
        )],
      ), (
        Node {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          ..default()
        },
        BackgroundColor(theme.gray.seven.into()),
      )],
    )],
  )
}
