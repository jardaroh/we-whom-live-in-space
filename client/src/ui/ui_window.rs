use bevy::{math::VectorSpace, prelude::*};

#[derive(Event)]
pub struct WindowCloseEvent {
  pub window_entity: Entity,
}

#[derive(Event)]
pub struct WindowMinimizeEvent {
  pub window_entity: Entity,
}

#[derive(Event)]
pub struct WindowMaximizeEvent {
  pub window_entity: Entity,
}

#[derive(Event)]
pub struct WindowCollapseEvent {
  pub window_entity: Entity,
}

#[derive(Event)]
pub struct WindowFocusEvent {
  pub window_entity: Entity,
}

#[derive(Event)]
pub struct WindowDragEvent {
  pub window_entity: Entity,
  pub position: Vec2,
  pub drag_offset: Vec2,
  pub drag_handle: Option<ResizeHandle>,
}

#[derive(Event)]
pub struct WindowResizeEvent {
  pub window_entity: Entity,
  pub position: Vec2,
  pub size: Vec2,
  pub drag_handle: ResizeHandle,
}

#[derive(Event)]
pub struct WindowSnapEvent {
  pub window_entity: Entity,
  pub snap_position: Vec2,
  pub snap_size: Option<Vec2>,
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Static,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    None,
}

#[derive(Debug, Clone)]
pub struct SnapZone {
  pub position: Vec2,
  pub size: Vec2,
  pub snap_position: Vec2,
  pub snap_size: Option<Vec2>,
}

impl SnapZone {
  pub fn new(position: Vec2, size: Vec2) -> Self {
    Self {
      position,
      size,
      snap_position: position,
      snap_size: Some(size),
    }
  }
}

#[derive(Component, Debug, Clone)]
pub struct Window {
  pub title: String,
  pub size: Vec2,
  pub position: Vec2,
  pub z_index: i32,
  pub state: WindowState,
  pub drag_offset: Vec2,
  pub drag_handle: Option<ResizeHandle>,
}

impl Default for Window {
  fn default() -> Self {
    Self {
      title: "Window".to_string(),
      size: Vec2::new(800.0, 600.0),
      position: Vec2::new(100.0, 100.0),
      z_index: 0,
      state: WindowState::Normal,
      drag_offset: Vec2::ZERO,
      drag_handle: None,
    }
  }
}

#[derive(Component)]
pub struct WindowTitleBar {
  pub window_entity: Entity,
}

#[derive(Component)]
pub struct WindowContent;

#[derive(Component)]
pub struct WindowCloseButton {
  pub window_entity: Entity,
}

#[derive(Component)]
pub struct WindowMinimizeButton {
  pub window_entity: Entity,
}

#[derive(Component)]
pub struct WindowMaximizeButton {
  pub window_entity: Entity,
}

#[derive(Component)]
pub struct WindowCollapseButton {
  pub window_entity: Entity,
}

#[derive(Component)]
pub struct WindowResizeHandle {
  pub handle_type: ResizeHandle,
}

#[derive(Resource)]
pub struct WindowManager {
  pub next_z_index: i32,
  pub snap_threshold: f32,
  pub snap_zones: Vec<SnapZone>,
  pub is_dragging_window: bool,
  pub dragging_window_entity: Option<Entity>,
}

impl Default for WindowManager {
  fn default() -> Self {
    Self {
      next_z_index: 1,
      snap_threshold: 20.0,
      snap_zones: vec![
        SnapZone::new(Vec2::ZERO, Vec2::new(50.0, 50.0)),
      ],
      is_dragging_window: false,
      dragging_window_entity: None,
    }
  }
}

// Systems < ========================================================================= |
fn handle_window_focus_click(
  mut interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Window>)>,
  mut focus_events: EventWriter<WindowFocusEvent>,
  mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
  for (interaction, window_entity) in interaction_query.iter_mut() {
    if *interaction == Interaction::Pressed && mouse_buttons.just_pressed(MouseButton::Left) {
      focus_events.write(WindowFocusEvent {
        window_entity,
      });
    }
  }
}

fn handle_window_interactions(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &WindowTitleBar), (Changed<Interaction>)>,
    mut window_query: Query<&mut Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut focus_events: EventWriter<WindowFocusEvent>,
    mut drag_events: EventWriter<WindowDragEvent>,
) {
  for (interaction, mut bg_color, title_bar) in interaction_query.iter_mut() {
    match *interaction {
      Interaction::Pressed => {
        *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
        
        // Find the window this title bar belongs to
        // For now, we'll need to traverse up to find the window
        // This is a simplified approach - you might want to store window entity on title bar
        if mouse_buttons.just_pressed(MouseButton::Left) {
          focus_events.write(WindowFocusEvent {
            window_entity: title_bar.window_entity, // This should be the actual window entity
          });

          if let Ok(mut window) = window_query.get_mut(title_bar.window_entity) {
            window.drag_handle = Some(ResizeHandle::None); // use None to indicate dragging
          }
          
          // Start dragging
          drag_events.write(WindowDragEvent {
            window_entity: title_bar.window_entity, // This should be the actual window entity
            position: Vec2::ZERO, // You'll need to get cursor position
            drag_offset: Vec2::ZERO,
            drag_handle: None,
          });
        }
      }
      Interaction::Hovered => {
        *bg_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
      }
      Interaction::None => {
        *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
      }
    }
  }
}

fn handle_window_drag(
    mut drag_events: EventReader<WindowDragEvent>,
    mut window_query: Query<(&mut Window, &mut Node)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    primary_window: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
    mut last_cursor_pos: Local<Option<Vec2>>,
    mut window_manager: ResMut<WindowManager>,
) {
  for event in drag_events.read() {
    if let Ok(window) = primary_window.single() {
      if let Some(cursor_pos) = window.cursor_position() {
        *last_cursor_pos = Some(cursor_pos);

        if let Ok((mut win, _)) = window_query.get_mut(event.window_entity) {
          win.drag_offset = cursor_pos - win.position;

          window_manager.is_dragging_window = true;
          window_manager.dragging_window_entity = Some(event.window_entity);
        }
      }
    }
  }

  if mouse_buttons.pressed(MouseButton::Left) {
    if let Ok(window) = primary_window.single() {
      if let Some(cursor_pos) = window.cursor_position() {
        if let Some(last_pos) = *last_cursor_pos {
          let delta = cursor_pos - last_pos;

          let screen_width = window.width();
          let screen_height = window.height();

          for (mut win, mut node) in window_query.iter_mut() {
            if win.drag_handle.is_some() {
              let new_position = win.position + delta;
              
              // Strict clamping - window cannot go outside screen bounds
              let clamped_x = new_position.x.clamp(0.0, screen_width - win.size.x);
              let clamped_y = new_position.y.clamp(0.0, screen_height - win.size.y);
              
              win.position = Vec2::new(clamped_x, clamped_y);
              node.left = Val::Px(win.position.x);
              node.top = Val::Px(win.position.y);
            }
          }
        }
        *last_cursor_pos = Some(cursor_pos);
      }
    }
  } else {
    for (mut win, _) in window_query.iter_mut() {
      win.drag_handle = None; // Reset drag handle when mouse button is released
    }
    *last_cursor_pos = None; // Reset last cursor position

    window_manager.is_dragging_window = false;
    window_manager.dragging_window_entity = None;
  }
}

fn handle_window_resize(
    mut resize_events: EventReader<WindowResizeEvent>,
    mut window_query: Query<&mut Window>,
) {
    // TODO: Implement resize handling
}

fn handle_window_focus(
    mut focus_events: EventReader<WindowFocusEvent>,
    mut window_query: Query<&mut Window>,
    mut window_manager: ResMut<WindowManager>,
) {
  for event in focus_events.read() {
    if let Ok(mut window) = window_query.get_mut(event.window_entity) {
      window.z_index = window_manager.next_z_index;
      window_manager.next_z_index += 1;
    }
  }
}

fn handle_window_snap(
    mut snap_events: EventReader<WindowSnapEvent>,
    mut window_query: Query<&mut Window>,
    window_manager: Res<WindowManager>,
) {
    // TODO: Implement snapping logic
}

fn handle_window_buttons(
    mut interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, Or<(With<WindowCloseButton>, With<WindowMinimizeButton>, With<WindowMaximizeButton>, With<WindowCollapseButton>)>)>,
    close_query: Query<&WindowCloseButton>,
    minimize_query: Query<&WindowMinimizeButton>, 
    maximize_query: Query<&WindowMaximizeButton>,
    collapse_query: Query<&WindowCollapseButton>,
    mut close_events: EventWriter<WindowCloseEvent>,
    mut minimize_events: EventWriter<WindowMinimizeEvent>,
    mut maximize_events: EventWriter<WindowMaximizeEvent>,
    mut collapse_events: EventWriter<WindowCollapseEvent>,
) {
  for (interaction, button_entity) in interaction_query.iter() {
    if *interaction == Interaction::Pressed {
      if let Ok(close_button) = close_query.get(button_entity) {
        close_events.write(WindowCloseEvent { 
          window_entity: close_button.window_entity
        });
      } else if let Ok(minimize_button) = minimize_query.get(button_entity) {
        minimize_events.write(WindowMinimizeEvent {
          window_entity: minimize_button.window_entity
        });
      } else if let Ok(maximize_button) = maximize_query.get(button_entity) {
        maximize_events.write(WindowMaximizeEvent {
          window_entity: maximize_button.window_entity
        });
      } else if let Ok(collapse_button) = collapse_query.get(button_entity) {
        collapse_events.write(WindowCollapseEvent {
          window_entity: collapse_button.window_entity
        });
      }
    }
  }
}

fn update_window_z_order(
    mut window_query: Query<(Entity, &Window)>,
    mut commands: Commands,
) {
  for (mut entity, window) in window_query.iter_mut() {
    commands.entity(entity).insert(ZIndex(window.z_index));
  }
}

// Helper functions <================================================================= |
pub fn create_window(
  commands: &mut Commands,
  window_manager: &mut ResMut<WindowManager>,
  title: &str,
  size: Vec2,
  position: Vec2,
) -> Entity {
  let window_entity = commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      left: Val::Px(position.x),
      top: Val::Px(position.y),
      width: Val::Px(size.x),
      height: Val::Px(size.y),
      border: UiRect::all(Val::Px(1.0)),
      flex_direction: FlexDirection::Column,
      ..default()
    },
    BorderColor(Color::srgb(0.3, 0.3, 0.3)),
    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    ZIndex(window_manager.next_z_index),
    Interaction::default(),
    Window {
      title: title.to_string(),
      size,
      position,
      z_index: window_manager.next_z_index,
      ..default()
    },
  )).id();

  window_manager.next_z_index += 1;

  let title_bar = commands.spawn((
    Node {
      width: Val::Percent(100.0),
      height: Val::Px(30.0),
      flex_direction: FlexDirection::Row,
      justify_content: JustifyContent::SpaceBetween,
      align_items: AlignItems::Center,
      padding: UiRect::all(Val::Px(8.0)),
      ..default()
    },
    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    Interaction::default(),
    WindowTitleBar {
      window_entity,
    },
  )).id();

  let title_text = commands.spawn((
    Text::new(title),
    TextFont {
      font_size: 14.0,
      ..default()
    },
    TextColor(Color::srgb(0.9, 0.9, 0.9)),
  )).id();

  let buttons_container = commands.spawn((
    Node {
      flex_direction: FlexDirection::Row,
      column_gap: Val::Px(4.0),
      ..default()
    },
  )).id();

  let minimize_button = commands.spawn((
    Node {
      width: Val::Px(20.0),
      height: Val::Px(20.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
    BorderRadius::all(Val::Px(2.0)),
    Interaction::default(),
    WindowMinimizeButton {
      window_entity,
    },
  )).with_children(|parent| {
    parent.spawn((
      Text::new("-"),
      TextFont {
        font_size: 12.0,
        ..default()
      },
      TextColor(Color::srgb(0.9, 0.9, 0.9)),
    ));
  }).id();

  let close_button = commands.spawn((
    Node {
      width: Val::Px(20.0),
      height: Val::Px(20.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    BackgroundColor(Color::srgb(0.7, 0.7, 0.7)),
    BorderRadius::all(Val::Px(2.0)),
    Interaction::default(),
    WindowCloseButton {
      window_entity,
    },
  )).with_children(|parent| {
    parent.spawn((
      Text::new("×"),
      TextFont {
        font_size: 12.0,
        ..default()
      },
      TextColor(Color::srgb(0.9, 0.9, 0.9)),
    ));
  }).id();

  let content_area = commands.spawn((
    Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      padding: UiRect::all(Val::Px(8.0)),
      flex_direction: FlexDirection::Column,
      ..default()
    },
    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    WindowContent,
  )).with_children(|parent| {
    parent.spawn((
      Text::new("This is the window content area."),
      TextFont {
        font_size: 12.0,
        ..default()
      },
      TextColor(Color::srgb(0.8, 0.8, 0.8)),
    ));
  }).id();

  commands.entity(buttons_container).add_children(&[minimize_button, close_button]);
  commands.entity(title_bar).add_children(&[title_text, buttons_container]);
  commands.entity(window_entity).add_children(&[title_bar, content_area]);

  window_entity
}

// Plugin < ========================================================================== |
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<WindowCloseEvent>()
      .add_event::<WindowMinimizeEvent>()
      .add_event::<WindowMaximizeEvent>()
      .add_event::<WindowCollapseEvent>()
      .add_event::<WindowFocusEvent>()
      .add_event::<WindowDragEvent>()
      .add_event::<WindowResizeEvent>()
      .add_event::<WindowSnapEvent>()
      .init_resource::<WindowManager>()
      .add_systems(Update, (
        handle_window_focus_click,
        handle_window_interactions,
        handle_window_drag,
        handle_window_resize,
        handle_window_focus,
        handle_window_snap,
        handle_window_buttons,
        update_window_z_order,
      ).chain());
  }
}


