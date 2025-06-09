use spacetimedb::{
  reducer, table, ReducerContext, ScheduleAt, Table, TimeDuration,
};

#[derive(spacetimedb::SpacetimeType)]
pub enum EntityType {
  Star,
  Planet,
  Moon,
  Asteroid,
  Comet,
  Ship,
  Custom(String),
}

#[derive(spacetimedb::SpacetimeType)]
pub enum NodeName {
  Sun,
  Earth,
  Mars,
  Venus,
  Jupiter,
  Saturn,
  Uranus,
  Neptune,
  Pluto,
  Moon,
  Ceres,
  Eris,
  Haumea,
  Makemake,
  Phoebe,
  Titan,
  Callisto,
  Ganymede,
  Io,
  Europa,
  Enceladus,
  Triton,
  Charon,
  Titania,
  Oberon,
  Rhea,
  Iapetus,
  Dione,
  Tethys,
  Mimas,
  Hyperion,
  Ariel,
  Umbriel,
  Miranda,
  Custom(String),
}

#[derive(spacetimedb::SpacetimeType)]
pub struct DVec3 {
  x: f64,
  y: f64,
  z: f64,
}

#[derive(spacetimedb::SpacetimeType, Clone)]
pub struct DQuat {
  x: f64,
  y: f64,
  z: f64,
  w: f64,
}

#[table(name = node, public)]
pub struct Node {
  #[primary_key]
  #[auto_inc]
  id: u64,
  #[index(btree)]
  name: NodeName,
  #[index(btree)]
  parent_id: Option<u64>,
  node_position: DVec3,
  node_velocity: DVec3,
  node_rotation: DQuat,
  node_rotational_velocity: DVec3,
  orbit_progress: f64,
}

#[table(name = entity, public)]
pub struct Entity {
  #[primary_key]
  #[auto_inc]
  id: u64,
  #[index(btree)]
  node: NodeName,
  #[unique]
  #[index(btree)]
  designation: String,
  relative_position: DVec3,
  relative_velocity: DVec3,
  relative_rotation: DQuat,
  relative_rotational_velocity: DVec3,
  mass: f64,
  max_impulse: DVec3, // main thrust (x), retro thrust (y), nav thrust (z)
  #[index(btree)]
  entity_type: EntityType,
}

#[table(name = test_reducer_schedule, scheduled(test_reducer))]
struct TestReducerSchedule {
  #[primary_key]
  #[auto_inc]
  scheduled_id: u64,

  scheduled_at: ScheduleAt,
}

#[table(name = animation_counter)]
struct AnimationCounter {
  #[primary_key]
  id: u64,
  counter: u64,
}

#[table(name = waypoint, public)]
pub struct Waypoint {
  #[primary_key]
  #[auto_inc]
  id: u64,
  #[index(btree)]
  entity_id: u64, // Foreign key to Entity table
  target_position: DVec3,
  #[index(btree)]
  order_index: u32, // For sequencing multiple waypoints (0 = next waypoint)
}

#[reducer]
fn test_reducer(
  ctx: &ReducerContext,
  _arg: TestReducerSchedule,
) {  
  log::info!("test_reducer executing...");
  
  // Find the TestShip entity
  if let Some(mut ship) = ctx.db.entity().designation().find(&"TestShip".to_string()) {
    log::info!("Found TestShip at position ({}, {}, {})", 
              ship.relative_position.x, ship.relative_position.y, ship.relative_position.z);
    
    // Find the next waypoint for this ship (order_index = 0 is the active waypoint)
    let mut target_waypoint = None;
    for waypoint in ctx.db.waypoint().iter() {
      if waypoint.entity_id == ship.id && waypoint.order_index == 0 {
        target_waypoint = Some(waypoint);
        break;
      }
    }
    
    if let Some(waypoint) = target_waypoint {
      log::info!("Found waypoint at ({}, {}, {})", 
                waypoint.target_position.x, waypoint.target_position.y, waypoint.target_position.z);
      
      // Calculate vector from ship to waypoint
      let target_pos = &waypoint.target_position;
      let current_pos = &ship.relative_position;
      
      let dx = target_pos.x - current_pos.x;
      let dy = target_pos.y - current_pos.y;
      let dz = target_pos.z - current_pos.z;
      
      let distance = (dx * dx + dy * dy + dz * dz).sqrt();
      
      // Don't try to rotate if we're already very close or if the distance is zero
      if distance < 0.1 {
        log::info!("Ship {} is very close to waypoint, no rotation needed", ship.designation);
        return;
      }
      
      // Calculate target direction vector (normalized)
      let target_direction = DVec3 {
        x: dx / distance,
        y: dy / distance,
        z: dz / distance,
      };
      
      // In Bevy, forward is -Z direction
      let forward_direction = DVec3 { x: 0.0, y: 0.0, z: -1.0 };
      
      // Calculate the quaternion that rotates from forward_direction to target_direction
      let target_quat = quat_from_direction(&forward_direction, &target_direction);
      
      // Current ship rotation as quaternion
      let current_quat = ship.relative_rotation;
      
      // Calculate the angular difference between current and target rotations
      let rotation_diff = quat_angle_between(&current_quat, &target_quat);
      
      log::info!("Target direction: ({:.3}, {:.3}, {:.3})", target_direction.x, target_direction.y, target_direction.z);
      log::info!("Current quat: ({:.3}, {:.3}, {:.3}, {:.3})", current_quat.x, current_quat.y, current_quat.z, current_quat.w);
      log::info!("Target quat: ({:.3}, {:.3}, {:.3}, {:.3})", target_quat.x, target_quat.y, target_quat.z, target_quat.w);
      log::info!("Rotation difference: {:.3} radians ({:.1} degrees)", rotation_diff, rotation_diff.to_degrees());
      
      // Physics-based rotation calculation using ship properties
      let target_tolerance = 0.044; // ±2.5 degrees in radians
      let dt = 0.05; // 50ms time step (20 FPS)
      
      // Extract ship's physical properties
      let mass = ship.mass;
      let nav_thrust = ship.max_impulse.z; // Navigation (rotational) thrust
      
      // Calculate moment of inertia from ship mass (assuming reasonable ship dimensions)
      // For a ship-like object, moment of inertia is typically mass * radius^2
      // Assuming ship dimensions roughly 10m x 8m x 6m (length x width x height)
      let ship_length = 10.0; // meters
      let ship_width = 8.0;   // meters
      let ship_height = 6.0;  // meters
      
      // Calculate moment of inertia for each axis (treating ship as a box)
      let moment_pitch = mass * (ship_width * ship_width + ship_height * ship_height) / 12.0; // rotation around X (pitch)
      let moment_yaw = mass * (ship_length * ship_length + ship_height * ship_height) / 12.0; // rotation around Y (yaw)
      let moment_roll = mass * (ship_length * ship_length + ship_width * ship_width) / 12.0; // rotation around Z (roll)
      
      // Use average moment of inertia for general rotation calculations
      let avg_moment_of_inertia = (moment_pitch + moment_yaw + moment_roll) / 3.0;
      let max_angular_acceleration = nav_thrust / avg_moment_of_inertia; // rad/s²
      
      // Current angular velocity magnitude
      let current_angular_vel = &ship.relative_rotational_velocity;
      let current_angular_speed = (current_angular_vel.x * current_angular_vel.x + 
                                  current_angular_vel.y * current_angular_vel.y + 
                                  current_angular_vel.z * current_angular_vel.z).sqrt();
      
      // Calculate time to stop with current velocity using current acceleration
      let time_to_stop = if max_angular_acceleration > 0.0 {
        current_angular_speed / max_angular_acceleration
      } else {
        0.0
      };
      
      // Distance to stop (area under deceleration curve)
      let stop_distance = current_angular_speed * time_to_stop - 0.5 * max_angular_acceleration * time_to_stop * time_to_stop;
      
      // Halfway point for acceleration/deceleration switch
      let halfway_point = rotation_diff / 2.0;
      
      log::info!("Ship mass: {:.1} kg, Calculated avg MOI: {:.1} kg⋅m², Nav thrust: {:.1} N", 
                mass, avg_moment_of_inertia, nav_thrust);
      log::info!("Ship dimensions: {:.1}m x {:.1}m x {:.1}m, MOI (pitch/yaw/roll): {:.1}/{:.1}/{:.1}", 
                ship_length, ship_width, ship_height, moment_pitch, moment_yaw, moment_roll);
      log::info!("Max angular accel: {:.4} rad/s², Current speed: {:.4} rad/s", 
                max_angular_acceleration, current_angular_speed);
      log::info!("Stop distance: {:.4} rad, Halfway: {:.4} rad", stop_distance, halfway_point);
      
      // Determine target angular velocity based on physics
      let target_angular_velocity = if rotation_diff <= stop_distance + target_tolerance {
        // We need to decelerate - calculate how much we should slow down
        let deceleration_needed = max_angular_acceleration * dt;
        (current_angular_speed - deceleration_needed).max(0.0)
      } else if rotation_diff > halfway_point {
        // We're in the first half - accelerate up to maximum safe velocity
        let max_safe_velocity = (max_angular_acceleration * rotation_diff).sqrt();
        let accelerated_velocity = current_angular_speed + max_angular_acceleration * dt;
        accelerated_velocity.min(max_safe_velocity)
      } else {
        // We're in the second half - start decelerating
        let remaining_distance = rotation_diff;
        let max_safe_velocity = (2.0 * max_angular_acceleration * remaining_distance).sqrt();
        current_angular_speed.min(max_safe_velocity)
      };
      
      // Calculate actual rotation step for this frame
      let rotation_speed = target_angular_velocity * dt;
      
      log::info!("Target angular velocity: {:.4} rad/s, Rotation step: {:.4} rad", 
                target_angular_velocity, rotation_speed);
      
      // Apply rotation step toward target
      let new_quat = if rotation_diff < target_tolerance {
        // Very close to target - use a small final slerp instead of snapping
        let final_t = (rotation_speed / rotation_diff).min(1.0);
        quat_slerp(&current_quat, &target_quat, final_t)
      } else {
        // Slerp (spherical linear interpolation) toward target
        let t = rotation_speed / rotation_diff; // Proportion of remaining rotation to complete this step
        quat_slerp(&current_quat, &target_quat, t.min(1.0))
      };
      
      log::info!("New quat: ({:.3}, {:.3}, {:.3}, {:.3})", new_quat.x, new_quat.y, new_quat.z, new_quat.w);
      
      // Update ship rotation
      ship.relative_rotation = new_quat;
      
      // Set rotational velocity with physics-based angular velocity
      if rotation_diff < target_tolerance {
        // Completely stop rotation when target is reached
        ship.relative_rotational_velocity = DVec3 { x: 0.0, y: 0.0, z: 0.0 };
        log::info!("Ship rotation stopped - target reached");
      } else {
        // Calculate rotation axis and apply physics-based angular velocity
        let axis = quat_rotation_axis(&current_quat, &target_quat);
        ship.relative_rotational_velocity = DVec3 {
          x: axis.x * target_angular_velocity,
          y: axis.y * target_angular_velocity,
          z: axis.z * target_angular_velocity,
        };
        log::info!("Angular velocity: ({:.4}, {:.4}, {:.4})", 
                  ship.relative_rotational_velocity.x, 
                  ship.relative_rotational_velocity.y, 
                  ship.relative_rotational_velocity.z);
      }
      
      // MOVEMENT SYSTEM - Only move when properly oriented
      let orientation_tolerance = 0.175; // ~10 degrees - close enough to start moving
      let close_enough_to_move = rotation_diff < orientation_tolerance;
      
      if close_enough_to_move {
        log::info!("Ship {} is oriented well enough to start moving (rotation diff: {:.3} rad)", 
                  ship.designation, rotation_diff);
        
        // Calculate movement physics
        let main_thrust = ship.max_impulse.x; // Forward thrust
        let retro_thrust = ship.max_impulse.y; // Reverse thrust  
        let main_acceleration = main_thrust / mass; // m/s²
        let retro_acceleration = retro_thrust / mass; // m/s²
        
        // Current velocity components
        let current_velocity = &ship.relative_velocity;
        let current_speed = (current_velocity.x * current_velocity.x + 
                            current_velocity.y * current_velocity.y + 
                            current_velocity.z * current_velocity.z).sqrt();
        
        // Calculate stopping distance with current velocity and retro thrust
        let stopping_time = if retro_acceleration > 0.0 {
          current_speed / retro_acceleration
        } else {
          0.0
        };
        let stopping_distance = current_speed * stopping_time - 0.5 * retro_acceleration * stopping_time * stopping_time;
        
        log::info!("Movement: distance={:.1}m, stopping_dist={:.1}m, current_speed={:.2}m/s", 
                  distance, stopping_distance, current_speed);
        
        // Improved movement logic with gentler physics
        let max_cruise_speed = 2.0; // Maximum cruise speed (m/s) - much lower for smoother movement
        let max_approach_speed = 1.0; // Maximum speed when close to waypoint (m/s)
        let close_distance_threshold = 10.0; // Switch to approach mode when closer than this (m)
        
        // Determine current max speed based on distance
        let current_max_speed = if distance < close_distance_threshold {
          max_approach_speed
        } else {
          max_cruise_speed
        };
        
        // Calculate desired speed based on distance - smooth curve
        let desired_speed = if distance < 1.0 {
          0.0 // Stop when very close
        } else {
          // Use smooth curve: speed increases with distance up to max
          let speed_factor = (distance / 20.0).min(1.0); // Reaches max at 20m distance
          current_max_speed * speed_factor
        };
        
        // Determine thrust direction and magnitude based on current vs desired speed
        let speed_error = desired_speed - current_speed;
        let thrust_magnitude = if speed_error.abs() < 0.1 {
          // Speed is close to desired - gentle adjustments only
          log::info!("CRUISE PHASE - speed: {:.2}m/s, desired: {:.2}m/s", current_speed, desired_speed);
          main_thrust * 0.1 // Very gentle thrust for stability
        } else if speed_error > 0.0 {
          // Need to speed up
          log::info!("ACCELERATION PHASE - speed: {:.2}m/s, desired: {:.2}m/s", current_speed, desired_speed);
          main_thrust * (speed_error / current_max_speed).min(1.0) // Proportional thrust
        } else {
          // Need to slow down
          log::info!("DECELERATION PHASE - speed: {:.2}m/s, desired: {:.2}m/s", current_speed, desired_speed);
          retro_thrust * (-speed_error / current_max_speed).min(1.0) // Proportional retro thrust
        };
        
        // ALWAYS move toward the waypoint - thrust magnitude controls speed, not direction
        // The ship should thrust in the direction of the target, not its current facing
        let acceleration_direction = target_direction; // Move toward waypoint
        
        log::info!("Acceleration direction (toward waypoint): ({:.3}, {:.3}, {:.3})", 
                  acceleration_direction.x, acceleration_direction.y, acceleration_direction.z);
        
        // Calculate acceleration step
        let thrust_acceleration = thrust_magnitude / mass;
        let acceleration_step = thrust_acceleration * dt;
        
        // Apply velocity change
        let new_velocity = DVec3 {
          x: current_velocity.x + acceleration_direction.x * acceleration_step,
          y: current_velocity.y + acceleration_direction.y * acceleration_step,
          z: current_velocity.z + acceleration_direction.z * acceleration_step,
        };
        
        // Apply position change using average velocity
        let avg_velocity = DVec3 {
          x: (current_velocity.x + new_velocity.x) * 0.5,
          y: (current_velocity.y + new_velocity.y) * 0.5,
          z: (current_velocity.z + new_velocity.z) * 0.5,
        };
        
        let new_position = DVec3 {
          x: ship.relative_position.x + avg_velocity.x * dt,
          y: ship.relative_position.y + avg_velocity.y * dt,
          z: ship.relative_position.z + avg_velocity.z * dt,
        };
        
        // Update ship state
        ship.relative_velocity = new_velocity;
        ship.relative_position = new_position;
        
        let new_speed = (ship.relative_velocity.x * ship.relative_velocity.x + 
                        ship.relative_velocity.y * ship.relative_velocity.y + 
                        ship.relative_velocity.z * ship.relative_velocity.z).sqrt();
        log::info!("Applied thrust: {:.1}N toward waypoint, new velocity: ({:.2}, {:.2}, {:.2}), new speed: {:.2}m/s", 
                  thrust_magnitude, ship.relative_velocity.x, ship.relative_velocity.y, ship.relative_velocity.z, new_speed);
        log::info!("New position: ({:.2}, {:.2}, {:.2})", ship.relative_position.x, ship.relative_position.y, ship.relative_position.z);
        
        // Suppress unused variable warning
        let _ = main_acceleration;
      } else {
        log::info!("Ship {} waiting for better orientation before moving (rotation diff: {:.3} rad)", 
                  ship.designation, rotation_diff);
      }
      
      // Check if ship has reached the waypoint (close enough to consider it reached)
      let waypoint_reached_tolerance = 3.0; // 3 meters - tighter tolerance 
      let reached_waypoint = distance < waypoint_reached_tolerance;
      
      if reached_waypoint {
        log::info!("Ship {} has reached waypoint! Distance: {:.2}m. Creating new waypoint...", ship.designation, distance);
        
        // Store values we need before moving ship
        let ship_id = ship.id;
        let ship_position = DVec3 { x: ship.relative_position.x, y: ship.relative_position.y, z: ship.relative_position.z };
        
        // Stop the ship when reaching waypoint to prevent overshoot
        ship.relative_velocity = DVec3 { x: 0.0, y: 0.0, z: 0.0 };
        
        // Update the entity in the database first
        ctx.db.entity().designation().update(ship);
        
        // Get counter for pseudo-randomness
        let counter = ctx.db.animation_counter().iter().next().map(|c| c.counter).unwrap_or(0);
        
        // Delete the current waypoint
        ctx.db.waypoint().id().delete(&waypoint.id);
        log::info!("Deleted waypoint {}", waypoint.id);
        
        // Create a new random waypoint
        let random_angle = ((ship_id * 7919 + counter * 1009) % 628) as f64 / 100.0; // Pseudo-random angle [0, 2π]
        let random_distance = 15.0 + ((ship_id * 1327 + random_angle as u64 * 2003) % 20) as f64; // Distance 15-35 units
        
        let new_x = ship_position.x + random_distance * random_angle.cos();
        let new_z = ship_position.z + random_distance * random_angle.sin();
        let new_y = ship_position.y + ((random_angle * 100.0) as u64 % 11) as f64 - 5.0; // Y variation: -5 to +5
        
        let new_waypoint = Waypoint {
          id: 0, // Auto-incremented
          entity_id: ship_id,
          target_position: DVec3 { x: new_x, y: new_y, z: new_z },
          order_index: 0,
        };
        
        ctx.db.waypoint().insert(new_waypoint);
        log::info!("Created new waypoint at ({:.1}, {:.1}, {:.1})", new_x, new_y, new_z);
        
        // Update animation counter for pseudo-randomness
        if let Some(mut counter) = ctx.db.animation_counter().id().find(&1) {
          counter.counter += 1;
          ctx.db.animation_counter().id().update(counter);
        } else {
          ctx.db.animation_counter().insert(AnimationCounter { id: 1, counter: 1 });
        }
      } else {
        // Update the entity in the database
        ctx.db.entity().designation().update(ship);
        
        log::info!("Updated ship - Distance to waypoint: {:.2}m, Rotation remaining: {:.3} rad", distance, rotation_diff);
      }
    } else {
      // No waypoint found - ship stays in place
      log::info!("No waypoint found for ship {}", ship.designation);
    }
  } else {
    log::info!("TestShip not found in database");
  }
}

// Helper functions for quaternion math

fn quat_from_direction(from: &DVec3, to: &DVec3) -> DQuat {
  // Calculate cross product for rotation axis
  let cross = DVec3 {
    x: from.y * to.z - from.z * to.y,
    y: from.z * to.x - from.x * to.z,
    z: from.x * to.y - from.y * to.x,
  };
  
  // Calculate dot product for angle
  let dot = from.x * to.x + from.y * to.y + from.z * to.z;
  
  // Handle edge cases
  if dot >= 0.99999 {
    // Vectors are essentially the same - no rotation needed
    return DQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
  }
  
  if dot <= -0.99999 {
    // Vectors are opposite - 180 degree rotation around any perpendicular axis
    // Choose a perpendicular axis
    let axis = if from.x.abs() < 0.9 {
      DVec3 { x: 1.0, y: 0.0, z: 0.0 }
    } else {
      DVec3 { x: 0.0, y: 1.0, z: 0.0 }
    };
    // Cross product to get perpendicular axis
    let perp = DVec3 {
      x: from.y * axis.z - from.z * axis.y,
      y: from.z * axis.x - from.x * axis.z,
      z: from.x * axis.y - from.y * axis.x,
    };
    let len = (perp.x * perp.x + perp.y * perp.y + perp.z * perp.z).sqrt();
    return DQuat { x: perp.x / len, y: perp.y / len, z: perp.z / len, w: 0.0 };
  }
  
  // Normal case
  let w = 1.0 + dot;
  let len = (cross.x * cross.x + cross.y * cross.y + cross.z * cross.z + w * w).sqrt();
  
  DQuat {
    x: cross.x / len,
    y: cross.y / len,
    z: cross.z / len,
    w: w / len,
  }
}

fn quat_angle_between(q1: &DQuat, q2: &DQuat) -> f64 {
  // Calculate the dot product of the quaternions
  let dot = q1.x * q2.x + q1.y * q2.y + q1.z * q2.z + q1.w * q2.w;
  
  // Clamp the dot product to avoid numerical errors
  let clamped_dot = dot.abs().min(1.0);
  
  // The angle between quaternions is 2 * acos(|dot|)
  2.0 * clamped_dot.acos()
}

fn quat_slerp(q1: &DQuat, q2: &DQuat, t: f64) -> DQuat {
  let mut dot = q1.x * q2.x + q1.y * q2.y + q1.z * q2.z + q1.w * q2.w;
  
  // If dot product is negative, negate one quaternion to take shorter path
  let (q2_x, q2_y, q2_z, q2_w) = if dot < 0.0 {
    dot = -dot;
    (-q2.x, -q2.y, -q2.z, -q2.w)
  } else {
    (q2.x, q2.y, q2.z, q2.w)
  };
  
  // If quaternions are very close, use linear interpolation
  if dot > 0.9995 {
    let x = q1.x + t * (q2_x - q1.x);
    let y = q1.y + t * (q2_y - q1.y);
    let z = q1.z + t * (q2_z - q1.z);
    let w = q1.w + t * (q2_w - q1.w);
    
    let len = (x * x + y * y + z * z + w * w).sqrt();
    return DQuat { x: x / len, y: y / len, z: z / len, w: w / len };
  }
  
  // Spherical linear interpolation
  let theta = dot.acos();
  let sin_theta = theta.sin();
  
  let t1 = ((1.0 - t) * theta).sin() / sin_theta;
  let t2 = (t * theta).sin() / sin_theta;
  
  DQuat {
    x: t1 * q1.x + t2 * q2_x,
    y: t1 * q1.y + t2 * q2_y,
    z: t1 * q1.z + t2 * q2_z,
    w: t1 * q1.w + t2 * q2_w,
  }
}

fn quat_rotation_axis(q1: &DQuat, q2: &DQuat) -> DVec3 {
  // Calculate the relative rotation quaternion
  let rel_quat = quat_multiply(&quat_inverse(q1), q2);
  
  // Extract the axis from the quaternion
  let sin_half_angle = (rel_quat.x * rel_quat.x + rel_quat.y * rel_quat.y + rel_quat.z * rel_quat.z).sqrt();
  
  if sin_half_angle < 1e-6 {
    // No significant rotation
    return DVec3 { x: 0.0, y: 0.0, z: 1.0 }; // Default axis
  }
  
  DVec3 {
    x: rel_quat.x / sin_half_angle,
    y: rel_quat.y / sin_half_angle,
    z: rel_quat.z / sin_half_angle,
  }
}

fn quat_multiply(q1: &DQuat, q2: &DQuat) -> DQuat {
  DQuat {
    x: q1.w * q2.x + q1.x * q2.w + q1.y * q2.z - q1.z * q2.y,
    y: q1.w * q2.y - q1.x * q2.z + q1.y * q2.w + q1.z * q2.x,
    z: q1.w * q2.z + q1.x * q2.y - q1.y * q2.x + q1.z * q2.w,
    w: q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z,
  }
}

fn quat_inverse(q: &DQuat) -> DQuat {
  let norm_sq = q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w;
  DQuat {
    x: -q.x / norm_sq,
    y: -q.y / norm_sq,
    z: -q.z / norm_sq,
    w: q.w / norm_sq,
  }
}

#[reducer(init)]
fn init(
  ctx: &ReducerContext,
) {
  ctx.db.node().insert(Node {
    id: 0,
    name: NodeName::Sun,
    parent_id: None,
    node_position: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    node_velocity: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    node_rotation: DQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
    node_rotational_velocity: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    orbit_progress: 1.0,
  });

  ctx.db.entity().insert(Entity {
    id: 0,
    node: NodeName::Sun,
    designation: "TestShip".to_string(),
    relative_position: DVec3 { x: 0.0, y: 5.0, z: -5.0 },
    relative_velocity: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    relative_rotation: DQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, // Identity quaternion (no rotation)
    relative_rotational_velocity: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    entity_type: EntityType::Ship,
    mass: 1000.0, // 1 ton
    max_impulse: DVec3 { x: 400.0, y: 250.0, z: 150.0 }, // main, retro, nav thrust - much lower for gentle movement
  });

  // Add a waypoint for the TestShip to fly to (requiring significant rotation)
  ctx.db.waypoint().insert(Waypoint {
    id: 0, // Auto-incremented
    entity_id: 1, // TestShip's ID
    target_position: DVec3 { x: 10.0, y: 5.0, z: -5.0 }, // Closer waypoint for testing
    order_index: 0, // First waypoint
  });

  ctx.db.test_reducer_schedule()
    .insert(TestReducerSchedule {
      scheduled_id: 1,
      scheduled_at: ScheduleAt::Interval(TimeDuration::from_micros(00_050_000)),
    });
}

#[reducer]
pub fn reset_ship(ctx: &ReducerContext) {
  log::info!("Resetting ship position and velocity...");
  
  if let Some(mut ship) = ctx.db.entity().designation().find(&"TestShip".to_string()) {
    log::info!("Found ship at position ({}, {}, {})", 
              ship.relative_position.x, ship.relative_position.y, ship.relative_position.z);
    
    let ship_id = ship.id; // Store the ID before moving ship
    
    // Reset ship position and velocity
    ship.relative_position = DVec3 { x: 0.0, y: 5.0, z: -5.0 };
    ship.relative_velocity = DVec3 { x: 0.0, y: 0.0, z: 0.0 };
    ship.relative_rotation = DQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }; // Identity quaternion
    ship.relative_rotational_velocity = DVec3 { x: 0.0, y: 0.0, z: 0.0 };
    
    ctx.db.entity().designation().update(ship);
    log::info!("Reset ship to origin (0, 5, -5)");
    
    // Also clear any existing waypoints and create a new one
    for waypoint in ctx.db.waypoint().iter() {
      if waypoint.entity_id == ship_id {
        ctx.db.waypoint().id().delete(&waypoint.id);
        log::info!("Deleted waypoint {}", waypoint.id);
      }
    }
    
    // Create a simple nearby waypoint for testing
    ctx.db.waypoint().insert(Waypoint {
      id: 0,
      entity_id: ship_id,
      target_position: DVec3 { x: 10.0, y: 5.0, z: -5.0 },
      order_index: 0,
    });
    log::info!("Created new waypoint at (10, 5, -5)");
  } else {
    log::info!("TestShip not found");
  }
}
