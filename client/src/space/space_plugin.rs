use bevy::prelude::*;

use super::backdrop::BackdropMaterial;

use super::backdrop::{
  setup_backdrop,
  backdrop_system,
};

pub fn space_plugin(app: &mut App) {
  app.add_plugins((
    MaterialPlugin::<BackdropMaterial>::default(),
  ))
  .add_systems(Startup, setup_backdrop)
  .add_systems(Update, backdrop_system);
}
