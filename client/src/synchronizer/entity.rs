use bevy::prelude::*;
use bevy_spacetimedb::{
  InsertEvent, UpdateEvent, DeleteEvent,
};
use std::collections::HashMap;

use crate::spacetime_bindings::{
  Entity as DbEntity, EntityType, DVec3, DQuat,
};
use crate::components::{Ship, Mass, MaxThrust, Acceleration};

/// Resource to track mapping between SpacetimeDB entity IDs and Bevy entity IDs
#[derive(Resource, Default)]
pub struct EntityMapping {
  pub spacetime_to_bevy: HashMap<u64, Entity>,
  pub bevy_to_spacetime: HashMap<Entity, u64>,
}

impl EntityMapping {
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

/// Convert SpacetimeDB DQuat to Bevy Quat
fn dquat_to_quat(dquat: &DQuat) -> Quat {
  Quat::from_xyzw(dquat.x as f32, dquat.y as f32, dquat.z as f32, dquat.w as f32)
}

/// Spawn a new entity based on database data
fn spawn_entity(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<StandardMaterial>>,
  db_entity: &DbEntity,
) -> Entity {
  let position = dvec3_to_vec3(&db_entity.relative_position);
  let transform = Transform::from_translation(position);

  let entity = match db_entity.entity_type {
    EntityType::Ship => {
      // Create the main ship body
      let main_body_mesh = meshes.add(Cuboid::new(1.0, 1.0, 3.0));
      let ship_material = materials.add(StandardMaterial {
        base_color: Srgba::hex("#ffd891").unwrap().into(),
        metallic: 0.25,
        perceptual_roughness: 0.25,
        ..default()
      });
      
      // Create the forward indicator (a triangular prism pointing forward)
      let indicator_mesh = meshes.add(Cuboid::new(0.5, 0.5, 1.5));
      let indicator_material = materials.add(StandardMaterial {
        base_color: Srgba::hex("#ff4444").unwrap().into(), // Red for forward direction
        metallic: 0.5,
        perceptual_roughness: 0.2,
        ..default()
      });
      
      // Spawn the main ship entity
      let ship_entity = commands.spawn((
        Mesh3d(main_body_mesh),
        MeshMaterial3d(ship_material),
        transform,
        Ship,
        Mass(1000.0),
        MaxThrust(bevy::math::DVec3::new(1000.0, 1000.0, 1000.0)),
        Acceleration::default(),
        Name::new(format!("Ship: {}", db_entity.designation)),
      )).with_children(|parent| {
        // Spawn the forward indicator as a child of the ship
        // In Bevy, forward is -Z direction, so we position the indicator at negative Z
        parent.spawn((
          Mesh3d(indicator_mesh),
          MeshMaterial3d(indicator_material),
          Transform::from_xyz(0.0, 0.0, -2.25), // Position it at the front of the ship (negative Z is forward)
          Name::new("Forward Indicator"),
        ));
      }).id();
      
      ship_entity
    },
    EntityType::Planet => {
      let sphere_mesh = meshes.add(Sphere::new(5.0));
      commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
          base_color: Srgba::hex("#4a90e2").unwrap().into(),
          metallic: 0.1,
          perceptual_roughness: 0.8,
          ..default()
        })),
        transform,
        Name::new(format!("Planet: {}", db_entity.designation)),
      )).id()
    },
    EntityType::Moon => {
      let sphere_mesh = meshes.add(Sphere::new(2.0));
      commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
          base_color: Srgba::hex("#c0c0c0").unwrap().into(),
          metallic: 0.1,
          perceptual_roughness: 0.9,
          ..default()
        })),
        transform,
        Name::new(format!("Moon: {}", db_entity.designation)),
      )).id()
    },
    EntityType::Asteroid => {
      let sphere_mesh = meshes.add(Sphere::new(1.0));
      let scale = 0.5 + (db_entity.id as f32 % 100.0) / 100.0 * 2.0; // Deterministic scale based on ID
      commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
          base_color: Srgba::hex("#8c8c8c").unwrap().into(),
          metallic: 0.1,
          perceptual_roughness: 0.9,
          ..default()
        })),
        transform.with_scale(Vec3::splat(scale)),
        Name::new(format!("Asteroid: {}", db_entity.designation)),
      )).id()
    },
    EntityType::Star => {
      let sphere_mesh = meshes.add(Sphere::new(10.0));
      commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
          base_color: Srgba::hex("#ffff00").unwrap().into(),
          emissive: LinearRgba::rgb(1.0, 1.0, 0.5),
          metallic: 0.0,
          perceptual_roughness: 1.0,
          ..default()
        })),
        transform,
        Name::new(format!("Star: {}", db_entity.designation)),
      )).id()
    },
    EntityType::Comet => {
      let sphere_mesh = meshes.add(Sphere::new(0.5));
      commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
          base_color: Srgba::hex("#e6f3ff").unwrap().into(),
          emissive: LinearRgba::rgb(0.2, 0.2, 1.0),
          metallic: 0.0,
          perceptual_roughness: 0.8,
          ..default()
        })),
        transform,
        Name::new(format!("Comet: {}", db_entity.designation)),
      )).id()
    },
    EntityType::Custom(_) => {
      let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
      commands.spawn((
        Mesh3d(cube_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
          base_color: Srgba::hex("#ff00ff").unwrap().into(),
          metallic: 0.5,
          perceptual_roughness: 0.5,
          ..default()
        })),
        transform,
        Name::new(format!("Custom: {}", db_entity.designation)),
      )).id()
    },
  };

  info!("Spawned entity: {} (ID: {}, Type: {:?})", db_entity.designation, db_entity.id, db_entity.entity_type);
  entity
}

/// Update an existing entity's transform based on database data
fn update_entity_transform(
  bevy_entity: Entity,
  db_entity: &DbEntity,
  transform_query: &mut Query<&mut Transform>,
) -> bool {
  if let Ok(mut transform) = transform_query.get_mut(bevy_entity) {
    let new_position = dvec3_to_vec3(&db_entity.relative_position);
    transform.translation = new_position;

    // Update rotation if needed
    let rotation = dquat_to_quat(&db_entity.relative_rotation);
    transform.rotation = rotation;

    debug!("Updated entity {} position to {:?}", db_entity.designation, new_position);
    true
  } else {
    warn!("Failed to update transform for entity {}: Bevy entity not found", db_entity.designation);
    false
  }
}

pub fn sync_entities_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut entity_mapping: ResMut<EntityMapping>,
  mut insert_events: EventReader<InsertEvent<DbEntity>>,
  mut update_events: EventReader<UpdateEvent<DbEntity>>,
  mut delete_events: EventReader<DeleteEvent<DbEntity>>,
  mut transform_query: Query<&mut Transform>,
) {
  // Debug logging to see if events are received
  let insert_count = insert_events.len();
  let update_count = update_events.len();
  let delete_count = delete_events.len();
  
  if insert_count > 0 || update_count > 0 || delete_count > 0 {
    info!("Entity events received - Inserts: {}, Updates: {}, Deletes: {}", 
          insert_count, update_count, delete_count);
  }

  // Handle entity insertions
  for event in insert_events.read() {
    let db_entity = &event.row;
    info!("Processing entity insert: {} (ID: {})", db_entity.designation, db_entity.id);
    
    // Check if we already have this entity (shouldn't happen, but be safe)
    if let Some(existing_entity) = entity_mapping.get_bevy_entity(db_entity.id) {
      warn!("Entity {} already exists as Bevy Entity {:?}, skipping insert", db_entity.id, existing_entity);
      continue;
    }

    // Spawn the new entity
    let bevy_entity = spawn_entity(&mut commands, &mut meshes, &mut materials, db_entity);
    
    // Track the mapping
    entity_mapping.insert(db_entity.id, bevy_entity);
    
    info!("Successfully inserted entity: {} -> Bevy Entity {:?}", db_entity.designation, bevy_entity);
  }

  // Handle entity updates
  for event in update_events.read() {
    let db_entity = &event.new;
    debug!("Processing entity update: {} (ID: {})", db_entity.designation, db_entity.id);
    
    if let Some(bevy_entity) = entity_mapping.get_bevy_entity(db_entity.id) {
      // Try to update the transform
      if !update_entity_transform(bevy_entity, db_entity, &mut transform_query) {
        // Transform update failed - this could be a timing issue where the entity
        // was just spawned and Transform component isn't ready yet
        // We'll keep the mapping and let future updates try again
        debug!("Transform update failed for entity {} but keeping mapping - likely timing issue", db_entity.id);
      }
    } else {
      // Entity doesn't exist in our mapping - this could happen if:
      // 1. We missed an insert event
      // 2. The mapping was corrupted somehow
      // For now, just log this and don't auto-spawn to avoid duplicates
      warn!("Received update for unknown entity {} - no mapping found. Skipping update.", db_entity.id);
    }
  }

  // Handle entity deletions
  for event in delete_events.read() {
    let db_entity = &event.row;
    info!("Processing entity delete: {} (ID: {})", db_entity.designation, db_entity.id);
    
    if let Some(bevy_entity) = entity_mapping.remove_by_spacetime_id(db_entity.id) {
      // Despawn the Bevy entity
      commands.entity(bevy_entity).despawn();
      info!("Successfully deleted entity: {} (Bevy Entity {:?})", db_entity.designation, bevy_entity);
    } else {
      warn!("Received delete for unknown entity {}", db_entity.id);
    }
  }
}
