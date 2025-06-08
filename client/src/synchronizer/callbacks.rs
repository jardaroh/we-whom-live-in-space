use bevy::prelude::*;
use crate::spacetime_bindings::*;

use super::entity::setup_entity_synchronization;

pub fn register_callbacks(ctx: &DbConnection) {
  println!("Registering callbacks for synchronization...");
  setup_entity_synchronization(ctx);
}
