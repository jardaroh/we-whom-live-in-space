use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub struct DVec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[allow(dead_code)]
impl DVec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z }
  }

  pub fn zero() -> Self {
    Self::new(0.0, 0.0, 0.0)
  }

  pub fn length(&self) -> f64 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  pub fn normalize(&self) -> Self {
    let len = self.length();
    if len > 0.0 {
      Self::new(self.x / len, self.y / len, self.z / len)
    } else {
      Self::zero()
    }
  }

  pub fn dot(&self, other: &DVec3) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn cross(&self, other: &DVec3) -> DVec3 {
    DVec3::new(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x,
    )
  }
}

impl std::ops::Add for DVec3 {
  type Output = DVec3;
  fn add(self, rhs: DVec3) -> DVec3 {
    DVec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl std::ops::Sub for DVec3 {
  type Output = DVec3;
  fn sub(self, rhs: DVec3) -> DVec3 {
    DVec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl std::ops::Mul<f64> for DVec3 {
  type Output = DVec3;
  fn mul(self, rhs: f64) -> DVec3 {
    DVec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl std::ops::AddAssign for DVec3 {
  fn add_assign(&mut self, rhs: DVec3) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
  }
}

impl std::ops::MulAssign<f64> for DVec3 {
  fn mul_assign(&mut self, rhs: f64) {
    self.x *= rhs;
    self.y *= rhs;
    self.z *= rhs;
  }
}

/// Flight control modes that determine how the autopilot behaves
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlightMode {
  /// Maintain current position and orientation (station keeping)
  Hold,
  /// Fly to a specific target position
  NavigateTo(DVec3),
  /// Follow a continuous target (e.g., player input)
  Track(DVec3),
  /// Intercept a moving target at a future point
  Intercept { target_pos: DVec3, target_vel: DVec3 },
  /// Manual control - apply thrust directly
  Manual(DVec3), // Direct thrust vector
}

/// Core Newtonian physics simulation - no control logic, just pure physics
pub fn simulate_physics(
  thrust_vector: DVec3,
  torque_vector: DVec3,
  mass: f64,
  moment_of_inertia: f64,
  position: &mut DVec3,
  velocity: &mut DVec3,
  rotation: &mut DVec3,
  rotational_velocity: &mut DVec3,
  delta_time: f64,
) {
  // Apply thrust to create acceleration (F = ma, so a = F/m)
  let acceleration = thrust_vector * (1.0 / mass);
  
  // Update velocity with acceleration
  *velocity += acceleration * delta_time;
  
  // Apply very light drag for stability (space has essentially no drag, but game feel)
  *velocity *= 0.9995; // Very minimal damping to preserve momentum
  
  // Update position with velocity
  *position += *velocity * delta_time;
  
  // Apply torque to create angular acceleration (τ = Iα, so α = τ/I)
  let angular_acceleration = torque_vector * (1.0 / moment_of_inertia);
  
  // Update rotational velocity with angular acceleration
  *rotational_velocity += angular_acceleration * delta_time;
  
  // Apply angular damping for stability
  *rotational_velocity *= 0.95; // More aggressive damping for stability during attitude control
  
  // Update rotation with rotational velocity
  *rotation += *rotational_velocity * delta_time;
  
  // Normalize rotation angles
  rotation.x = rotation.x % (2.0 * std::f64::consts::PI);
  rotation.y = rotation.y % (2.0 * std::f64::consts::PI);
  rotation.z = rotation.z % (2.0 * std::f64::consts::PI);
}

/// Flight control system - calculates thrust needed to achieve desired behavior
pub fn calculate_flight_control(
  flight_mode: FlightMode,
  current_position: DVec3,
  current_velocity: DVec3,
  current_rotation: DVec3,
  mass: f64,
  max_main_thrust: f64,    // Forward thrust (X)
  max_retro_thrust: f64,   // Reverse thrust (Y) 
  max_nav_thrust: f64,     // Maneuvering thrust (Z)
  _delta_time: f64,
) -> (DVec3, DVec3) { // Returns (thrust_vector, torque_vector)
  match flight_mode {
    FlightMode::Hold => {
      // Station keeping - gentle corrections to maintain position
      calculate_hold_control(current_velocity, current_rotation, max_nav_thrust)
    },
    
    FlightMode::NavigateTo(target_pos) => {
      calculate_navigation_control(
        current_position,
        current_velocity,
        current_rotation,
        target_pos,
        mass,
        max_main_thrust,
        max_retro_thrust,
        max_nav_thrust,
        _delta_time,
      )
    },
    
    FlightMode::Track(target_pos) => {
      // Similar to NavigateTo but with different tuning for responsive tracking
      calculate_tracking_control(
        current_position,
        current_velocity,
        current_rotation,
        target_pos,
        mass,
        max_main_thrust,
        max_retro_thrust,
        max_nav_thrust,
        _delta_time,
      )
    },
    
    FlightMode::Intercept { target_pos, target_vel } => {
      calculate_intercept_control(
        current_position,
        current_velocity,
        current_rotation,
        target_pos,
        target_vel,
        mass,
        max_main_thrust,
        max_retro_thrust,
        max_nav_thrust,
        _delta_time,
      )
    },
    
    FlightMode::Manual(thrust_vector) => {
      // Direct thrust control - convert to body-relative thrust
      let clamped_thrust = DVec3::new(
        thrust_vector.x.clamp(-max_retro_thrust, max_main_thrust), // Forward/back
        thrust_vector.y.clamp(-max_nav_thrust, max_nav_thrust),    // Up/down
        thrust_vector.z.clamp(-max_nav_thrust, max_nav_thrust),    // Left/right
      );
      (clamped_thrust, DVec3::zero()) // No automatic rotation in manual mode
    },
  }
}

/// Calculate attitude control - determines rotation needed to face target direction
fn calculate_attitude_control(
  current_rotation: DVec3,
  target_direction: DVec3,
  max_nav_thrust: f64,
) -> DVec3 {
  // Simplified attitude control - just point toward target
  let desired_yaw = target_direction.z.atan2(target_direction.x);
  let desired_pitch = (-target_direction.y).atan2((target_direction.x.powi(2) + target_direction.z.powi(2)).sqrt());
  
  // Calculate rotation errors
  let mut yaw_error = desired_yaw - current_rotation.y;
  let mut pitch_error = desired_pitch - current_rotation.x;
  
  // Normalize angles to -π to π for shortest rotation path
  if yaw_error > std::f64::consts::PI {
    yaw_error -= 2.0 * std::f64::consts::PI;
  } else if yaw_error < -std::f64::consts::PI {
    yaw_error += 2.0 * std::f64::consts::PI;
  }
  
  if pitch_error > std::f64::consts::PI {
    pitch_error -= 2.0 * std::f64::consts::PI;
  } else if pitch_error < -std::f64::consts::PI {
    pitch_error += 2.0 * std::f64::consts::PI;
  }
  
  // Strong attitude control gains for responsive rotation
  let attitude_gain = 25.0; // Even stronger rotation for better alignment
  let max_torque = max_nav_thrust * 3.0; // More torque available for faster rotation
  
  DVec3::new(
    (pitch_error * attitude_gain).clamp(-max_torque, max_torque),
    (yaw_error * attitude_gain).clamp(-max_torque, max_torque),
    0.0, // No roll control for now
  )
}

/// Station keeping - maintain current position by countering drift
fn calculate_hold_control(
  current_velocity: DVec3,
  current_rotation: DVec3,
  max_nav_thrust: f64,
) -> (DVec3, DVec3) {
  // Apply gentle counter-thrust to stop drift using nav thrusters only
  let damping_factor = 0.1;
  let hold_thrust = DVec3::new(
    0.0, // No main thrust for station keeping
    0.0, // No retro thrust for station keeping
    (-current_velocity.length() * damping_factor).clamp(-max_nav_thrust, max_nav_thrust),
  );
  
  // Minimal attitude control to stay stable
  let torque = DVec3::new(
    (-current_rotation.x * 0.1).clamp(-max_nav_thrust * 0.05, max_nav_thrust * 0.05),
    (-current_rotation.y * 0.1).clamp(-max_nav_thrust * 0.05, max_nav_thrust * 0.05),
    0.0,
  );
  
  (hold_thrust, torque)
}

/// Calculate thrust for precise navigation to a fixed target
fn calculate_navigation_control(
  current_position: DVec3,
  current_velocity: DVec3,
  current_rotation: DVec3,
  target_position: DVec3,
  _mass: f64,
  max_main_thrust: f64,
  _max_retro_thrust: f64,
  max_nav_thrust: f64,
  _delta_time: f64,
) -> (DVec3, DVec3) {
  let displacement = target_position - current_position;
  let distance = displacement.length();
  
  if distance < 0.1 {
    // Very close to target - use station keeping
    return calculate_hold_control(current_velocity, current_rotation, max_nav_thrust);
  }
  
  let target_direction = displacement.normalize();
  
  // Calculate attitude control first
  let torque = calculate_attitude_control(current_rotation, target_direction, max_nav_thrust);
  
  // Check if ship is pointing roughly in the right direction
  let ship_forward = DVec3::new(
    current_rotation.y.cos() * current_rotation.x.cos(),
    -current_rotation.x.sin(),
    current_rotation.y.sin() * current_rotation.x.cos(),
  );
  
  let alignment = ship_forward.dot(&target_direction);
  let alignment_threshold = 0.7; // Much stricter alignment requirement - ships must be well-aligned
  
  let thrust = if alignment > alignment_threshold {
    // Ship is well aligned - use main thrust to accelerate toward target
    let approach_speed = if distance > 15.0 {
      25.0 // Moderate speed for long distances
    } else {
      (distance * 2.0).max(1.0) // Slower approach for precision
    };
    
    let desired_velocity = target_direction * approach_speed;
    let velocity_error = desired_velocity - current_velocity;
    
    // Use main thrust along ship's forward direction
    let thrust_amount = (velocity_error.dot(&ship_forward) * 6.0).clamp(0.0, max_main_thrust);
    
    DVec3::new(thrust_amount, 0.0, 0.0) // Main thrust only when aligned
  } else {
    // Ship not aligned - NO main thrust, just rotate to face target
    // Only use very minimal nav thrust to avoid unwanted movement while rotating
    DVec3::new(0.0, 0.0, 0.0) // No thrust while rotating - let attitude control handle it
  };
  
  (thrust, torque)
}

/// Calculate thrust for responsive tracking of a moving target
fn calculate_tracking_control(
  current_position: DVec3,
  current_velocity: DVec3,
  current_rotation: DVec3,
  target_position: DVec3,
  _mass: f64,
  max_main_thrust: f64,
  _max_retro_thrust: f64,
  max_nav_thrust: f64,
  _delta_time: f64,
) -> (DVec3, DVec3) {
  // Similar to navigation but more aggressive
  calculate_navigation_control(
    current_position,
    current_velocity,
    current_rotation,
    target_position,
    _mass,
    max_main_thrust * 1.2, // More aggressive thrust
    _max_retro_thrust,
    max_nav_thrust,
    _delta_time,
  )
}

/// Calculate thrust for intercepting a moving target
fn calculate_intercept_control(
  current_position: DVec3,
  current_velocity: DVec3,
  current_rotation: DVec3,
  target_position: DVec3,
  target_velocity: DVec3,
  mass: f64,
  max_main_thrust: f64,
  max_retro_thrust: f64,
  max_nav_thrust: f64,
  delta_time: f64,
) -> (DVec3, DVec3) {
  // Predict where the target will be
  let time_to_intercept = estimate_intercept_time(
    current_position,
    current_velocity,
    target_position,
    target_velocity,
  );
  
  let predicted_target_position = target_position + target_velocity * time_to_intercept;
  
  // Navigate to the predicted position
  calculate_navigation_control(
    current_position,
    current_velocity,
    current_rotation,
    predicted_target_position,
    mass,
    max_main_thrust,
    max_retro_thrust,
    max_nav_thrust,
    delta_time,
  )
}

/// Estimate time required to intercept a moving target
fn estimate_intercept_time(
  current_position: DVec3,
  current_velocity: DVec3,
  target_position: DVec3,
  target_velocity: DVec3,
) -> f64 {
  let relative_position = target_position - current_position;
  let relative_velocity = target_velocity - current_velocity;
  
  let distance = relative_position.length();
  let closing_speed = relative_velocity.length();
  
  if closing_speed < 0.1 {
    // Not closing significantly - use simple time estimate
    distance / 10.0 // Assume average approach speed
  } else {
    distance / closing_speed
  }
}

/// Main trajectory solver - combines flight control with physics simulation
#[allow(dead_code)]
pub fn solve(
  flight_mode: FlightMode,
  mass: f64,
  max_thrust: DVec3, // x=main, y=retro, z=nav
  position: &mut DVec3,
  velocity: &mut DVec3,
  rotation: &mut DVec3,
  rotational_velocity: &mut DVec3,
) {
  const DELTA_TIME: f64 = 1.0 / 20.0; // Increased from 1/60 for more responsive simulation
  const MOMENT_OF_INERTIA: f64 = 10.0; // Much lower for faster rotation
  
  // Calculate flight control commands (thrust and torque)
  let (thrust_vector, torque_vector) = calculate_flight_control(
    flight_mode,
    *position,
    *velocity,
    *rotation,
    mass,
    max_thrust.x, // Main thrust
    max_thrust.y, // Retro thrust  
    max_thrust.z, // Nav thrust
    DELTA_TIME,
  );
  
  // Apply physics simulation
  simulate_physics(
    thrust_vector,
    torque_vector,
    mass,
    MOMENT_OF_INERTIA,
    position,
    velocity,
    rotation,
    rotational_velocity,
    DELTA_TIME,
  );
}

/// Legacy function for backward compatibility - converts target position to NavigateTo mode
#[allow(dead_code)]
pub fn solve_to_target(
  target_position: DVec3,
  mass: f64,
  max_thrust: DVec3,
  position: &mut DVec3,
  velocity: &mut DVec3,
  rotation: &mut DVec3,
  rotational_velocity: &mut DVec3,
) {
  solve(
    FlightMode::NavigateTo(target_position),
    mass,
    max_thrust,
    position,
    velocity,
    rotation,
    rotational_velocity,
  );
}
