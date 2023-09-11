use bevy::prelude::App;

fn reverse_string(s: String) -> String { s.as_bytes().into_iter().rev().collect() }

pub fn tests_plugin(app: &mut App) {
  println!("{}", reverse_string("asdfg".into()));
  app
}
