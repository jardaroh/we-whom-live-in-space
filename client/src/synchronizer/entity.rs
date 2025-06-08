use bevy::prelude::*;
use bevy_spacetimedb::{
  ReadInsertEvent,
};

use crate::spacetime_bindings::{
  Entity as DbEntity,
};

pub fn sync_entities_system(
  mut commands: Commands,
  mut events: ReadInsertEvent<DbEntity>,
) {

}
