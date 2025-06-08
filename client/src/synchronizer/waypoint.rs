use bevy::prelude::*;
use bevy_spacetimedb::{
  InsertEvent, UpdateEvent, DeleteEvent,
};
use std::collections::HashMap;

use crate::spacetime_bindings::{
  Waypoint as DbWaypoint, DVec3,
};

/// Component to mark waypoint entities
#[derive(Component)]
pub struct WaypointMarker {
  pub spacetime_id: u64,
  pub entity_id: u64,
  pub order_index: u32,
}

/// Resource to track mapping between SpacetimeDB waypoint IDs and Bevy entity IDs
#[derive(Resource, Default)]
pub struct WaypointMapping {
  pub spacetime_to_bevy: HashMap<u64, Entity>,
  pub bevy_to_spacetime: HashMap<Entity, u64>,
}

impl WaypointMapping {
  pub fn insert(&mut self, spacetime_id: u64, bevy_entity: Entity) {
    self.spacetime_to_bevy.insert(spacetime_id, bevy_entity);
    self.bevy_to_spacetime.insert(bevy_entity, spacetime_id);
  }

  pub fn remove_by_spacetime_id(&mut self, spacetime_id: u64) -> Option<Entity> {
    if let Some(bevy_entity) = self.spacetime_to_bevy.remove(&spacetime_id) {
      self.bevy_to_spacetime.remove(&bevy_entity);
      Some(bevy_entity)
    } else {
      None
    }
  }

  pub fn get_bevy_entity(&self, spacetime_id: u64) -> Option<Entity> {
    self.spacetime_to_bevy.get(&spacetime_id).copied()
  }

  pub fn get_spacetime_id(&self, bevy_entity: Entity) -> Option<u64> {
    self.bevy_to_spacetime.get(&bevy_entity).copied()
  }
}

/// Convert SpacetimeDB DVec3 to Bevy Vec3
fn dvec3_to_vec3(dvec3: &DVec3) -> Vec3 {
  Vec3::new(dvec3.x as f32, dvec3.y as f32, dvec3.z as f32)
}

/// Spawn a new waypoint entity based on database data
fn spawn_waypoint(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<StandardMaterial>>,
  db_waypoint: &DbWaypoint,
) -> Entity {
  let position = dvec3_to_vec3(&db_waypoint.target_position);
  let transform = Transform::from_translation(position);

  // Create a bright red thin tall cuboid
  let waypoint_mesh = meshes.add(Cuboid::new(0.2, 1.5, 0.2)); // width, height, depth
  
  commands.spawn((
    Mesh3d(waypoint_mesh),
    MeshMaterial3d(materials.add(StandardMaterial {
      base_color: Srgba::hex("#ff0000").unwrap().into(), // Bright red
      emissive: LinearRgba::rgb(1.0, 0.0, 0.0), // Strong red glow
      metallic: 0.0,
      perceptual_roughness: 0.1,
      ..default()
    })),
    transform,
    WaypointMarker {
      spacetime_id: db_waypoint.id,
      entity_id: db_waypoint.entity_id,
      order_index: db_waypoint.order_index,
    },
    Name::new(format!("Waypoint {} (Entity: {}, Order: {})", 
      db_waypoint.id, 
      db_waypoint.entity_id, 
      db_waypoint.order_index)),
  )).id()
}

/// Update waypoint position based on database data
fn update_waypoint_transform(
  bevy_entity: Entity,
  db_waypoint: &DbWaypoint,
  transform_query: &mut Query<&mut Transform>,
) -> bool {
  if let Ok(mut transform) = transform_query.get_mut(bevy_entity) {
    let new_position = dvec3_to_vec3(&db_waypoint.target_position);
    transform.translation = new_position;
    true
  } else {
    false
  }
}

/// System to synchronize waypoints from SpacetimeDB
pub fn sync_waypoints_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut waypoint_mapping: ResMut<WaypointMapping>,
  mut transform_query: Query<&mut Transform>,
  mut insert_events: EventReader<InsertEvent<DbWaypoint>>,
  mut update_events: EventReader<UpdateEvent<DbWaypoint>>,
  mut delete_events: EventReader<DeleteEvent<DbWaypoint>>,
) {
  // Debug logging to see if events are received
  let insert_count = insert_events.len();
  let update_count = update_events.len();
  let delete_count = delete_events.len();
  
  if insert_count > 0 || update_count > 0 || delete_count > 0 {
    info!("Waypoint events received - Inserts: {}, Updates: {}, Deletes: {}", 
          insert_count, update_count, delete_count);
  }

  // Handle waypoint insertions
  for event in insert_events.read() {
    let db_waypoint = &event.row;
    info!("Processing waypoint insert: ID {} for entity {} at order {}", 
          db_waypoint.id, db_waypoint.entity_id, db_waypoint.order_index);
    
    // Check if we already have this waypoint (shouldn't happen, but be safe)
    if let Some(existing_entity) = waypoint_mapping.get_bevy_entity(db_waypoint.id) {
      warn!("Waypoint {} already exists as Bevy Entity {:?}, skipping insert", 
            db_waypoint.id, existing_entity);
      continue;
    }

    // Spawn the new waypoint
    let bevy_entity = spawn_waypoint(&mut commands, &mut meshes, &mut materials, db_waypoint);
    
    // Track the mapping
    waypoint_mapping.insert(db_waypoint.id, bevy_entity);
    
    info!("Successfully inserted waypoint: {} -> Bevy Entity {:?}", 
          db_waypoint.id, bevy_entity);
  }

  // Handle waypoint updates
  for event in update_events.read() {
    let db_waypoint = &event.new;
    debug!("Processing waypoint update: ID {} for entity {}", 
           db_waypoint.id, db_waypoint.entity_id);
    
    if let Some(bevy_entity) = waypoint_mapping.get_bevy_entity(db_waypoint.id) {
      // Try to update the transform
      if !update_waypoint_transform(bevy_entity, db_waypoint, &mut transform_query) {
        debug!("Transform update failed for waypoint {} but keeping mapping - likely timing issue", 
               db_waypoint.id);
      }
    } else {
      warn!("Received update for unknown waypoint {} - no mapping found. Skipping update.", 
            db_waypoint.id);
    }
  }

  // Handle waypoint deletions
  for event in delete_events.read() {
    let db_waypoint = &event.row;
    info!("Processing waypoint delete: ID {} for entity {}", 
          db_waypoint.id, db_waypoint.entity_id);
    
    if let Some(bevy_entity) = waypoint_mapping.remove_by_spacetime_id(db_waypoint.id) {
      // Despawn the Bevy entity
      commands.entity(bevy_entity).despawn();
      info!("Successfully deleted waypoint: {} (Bevy Entity {:?})", 
            db_waypoint.id, bevy_entity);
    } else {
      warn!("Received delete for unknown waypoint {}", db_waypoint.id);
    }
  }
}