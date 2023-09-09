use bevy::{ecs::query::ReadOnlyWorldQuery,
           prelude::{default, Bundle, Entity, Name},
           utils::HashSet};

use crate::foxtrot::components::{self, Char, Container};

#[derive(Clone, Default)]
pub struct Tile {
  pub is_wall: bool,
  pub bg_color: &'static str,
  pub name: &'static str,
  pub char: Option<char>,
  pub contents: HashSet<Entity>
}
pub type TileBundle = (components::Tile, Name, Char, Container);
// fn tile(is_wall: bool, bg_color: &'static str, name: &'static str, char: Option<char>) -> TileBundle {
//   (components::Tile { is_wall, bg_color }, Name::name, Char::char, Container::default())
// }
// fn tile(is_wall: bool,
//         bg_color: &'static str,
//         name: &'static str,
//         char: Option<char>)
//         -> Tile {
//   Tile { is_wall,
//          bg_color,
//          name,
//          char,
//          contents: default() }
// }
// pub fn wall() -> Tile { tile(true, "#666666", "wall", Some('#')) }
// pub fn tree() -> Tile { tile(true, "#27AD00", "tree", Some('🌲')) }
// pub fn rock() -> Tile { tile(true, "#71A269", "rock", Some('🪨')) }
// pub fn water() -> Tile { tile(false, "#5961FF", "water", None) }
// pub fn sand() -> Tile { tile(false, "#D9DC60", "sand", None) }
// pub fn grass() -> Tile { tile(false, "#22B800", "grass", None) }
// pub fn ladder() -> Tile { tile(false, "#4A4A4A", "ladder", Some('🪜')) }
// impl Default for Tile {
//   fn default() -> Self { grass() }
// }
