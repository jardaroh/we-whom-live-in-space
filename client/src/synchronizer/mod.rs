use bevy::prelude::*;
use bevy_spacetimedb::{
  register_reducers, tables, StdbConnectedEvent, StdbConnection, StdbConnectionErrorEvent, StdbDisconnectedEvent, StdbPlugin
};
// use spacetimedb_sdk::{
//   Table,
// };

mod entity;

use crate::spacetime_bindings::*;

use entity::sync_entities_system;

pub fn synchronizer_plugin(app: &mut App) {
  app.add_plugins(
    StdbPlugin::default()
      .with_connection(|send_connected, send_disconnected, send_connect_error, _| {{
        let conn = DbConnection::builder()
          .with_module_name("test-reducer")
          .with_uri("http://localhost:3000")
          .on_connect_error(move |_ctx, err| {
            send_connect_error.send(StdbConnectionErrorEvent { err }).unwrap();
          })
          .on_disconnect(move |_ctx, err| {
            send_disconnected.send(StdbDisconnectedEvent { err }).unwrap();
          })
          .on_connect(move |_ctx, _id, _c| {
            send_connected.send(StdbConnectedEvent {}).unwrap();
          })
          .build()
          .expect("Failed to create DbConnection");
        
        conn.run_threaded();
        conn
      }})
      .with_events(|plugin, app, db, _reducers| {{
        tables!(
          entity,
          node,
        );

        register_reducers!();
      }}),
  );
  app.add_systems(
    Update,
    (
      on_connected,
      sync_entities_system,
    ),
  );
}


fn on_connected(
  mut events: EventReader<StdbConnectedEvent>,
  stdb: Res<StdbConnection<DbConnection>>,
) {
  for _ in events.read() {
    info!("Connected to SpacetimeDB!");
  }
}
