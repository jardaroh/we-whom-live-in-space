use crate::spacetime_bindings::*;

pub fn connect_to_db() -> DbConnection {
  DbConnection::builder()
    .on_connect(|_, _, _| {
      println!("Connected to SpacetimeDB!");
    })
    .on_connect_error(|_, err| {
      eprintln!("Failed to connect to SpacetimeDB: {}", err);
    })
    .on_disconnect(|_, _| {
      println!("Disconnected from SpacetimeDB.");
    })
    .with_module_name("test-reducer")
    .with_uri("http://localhost:3000")
    .build()
    .expect("Failed to create DbConnection")
}
