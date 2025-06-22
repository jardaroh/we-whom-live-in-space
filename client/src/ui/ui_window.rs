use std::os::unix::process;

use bevy::{asset::UnknownTyped, ecs::relationship::RelatedSpawnerCommands, math::VectorSpace, picking::window, prelude::*, ui::FocusPolicy};

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
pub struct WindowContentUpdateEvent {
  pub window_entity: Entity,
  pub new_content: Entity,
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

#[derive(Component)]
pub struct ResizeHandleRef {
  pub window_entity: Entity,
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
  pub is_collapsed: bool,
  pub uncollapsed_size: Vec2,
  pub is_maximized: bool,
  pub pre_maximized_size: Vec2,
  pub pre_maximized_position: Vec2,
  pub is_minimized: bool,
  pub pre_minimized_size: Vec2,
  pub pre_minimized_position: Vec2,
  pub minimize_slot: Option<usize>,
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
      is_collapsed: false,
      uncollapsed_size: Vec2::new(800.0, 600.0),
      is_maximized: false,
      pre_maximized_size: Vec2::new(800.0, 600.0),
      pre_maximized_position: Vec2::new(100.0, 100.0),
      is_minimized: false,
      pre_minimized_size: Vec2::new(800.0, 600.0),
      pre_minimized_position: Vec2::new(100.0, 100.0),
      minimize_slot: None,
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
pub struct WindowContentArea {
  pub window_entity: Entity,
}

#[derive(Component)]
pub struct WindowResizeHandle {
  pub handle_type: ResizeHandle,
}

#[derive(Resource)]
pub struct WindowManager {
  pub next_z_index: i32,
  pub is_dragging_window: bool,
  pub is_resizing_window: bool,
  pub dragging_window_entity: Option<Entity>,
  pub pending_content_updates: Vec<(Entity, Box<dyn FnOnce(&mut RelatedSpawnerCommands<ChildOf>) + Send + Sync>)>,
}

impl Default for WindowManager {
  fn default() -> Self {
    Self {
      next_z_index: 1,
      is_dragging_window: false,
      is_resizing_window: false,
      dragging_window_entity: None,
      pending_content_updates: vec![],
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
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &WindowTitleBar), Changed<Interaction>>,
    mut window_query: Query<&mut Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut focus_events: EventWriter<WindowFocusEvent>,
    mut drag_events: EventWriter<WindowDragEvent>,
    window_manager: Res<WindowManager>,
) {
  for (interaction, mut bg_color, title_bar) in interaction_query.iter_mut() {
    match *interaction {
      Interaction::Pressed => {
        *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
        
        // Find the window this title bar belongs to
        // For now, we'll need to traverse up to find the window
        // This is a simplified approach - you might want to store window entity on title bar
        if mouse_buttons.just_pressed(MouseButton::Left) && !window_manager.is_resizing_window {
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

fn handle_resize_interactions(
  mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &WindowResizeHandle, &ResizeHandleRef), Changed<Interaction>>,
  mut window_query: Query<&mut Window>,
  mouse_buttons: Res<ButtonInput<MouseButton>>,
  mut resize_events: EventWriter<WindowResizeEvent>,
  primary_window: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
  mut window_manager: ResMut<WindowManager>,
) {
  for (interaction, mut bg_color, resize_handle, handle_ref) in interaction_query.iter_mut() {
    match *interaction {
      Interaction::Pressed => {
        *bg_color = BackgroundColor(Color::srgba(0.7, 0.7, 0.7, 0.5)); // Make visible when pressed
        
        if mouse_buttons.just_pressed(MouseButton::Left) {
          window_manager.is_resizing_window = true;

          if let Ok(window) = primary_window.single() {
            if let Some(cursor_pos) = window.cursor_position() {
              if let Ok(mut win) = window_query.get_mut(handle_ref.window_entity) {
                win.drag_handle = Some(resize_handle.handle_type);
                
                resize_events.write(WindowResizeEvent {
                  window_entity: handle_ref.window_entity,
                  position: cursor_pos,
                  size: win.size,
                  drag_handle: resize_handle.handle_type,
                });
              }
            }
          }
        }
      }
      Interaction::Hovered => {
        *bg_color = BackgroundColor(Color::srgba(0.6, 0.6, 0.6, 0.3)); // Semi-visible on hover
      }
      Interaction::None => {
        *bg_color = BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.0)); // Invisible
      }
    }
  }
}

fn handle_window_drag(
    mut drag_events: EventReader<WindowDragEvent>,
    mut window_query: Query<(Entity, &mut Window, &mut Node)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    primary_window: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
    mut last_cursor_pos: Local<Option<Vec2>>,
    mut window_manager: ResMut<WindowManager>,
) {
  for event in drag_events.read() {
    if let Ok((entity, window, _)) = window_query.get(event.window_entity) {
      if window.is_maximized || window.is_minimized {
        continue; // Skip dragging for maximized windows
      }
    }
    
    if let Ok(window) = primary_window.single() {
      if let Some(cursor_pos) = window.cursor_position() {
        *last_cursor_pos = Some(cursor_pos);

        if let Ok((entity, mut win, _)) = window_query.get_mut(event.window_entity) {
          win.drag_offset = cursor_pos - win.position;
          win.drag_handle = Some(ResizeHandle::None);
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
          let screen_size = Vec2::new(screen_width, screen_height);

          let mut other_windows: Vec<(Vec2, Vec2)> = Vec::new();
          let dragging_entity = window_manager.dragging_window_entity;

          for (entity, win, _) in window_query.iter() {
            if win.drag_handle == Some(ResizeHandle::None) || win.is_maximized {
              continue;
            }
            if let Some(dragging) = dragging_entity {
              if win.z_index == window_query.get(dragging).map(|(e, w, _)| w.z_index).unwrap_or(-1) {
                continue;
              }
            }
            other_windows.push((win.position, win.size));
          }

          for (entity, mut win, mut node) in window_query.iter_mut() {
            if win.drag_handle == Some(ResizeHandle::None) && !win.is_maximized && !win.is_minimized {
              if let Some(dragging_entity) = window_manager.dragging_window_entity {
                let proposed_position = win.position + delta;
                if dragging_entity == entity {
                  // Strict clamping - window cannot go outside screen bounds
                  let clamped_x = proposed_position.x.clamp(0.0, screen_width - win.size.x);
                  let clamped_y = proposed_position.y.clamp(0.0, screen_height - win.size.y);
                  
                  win.position = Vec2::new(clamped_x, clamped_y);
                  node.left = Val::Px(win.position.x);
                  node.top = Val::Px(win.position.y);
                }
              };
            }
          }
        }
        *last_cursor_pos = Some(cursor_pos);
      }
    }
  } else {
    for (_entity, mut win, _) in window_query.iter_mut() {
      if win.drag_handle == Some(ResizeHandle::None) {
        win.drag_handle = None;
      }
    }
    *last_cursor_pos = None;
    window_manager.is_dragging_window = false;
    window_manager.dragging_window_entity = None;
  }
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

fn handle_window_resize(
  mut resize_events: EventReader<WindowResizeEvent>,
  mut window_query: Query<(Entity, &mut Window, &mut Node)>,
  title_bar_query: Query<&Node, (With<WindowTitleBar>, Without<Window>)>,
  content_query: Query<&Node, (With<WindowContentArea>, Without<Window>, Without<WindowTitleBar>)>,
  children_query: Query<&Children>,
  node_query: Query<&Node, (Without<Window>, Without<WindowTitleBar>, Without<WindowContentArea>)>,
  mouse_buttons: Res<ButtonInput<MouseButton>>,
  primary_window: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
  mut last_cursor_pos: Local<Option<Vec2>>,
  mut window_manager: ResMut<WindowManager>,
) {
  // Start resize
  for event in resize_events.read() {
    if let Ok((entity, window, _)) = window_query.get(event.window_entity) {
      if window.is_collapsed || window.is_maximized || window.is_minimized {
        continue;
      }
    }

    if let Ok(window) = primary_window.single() {
      if let Some(cursor_pos) = window.cursor_position() {
        *last_cursor_pos = Some(cursor_pos);
        window_manager.is_dragging_window = true;
        window_manager.dragging_window_entity = Some(event.window_entity);
      }
    }
  }

  // Handle ongoing resize
  if mouse_buttons.pressed(MouseButton::Left) {
    if let Ok(window) = primary_window.single() {
      if let Some(cursor_pos) = window.cursor_position() {
        if let Some(last_pos) = *last_cursor_pos {
          let delta = cursor_pos - last_pos;
          let screen_width = window.width();
          let screen_height = window.height();

          // Only resize the window that's being resized
          if let Some(resizing_entity) = window_manager.dragging_window_entity {
            if let Ok((entity, mut win, mut node)) = window_query.get_mut(resizing_entity) {
              if win.is_collapsed || win.is_maximized || win.is_minimized {
                return; // Skip resizing for collapsed, maximized, or minimized windows
              }

              if let Some(handle_type) = win.drag_handle {
                if handle_type != ResizeHandle::None {
                  // Calculate minimum size for this specific window
                  let min_size = calculate_window_minimum_size(
                    entity, // Use the correct entity
                    &title_bar_query,
                    &content_query,
                    &children_query,
                    &node_query,
                  );

                  let mut new_position = win.position;
                  let mut new_size = win.size;
                  
                  match handle_type {
                    ResizeHandle::Right => {
                      new_size.x = (win.size.x + delta.x).max(min_size.x);
                      let max_width = screen_width - win.position.x;
                      new_size.x = new_size.x.min(max_width);
                    }
                    ResizeHandle::Bottom => {
                      new_size.y = (win.size.y + delta.y).max(min_size.y);
                      let max_height = screen_height - win.position.y;
                      new_size.y = new_size.y.min(max_height);
                    }
                    ResizeHandle::BottomRight => {
                      new_size.x = (win.size.x + delta.x).max(min_size.x);
                      new_size.y = (win.size.y + delta.y).max(min_size.y);
                      let max_width = screen_width - win.position.x;
                      let max_height = screen_height - win.position.y;
                      new_size.x = new_size.x.min(max_width);
                      new_size.y = new_size.y.min(max_height);
                    }
                    ResizeHandle::Left => {
                      let proposed_width = (win.size.x - delta.x).max(min_size.x);
                      new_position.x = win.position.x + (win.size.x - proposed_width);
                      if new_position.x >= 0.0 {
                        new_size.x = proposed_width;
                      }
                    }
                    ResizeHandle::Top => {
                      let proposed_height = (win.size.y - delta.y).max(min_size.y);
                      new_position.y = win.position.y + (win.size.y - proposed_height);
                      if new_position.y >= 0.0 {
                        new_size.y = proposed_height;
                      }
                    }
                    ResizeHandle::TopLeft => {
                      let proposed_width = (win.size.x - delta.x).max(min_size.x);
                      let proposed_height = (win.size.y - delta.y).max(min_size.y);
                      let proposed_pos_x = win.position.x + (win.size.x - proposed_width);
                      let proposed_pos_y = win.position.y + (win.size.y - proposed_height);
                      
                      if proposed_pos_x >= 0.0 && proposed_pos_y >= 0.0 {
                        new_size.x = proposed_width;
                        new_size.y = proposed_height;
                        new_position.x = proposed_pos_x;
                        new_position.y = proposed_pos_y;
                      }
                    }
                    ResizeHandle::TopRight => {
                      let proposed_width = (win.size.x + delta.x).max(min_size.x);
                      let proposed_height = (win.size.y - delta.y).max(min_size.y);
                      let proposed_pos_y = win.position.y + (win.size.y - proposed_height);
                      let max_width = screen_width - win.position.x;
                      
                      if proposed_pos_y >= 0.0 {
                        new_size.x = proposed_width.min(max_width);
                        new_size.y = proposed_height;
                        new_position.y = proposed_pos_y;
                      }
                    }
                    ResizeHandle::BottomLeft => {
                      let proposed_width = (win.size.x - delta.x).max(min_size.x);
                      let proposed_height = (win.size.y + delta.y).max(min_size.y);
                      let proposed_pos_x = win.position.x + (win.size.x - proposed_width);
                      let max_height = screen_height - win.position.y;
                      
                      if proposed_pos_x >= 0.0 {
                        new_size.x = proposed_width;
                        new_size.y = proposed_height.min(max_height);
                        new_position.x = proposed_pos_x;
                      }
                    }
                    _ => {}
                  }
                  win.position = new_position;
                  win.size = new_size;
                  node.left = Val::Px(win.position.x);
                  node.top = Val::Px(win.position.y);
                  node.width = Val::Px(win.size.x);
                  node.height = Val::Px(win.size.y);
                }
              }
            }
          }
        }
        *last_cursor_pos = Some(cursor_pos);
      }
    }
  } else {
    // Stop resizing - rest of the function remains the same
    for (_entity, mut win, _) in window_query.iter_mut() {
      if let Some(handle_type) = win.drag_handle {
        if handle_type != ResizeHandle::None {
          win.drag_handle = None;
        }
      }
    }
    *last_cursor_pos = None;
    
    window_manager.is_resizing_window = false;
    if window_manager.is_dragging_window {
      window_manager.is_dragging_window = false;
      window_manager.dragging_window_entity = None;
    }
  }
}

fn enforce_window_minimum_sizes(
  mut window_query: Query<(Entity, &mut Window, &mut Node), Changed<Window>>,
  title_bar_query: Query<&Node, (With<WindowTitleBar>, Without<Window>)>,
  content_query: Query<&Node, (With<WindowContentArea>, Without<Window>, Without<WindowTitleBar>)>,
  children_query: Query<&Children>,
  node_query: Query<&Node, (Without<Window>, Without<WindowTitleBar>, Without<WindowContentArea>)>,
) {
  for (entity, mut window, mut node) in window_query.iter_mut() {
    let min_size = calculate_window_minimum_size(
      entity,
      &title_bar_query,
      &content_query,
      &children_query,
      &node_query,
    );
    
    // Enforce minimum size
    if window.size.x < min_size.x || window.size.y < min_size.y {
      window.size.x = window.size.x.max(min_size.x);
      window.size.y = window.size.y.max(min_size.y);
      
      // Update UI node
      node.width = Val::Px(window.size.x);
      node.height = Val::Px(window.size.y);
    }
  }
}

fn handle_window_buttons(
    mut interaction_query: Query<
      (&Interaction, Entity),
      (Changed<Interaction>, Or<(
        With<WindowCloseButton>,
        With<WindowMinimizeButton>,
        With<WindowMaximizeButton>,
        With<WindowCollapseButton>,
      )>)>,
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

fn handle_window_close(
    mut close_events: EventReader<WindowCloseEvent>,
    mut commands: Commands,
    window_query: Query<Entity, With<Window>>,
) {
    for event in close_events.read() {
        if let Ok(window_entity) = window_query.get(event.window_entity) {
            // Despawn the window and all its children recursively
            commands.entity(window_entity).despawn();
        }
    }
}

fn handle_window_collapse(
    mut collapse_events: EventReader<WindowCollapseEvent>,
    mut window_query: Query<(&mut Window, &mut Node)>,
    mut content_query: Query<&mut Visibility, (With<WindowContent>, Without<Window>)>,
    mut resize_handle_query: Query<&mut Visibility, (With<WindowResizeHandle>, Without<Window>, Without<WindowContent>)>,
    children_query: Query<&Children>,
) {
    for event in collapse_events.read() {
        if let Ok((mut window, mut window_node)) = window_query.get_mut(event.window_entity) {
            window.is_collapsed = !window.is_collapsed;
            
            if window.is_collapsed {
                // Store the current size before collapsing
                window.uncollapsed_size = window.size;
                
                // Set window height to just the title bar (30px + padding)
                let collapsed_height = 30.0;
                window.size.y = collapsed_height;
                window_node.height = Val::Px(collapsed_height);
                
                // Hide content area and resize handles
                hide_window_children(event.window_entity, &children_query, &mut content_query, &mut resize_handle_query, false);
            } else {
                // Restore the original size
                window.size = window.uncollapsed_size;
                window_node.width = Val::Px(window.size.x);
                window_node.height = Val::Px(window.size.y);
                
                // Show content area and resize handles
                hide_window_children(event.window_entity, &children_query, &mut content_query, &mut resize_handle_query, true);
            }
        }
    }
}

fn handle_window_maximize(
    mut maximize_events: EventReader<WindowMaximizeEvent>,
    mut window_query: Query<(&mut Window, &mut Node)>,
    mut content_query: Query<&mut Visibility, (With<WindowContent>, Without<Window>)>,
    mut resize_handle_query: Query<&mut Visibility, (With<WindowResizeHandle>, Without<Window>, Without<WindowContent>)>,
    children_query: Query<&Children>,
    primary_window: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
) {
    for event in maximize_events.read() {
        if let Ok((mut window, mut window_node)) = window_query.get_mut(event.window_entity) {
            if let Ok(screen) = primary_window.single() {
                window.is_maximized = !window.is_maximized;
                
                if window.is_maximized {
                    // Store the current size and position before maximizing
                    window.pre_maximized_size = window.size;
                    window.pre_maximized_position = window.position;
                    
                    // If the window is collapsed, also store the uncollapsed size and expand it
                    if window.is_collapsed {
                        window.is_collapsed = false;
                        window.uncollapsed_size = window.pre_maximized_size;
                        // Show content when maximizing a collapsed window
                        hide_window_children(event.window_entity, &children_query, &mut content_query, &mut resize_handle_query, true);
                    }
                    
                    // Set to fullscreen
                    window.position = Vec2::ZERO;
                    window.size = Vec2::new(screen.width(), screen.height());
                    window.state = WindowState::Maximized;
                    
                    // Update the UI node
                    window_node.left = Val::Px(0.0);
                    window_node.top = Val::Px(0.0);
                    window_node.width = Val::Px(screen.width());
                    window_node.height = Val::Px(screen.height());
                    
                    // Hide resize handles when maximized
                    hide_resize_handles(event.window_entity, &children_query, &mut resize_handle_query, false);
                } else {
                    // Restore the original size and position
                    window.position = window.pre_maximized_position;
                    window.size = window.pre_maximized_size;
                    window.state = WindowState::Normal;
                    
                    // Update the UI node
                    window_node.left = Val::Px(window.position.x);
                    window_node.top = Val::Px(window.position.y);
                    window_node.width = Val::Px(window.size.x);
                    window_node.height = Val::Px(window.size.y);
                    
                    // Show resize handles when unmaximized
                    hide_resize_handles(event.window_entity, &children_query, &mut resize_handle_query, true);
                }
            }
        }
    }
}

fn handle_window_minimize(
  mut minimize_events: EventReader<WindowMinimizeEvent>,
  mut window_query: Query<(&mut Window, &mut Node)>,
  mut content_query: Query<&mut Visibility, (With<WindowContent>, Without<Window>)>,
  mut resize_handle_query: Query<&mut Visibility, (With<WindowResizeHandle>, Without<Window>, Without<WindowContent>)>,
  children_query: Query<&Children>,
  primary_window: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
  mut window_manager: ResMut<WindowManager>,
) {
  for event in minimize_events.read() {
    // First, collect the used slots before we start mutating
    let mut used_slots = std::collections::HashSet::new();
    for (window, _) in window_query.iter() {
      if window.is_minimized {
        if let Some(slot) = window.minimize_slot {
          used_slots.insert(slot);
        }
      }
    }
    
    // Find the first available slot
    let mut slot = 0;
    while used_slots.contains(&slot) {
      slot += 1;
    }
    
    // Now handle the minimize event
    if let Ok((mut window, mut window_node)) = window_query.get_mut(event.window_entity) {
      if let Ok(screen) = primary_window.single() {
        window.is_minimized = !window.is_minimized;
        
        if window.is_minimized {
          // Store the current size and position before minimizing
          window.pre_minimized_size = window.size;
          window.pre_minimized_position = window.position;
          
          // If the window is maximized, restore to normal first
          if window.is_maximized {
            window.is_maximized = false;
            window.size = window.pre_maximized_size;
            window.position = window.pre_maximized_position;
          }
          
          // If the window is collapsed, expand it first then minimize
          if window.is_collapsed {
            window.is_collapsed = false;
            window.size = window.uncollapsed_size;
          }
          
          window.minimize_slot = Some(slot);
          
          // Calculate position in taskbar
          let taskbar_position = calculate_minimize_position(slot, screen.width(), screen.height());
          window.position = taskbar_position;
          window.size = Vec2::new(200.0, 30.0); // Taskbar window size
          window.state = WindowState::Minimized;
          
          // Update the UI node
          window_node.left = Val::Px(window.position.x);
          window_node.top = Val::Px(window.position.y);
          window_node.width = Val::Px(window.size.x);
          window_node.height = Val::Px(window.size.y);
          
          // Hide content and resize handles
          hide_window_children(event.window_entity, &children_query, &mut content_query, &mut resize_handle_query, false);
          hide_resize_handles(event.window_entity, &children_query, &mut resize_handle_query, false);
        } else {
          // Restore the window from minimized state
          window.position = window.pre_minimized_position;
          window.size = window.pre_minimized_size;
          window.state = WindowState::Normal;
          window.minimize_slot = None;
          
          // Bring window to front
          window.z_index = window_manager.next_z_index;
          window_manager.next_z_index += 1;
          
          // Update the UI node
          window_node.left = Val::Px(window.position.x);
          window_node.top = Val::Px(window.position.y);
          window_node.width = Val::Px(window.size.x);
          window_node.height = Val::Px(window.size.y);
          
          // Show content and resize handles
          hide_window_children(event.window_entity, &children_query, &mut content_query, &mut resize_handle_query, true);
          hide_resize_handles(event.window_entity, &children_query, &mut resize_handle_query, true);
        }
      }
    }
  }
}

pub fn process_pending_content_updates(
  mut commands: Commands,
  mut window_manager: ResMut<WindowManager>,
  content_area_query: Query<(Entity, &WindowContentArea)>,
) {
  let updates = std::mem::take(&mut window_manager.pending_content_updates);

  for (window_entity, content_builder) in updates {
    for (content_area_entity, content_area) in content_area_query.iter() {
      if content_area.window_entity == window_entity {
        commands.entity(content_area_entity).despawn_related::<Children>();
        commands.entity(content_area_entity).with_children(content_builder);
        break;
      }
    }
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
    FocusPolicy::Block,
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
      overflow: Overflow::clip(),
      ..default()
    },
    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    Interaction::default(),
    WindowTitleBar {
      window_entity,
    },
  )).id();

  let title_container = commands.spawn(Node {
      flex_direction: FlexDirection::Row,
      align_items: AlignItems::Center,
      flex_grow: 1.0,
      flex_shrink: 1.0,
      min_width: Val::Px(0.0),
      overflow: Overflow::clip(),
      ..default()
    }).id();

  let title_text = commands.spawn((
    Text::new(title),
    TextFont {
      font_size: 14.0,
      ..default()
    },
    TextColor(Color::srgb(0.9, 0.9, 0.9)),
    TextLayout::new_with_no_wrap(),
    Node {
      max_width: Val::Percent(100.0),
      overflow: Overflow::clip(),
      ..default()
    },
  )).id();

  let buttons_container = commands.spawn((
    Node {
      flex_direction: FlexDirection::Row,
      column_gap: Val::Px(4.0),
      flex_shrink: 0.0,
      align_items: AlignItems::Center,
      ..default()
    },
  )).id();

  let collapse_button = commands.spawn((
    Node {
      width: Val::Px(20.0),
      height: Val::Px(20.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
    BorderRadius::all(Val::Px(2.0)),
    Interaction::default(),
    WindowCollapseButton {
      window_entity,
    },
  )).with_children(|parent| {
    parent.spawn((
      Text::new("_"),
      TextFont {
        font_size: 12.0,
        ..default()
      },
      TextColor(Color::srgb(0.9, 0.9, 0.9)),
    ));
  }).id();

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

  let maximize_button = commands.spawn((
    Node {
      width: Val::Px(20.0),
      height: Val::Px(20.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
    BorderRadius::all(Val::Px(2.0)),
    Interaction::default(),
    WindowMaximizeButton {
      window_entity,
    },
  )).with_children(|parent| {
    parent.spawn((
      Text::new("□"), // Square symbol for maximize
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
    WindowContentArea {
      window_entity,
    },
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

  let resize_handle_size = 8.0;
  // Corner handles
  let top_left_handle = create_resize_handle(commands, window_entity, ResizeHandle::TopLeft, resize_handle_size, Vec2::new(0.0, 0.0));
  let top_right_handle = create_resize_handle(commands, window_entity, ResizeHandle::TopRight, resize_handle_size, Vec2::new(size.x - resize_handle_size, 0.0));
  let bottom_left_handle = create_resize_handle(commands, window_entity, ResizeHandle::BottomLeft, resize_handle_size, Vec2::new(0.0, size.y - resize_handle_size));
  let bottom_right_handle = create_resize_handle(commands, window_entity, ResizeHandle::BottomRight, resize_handle_size, Vec2::new(size.x - resize_handle_size, size.y - resize_handle_size));
  
  // Edge handles
  let top_handle = create_edge_resize_handle(commands, window_entity, ResizeHandle::Top, Vec2::new(size.x - 2.0 * resize_handle_size, resize_handle_size), Vec2::new(resize_handle_size, 0.0));
  let bottom_handle = create_edge_resize_handle(commands, window_entity, ResizeHandle::Bottom, Vec2::new(size.x - 2.0 * resize_handle_size, resize_handle_size), Vec2::new(resize_handle_size, size.y - resize_handle_size));
  let left_handle = create_edge_resize_handle(commands, window_entity, ResizeHandle::Left, Vec2::new(resize_handle_size, size.y - 2.0 * resize_handle_size), Vec2::new(0.0, resize_handle_size));
  let right_handle = create_edge_resize_handle(commands, window_entity, ResizeHandle::Right, Vec2::new(resize_handle_size, size.y - 2.0 * resize_handle_size), Vec2::new(size.x - resize_handle_size, resize_handle_size));

  commands.entity(title_container).add_children(&[title_text]);
  commands.entity(buttons_container).add_children(&[collapse_button, minimize_button, maximize_button, close_button]);
  commands.entity(title_bar).add_children(&[title_container, buttons_container]);
  commands.entity(window_entity).add_children(&[
    title_bar,
    content_area,
    top_left_handle,
    top_right_handle,
    bottom_left_handle,
    bottom_right_handle,
    top_handle,
    bottom_handle,
    left_handle,
    right_handle,
  ]);

  window_entity
}

pub fn update_window_content<F>(
  window_manager: &mut ResMut<WindowManager>,
  window_entity: Entity,
  content_builder: F,
) where
  F: FnOnce(&mut RelatedSpawnerCommands<ChildOf>) + Send + Sync + 'static,
{
  window_manager.pending_content_updates.push((window_entity, Box::new(content_builder)));
}

fn update_resize_handle_positions(
  window_query: Query<(Entity, &Window), Changed<Window>>,
  mut handle_query: Query<(&mut Node, &WindowResizeHandle, &ResizeHandleRef)>,
) {
  for (window_entity, window) in window_query.iter() {
    let resize_handle_size = 8.0;
    
    for (mut handle_node, resize_handle, handle_ref) in handle_query.iter_mut() {
      if handle_ref.window_entity == window_entity {
        match resize_handle.handle_type {
          ResizeHandle::TopLeft => {
            handle_node.left = Val::Px(0.0);
            handle_node.top = Val::Px(0.0);
          }
          ResizeHandle::TopRight => {
            handle_node.left = Val::Px(window.size.x - resize_handle_size);
            handle_node.top = Val::Px(0.0);
          }
          ResizeHandle::BottomLeft => {
            handle_node.left = Val::Px(0.0);
            handle_node.top = Val::Px(window.size.y - resize_handle_size);
          }
          ResizeHandle::BottomRight => {
            handle_node.left = Val::Px(window.size.x - resize_handle_size);
            handle_node.top = Val::Px(window.size.y - resize_handle_size);
          }
          ResizeHandle::Top => {
            handle_node.left = Val::Px(resize_handle_size);
            handle_node.top = Val::Px(0.0);
            handle_node.width = Val::Px(window.size.x - 2.0 * resize_handle_size);
          }
          ResizeHandle::Bottom => {
            handle_node.left = Val::Px(resize_handle_size);
            handle_node.top = Val::Px(window.size.y - resize_handle_size);
            handle_node.width = Val::Px(window.size.x - 2.0 * resize_handle_size);
          }
          ResizeHandle::Left => {
            handle_node.left = Val::Px(0.0);
            handle_node.top = Val::Px(resize_handle_size);
            handle_node.height = Val::Px(window.size.y - 2.0 * resize_handle_size);
          }
          ResizeHandle::Right => {
            handle_node.left = Val::Px(window.size.x - resize_handle_size);
            handle_node.top = Val::Px(resize_handle_size);
            handle_node.height = Val::Px(window.size.y - 2.0 * resize_handle_size);
          }
          ResizeHandle::None => {} // Skip for drag handles
        }
      }
    }
  }
}

fn hide_window_children(
    window_entity: Entity,
    children_query: &Query<&Children>,
    content_query: &mut Query<&mut Visibility, (With<WindowContent>, Without<Window>)>,
    resize_handle_query: &mut Query<&mut Visibility, (With<WindowResizeHandle>, Without<Window>, Without<WindowContent>)>,
    show: bool,
) {
    let visibility = if show { Visibility::Inherited } else { Visibility::Hidden };
    
    // Recursively find and hide/show children of this specific window
    if let Ok(children) = children_query.get(window_entity) {
        for child in children.iter() {
            // Check if this child is a content area
            if let Ok(mut content_visibility) = content_query.get_mut(child) {
                *content_visibility = visibility;
            }
            
            // Check if this child is a resize handle
            if let Ok(mut handle_visibility) = resize_handle_query.get_mut(child) {
                *handle_visibility = visibility;
            }
            
            // Recursively check grandchildren
            hide_window_children_recursive(child, children_query, content_query, resize_handle_query, visibility);
        }
    }
}

fn hide_window_children_recursive(
    entity: Entity,
    children_query: &Query<&Children>,
    content_query: &mut Query<&mut Visibility, (With<WindowContent>, Without<Window>)>,
    resize_handle_query: &mut Query<&mut Visibility, (With<WindowResizeHandle>, Without<Window>, Without<WindowContent>)>,
    visibility: Visibility,
) {
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            // Check if this child is a content area
            if let Ok(mut content_visibility) = content_query.get_mut(child) {
                *content_visibility = visibility;
            }
            
            // Check if this child is a resize handle
            if let Ok(mut handle_visibility) = resize_handle_query.get_mut(child) {
                *handle_visibility = visibility;
            }
            
            // Continue recursively
            hide_window_children_recursive(child, children_query, content_query, resize_handle_query, visibility);
        }
    }
}

fn hide_resize_handles(
    window_entity: Entity,
    children_query: &Query<&Children>,
    resize_handle_query: &mut Query<&mut Visibility, (With<WindowResizeHandle>, Without<Window>, Without<WindowContent>)>,
    show: bool,
) {
    let visibility = if show { Visibility::Inherited } else { Visibility::Hidden };
    
    // Find all descendants of the window
    let mut to_check = vec![window_entity];
    let mut checked = std::collections::HashSet::new();
    
    while let Some(entity) = to_check.pop() {
        if checked.contains(&entity) {
            continue;
        }
        checked.insert(entity);
        
        // Check if this entity is a resize handle
        if let Ok(mut handle_visibility) = resize_handle_query.get_mut(entity) {
            *handle_visibility = visibility;
        }
        
        // Add children to check list
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                to_check.push(child);
            }
        }
    }
}

// Handle creation functions < ======================================================= |
// Helper function to create corner resize handles
fn create_resize_handle(
  commands: &mut Commands,
  window_entity: Entity,
  handle_type: ResizeHandle,
  size: f32,
  position: Vec2,
) -> Entity {
  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      left: Val::Px(position.x),
      top: Val::Px(position.y),
      width: Val::Px(size),
      height: Val::Px(size),
      ..default()
    },
    BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.0)), // Invisible by default
    Interaction::default(),
    WindowResizeHandle {
      handle_type,
    },
    ResizeHandleRef {
      window_entity,
    },
  )).id()
}

// Helper function to create edge resize handles
fn create_edge_resize_handle(
  commands: &mut Commands,
  window_entity: Entity,
  handle_type: ResizeHandle,
  size: Vec2,
  position: Vec2,
) -> Entity {
  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      left: Val::Px(position.x),
      top: Val::Px(position.y),
      width: Val::Px(size.x),
      height: Val::Px(size.y),
      ..default()
    },
    BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.0)), // Invisible by default
    Interaction::default(),
    WindowResizeHandle {
      handle_type,
    },
    ResizeHandleRef {
      window_entity,
    },
  )).id()
}

fn calculate_minimize_position(slot: usize, screen_width: f32, screen_height: f32) -> Vec2 {
  let taskbar_width = 200.0;
  let taskbar_height = 30.0;
  let taskbar_margin = 5.0;
  
  // Calculate how many windows fit per row
  let windows_per_row = ((screen_width - taskbar_margin) / (taskbar_width + taskbar_margin)).floor() as usize;
  
  // Calculate row and column
  let row = slot / windows_per_row;
  let col = slot % windows_per_row;
  
  // Calculate position (from bottom-left, going right then up)
  let x = taskbar_margin + col as f32 * (taskbar_width + taskbar_margin);
  let y = screen_height - taskbar_height - taskbar_margin - (row as f32 * (taskbar_height + taskbar_margin));
  
  Vec2::new(x, y)
}

fn calculate_window_minimum_size(
  window_entity: Entity,
  title_bar_query: &Query<&Node, (With<WindowTitleBar>, Without<Window>)>,
  content_query: &Query<&Node, (With<WindowContentArea>, Without<Window>, Without<WindowTitleBar>)>,
  children_query: &Query<&Children>,
  node_query: &Query<&Node, (Without<Window>, Without<WindowTitleBar>, Without<WindowContentArea>)>,
) -> Vec2 {
  let mut min_width: f32 = 150.0; // Absolute minimum
  let mut min_height: f32 = 30.0; // Title bar height
  
  // Calculate title bar minimum width
  if let Ok(children) = children_query.get(window_entity) {
    for child in children.iter() {
      if let Ok(_title_bar_node) = title_bar_query.get(child) {
        // Title bar needs space for title text + buttons
        // Estimate: 4 buttons * 20px + gaps + padding + min title space
        let button_area = 4.0 * 20.0 + 3.0 * 4.0; // buttons + gaps
        let padding = 16.0; // 8px each side
        let min_title_space = 50.0; // Minimum space for title text
        
        min_width = min_width.max(button_area + padding + min_title_space);
        min_height = 30.0; // Fixed title bar height
        break;
      }
    }
    
    // Calculate content area minimum size
    for child in children.iter() {
      if let Ok(_content_node) = content_query.get(child) {
        // Get content minimum size (this is basic - you might want more sophisticated calculation)
        let content_min_width = calculate_content_minimum_width(child, &children_query, &node_query);
        let content_min_height = 50.0; // Minimum content height
        
        min_width = min_width.max(content_min_width);
        min_height += content_min_height;
        break;
      }
    }
  }
  
  Vec2::new(min_width, min_height)
}

// Helper function to calculate content minimum width
fn calculate_content_minimum_width(
  content_entity: Entity,
  children_query: &Query<&Children>,
  node_query: &Query<&Node, (Without<Window>, Without<WindowTitleBar>, Without<WindowContentArea>)>,
) -> f32 {
  let mut min_width = 0.0;
  
  if let Ok(children) = children_query.get(content_entity) {
    for child in children.iter() {
      if let Ok(node) = node_query.get(child) {
        // Calculate minimum width based on node properties
        let node_min_width = match (&node.width, &node.min_width) {
          (Val::Px(w), _) => *w,
          (_, Val::Px(min_w)) => *min_w,
          _ => 0.0,
        };
        
        // For flex direction column, take the max width of children
        // For flex direction row, sum the widths
        match node.flex_direction {
          FlexDirection::Row => min_width += node_min_width,
          FlexDirection::Column => min_width = min_width.max(node_min_width),
          _ => min_width = min_width.max(node_min_width),
        }
        
        // Recursively check children
        let child_min_width = calculate_content_minimum_width(child, children_query, node_query);
        match node.flex_direction {
          FlexDirection::Row => min_width += child_min_width,
          FlexDirection::Column => min_width = min_width.max(child_min_width),
          _ => min_width = min_width.max(child_min_width),
        }
      }
    }
  }
  
  min_width
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
      .init_resource::<WindowManager>()
      .add_systems(Update, (
        handle_window_focus_click,
        handle_resize_interactions,
        handle_window_interactions,
        handle_window_drag,
        handle_window_resize,
        update_resize_handle_positions,
        handle_window_focus,
        handle_window_buttons,
        handle_window_collapse,
        handle_window_maximize,
        handle_window_minimize,
        update_window_z_order,
        handle_window_close,
        process_pending_content_updates,
        enforce_window_minimum_sizes,
      ).chain());
  }
}


