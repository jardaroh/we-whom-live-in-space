use bevy::{
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  input_focus::tab_navigation::TabGroup,
  prelude::*,
  winit::WinitSettings,
};
// use rand crate for random number generation
use rand::Rng;

mod constants;

mod ui;
mod camera;
mod space;
mod mesh_utils;
mod movement;

use ui::ui_theme::Theme;
use ui::ui_button::button;
use ui::ui_checkbox::checkbox;
use ui::ui_input::text_input;

use ui::ui_plugin::ui_plugin;
use camera::camera_plugin::camera_plugin;
use space::space_plugin::space_plugin;
use mesh_utils::mesh_utils_plugin;

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Asteroid;

fn setup_ui_test(mut commands: Commands, theme: Res<Theme>) {
  commands.spawn((
    Node {
      display: Display::Grid,
      grid_template_columns: vec![GridTrack::min_content(); 1],
      grid_template_rows: vec![GridTrack::min_content(); 4],
      row_gap: Val::Px(6.0),
      ..default()
    },
    TabGroup::new(0),
    children![
      button(
        &theme,
        "Click Me",
        (Val::Px(200.0), Val::Px(50.0)),
      ),
      checkbox(
        &theme,
        false,
      ),
      checkbox(
        &theme,
        true,
      ),
      text_input(
        &theme,
        "Type here...",
      ),
    ],
  ));
}

pub fn setup_camera_test(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 5.0));
  commands.spawn((
    Mesh3d(cube_mesh),
    MeshMaterial3d(materials.add(StandardMaterial {
      base_color: Srgba::hex("#ffd891").unwrap().into(),
      metallic: 0.25,
      perceptual_roughness: 0.25,
      ..default()
    })),
    Transform::from_xyz(0.0, 0.0, 0.0),
    Ship,
  ));

  commands.spawn((
    DirectionalLight {
      illuminance: 1_5000.0,
      ..default()
    },
    Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
  ));

  // Spawn asteroids
  let sphere_mesh = meshes.add(Sphere::new(1.0));
  for _ in 0..300 {
    let x = (rand::random::<f32>() - 0.5) * 500.0;
    let y = (rand::random::<f32>() - 0.5) * 500.0;
    let z = (rand::random::<f32>() - 0.5) * 500.0;
    let scale = 0.5 + rand::random::<f32>() * 2.0;
    
    let color = match rand::random::<u8>() % 4 {
      0 => Srgba::hex("#8c8c8c").unwrap(),  // Gray
      1 => Srgba::hex("#a67c52").unwrap(),  // Brown
      2 => Srgba::hex("#5c4033").unwrap(),  // Dark brown
      _ => Srgba::hex("#666666").unwrap(),  // Darker gray
    };

    commands.spawn((
      Mesh3d(sphere_mesh.clone()),
      MeshMaterial3d(materials.add(StandardMaterial {
        base_color: color.into(),
        metallic: 0.1,
        perceptual_roughness: 0.9,
        ..default()
      })),
      Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale)),
      Asteroid,
    ));
  }
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(WinitSettings::default())
    .add_plugins((
      FrameTimeDiagnosticsPlugin::default(),
      LogDiagnosticsPlugin::default(),
      ui_plugin,
      camera_plugin,
      space_plugin,
      mesh_utils_plugin,
    ))
    .add_systems(Startup, (
      setup_ui_test,
      setup_camera_test,
    ))
    
    .run();
}
