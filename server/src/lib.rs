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
  node_rotation: DVec3,
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
  relative_rotation: DVec3,
  relative_rotational_velocity: DVec3,
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

#[reducer]
fn test_reducer(
  ctx: &ReducerContext,
  _arg: TestReducerSchedule,
) {  
  // Get or create animation counter
  let mut counter = match ctx.db.animation_counter().id().find(&1) {
    Some(c) => c,
    None => {
      // Initialize counter if it doesn't exist
      ctx.db.animation_counter().insert(AnimationCounter {
        id: 1,
        counter: 0,
      });
      ctx.db.animation_counter().id().find(&1).unwrap()
    }
  };

  // Find the TestShip entity
  if let Some(mut ship) = ctx.db.entity().designation().find(&"TestShip".to_string()) {
    // Calculate angle for smooth circular motion using counter instead of time
    // Increment counter for animation progression
    let current_counter = counter.counter;
    counter.counter += 1;
    ctx.db.animation_counter().id().update(counter);
    
    // Convert counter to smooth angle (adjust this factor to control rotation speed)
    let angle_factor = 0.02; // Adjust this to control rotation speed
    let angle = (current_counter as f64) * angle_factor;
    let radius = 5.0;
    let elevation = 5.0;
    
    // Calculate new position in a circle around the Sun
    let new_x = radius * angle.cos();
    let new_z = radius * angle.sin();
    
    // Update the ship's position
    ship.relative_position = DVec3 {
      x: new_x,
      y: elevation,
      z: new_z,
    };
    
    // Update the entity in the database
    ctx.db.entity().designation().update(ship);
  }
}

#[reducer(init)]
fn init(
  ctx: &ReducerContext,
) {
  ctx.db.node().insert(Node {
    id: 1,
    name: NodeName::Sun,
    parent_id: None,
    node_position: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    node_velocity: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    node_rotation: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    node_rotational_velocity: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    orbit_progress: 1.0,
  });

  ctx.db.entity().insert(Entity {
    id: 1,
    node: NodeName::Sun,
    designation: "TestShip".to_string(),
    relative_position: DVec3 { x: 0.0, y: 5.0, z: -5.0 },
    relative_velocity: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    relative_rotation: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    relative_rotational_velocity: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
    entity_type: EntityType::Ship,
  });

  ctx.db.test_reducer_schedule()
    .insert(TestReducerSchedule {
      scheduled_id: 1,
      scheduled_at: ScheduleAt::Interval(TimeDuration::from_micros(00_050_000)),
    });
}
