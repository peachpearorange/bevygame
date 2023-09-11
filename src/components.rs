pub use bevy::prelude::Name;
use bevy::{ecs::system::{SystemParam, SystemState},
           prelude::{Color, Component, Entity, Input, KeyCode, Query, Res, Transform, World},
           utils::{HashMap, HashSet}};

use crate::gamething::Dir;
#[derive(Component)]
pub struct Crafter;
#[derive(Component)]
pub struct Conveyor;
enum KeyFunction {}
pub struct KeyMap(HashMap<KeyCode, KeyFunction>);
pub struct Action {}
pub enum InputState {
  None,
  Number(i32),
  Loop,
  Search,
  Action(Action)
}
#[derive(Component)]
pub struct Char(pub char);
#[derive(Component)]
pub struct AttackPlayer;
#[derive(Component)]
pub struct DragonAttack;
#[derive(Component)]
pub struct RandomMovement;
#[derive(Component)]
pub struct EnemyMovement;
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub enum Interact {
  GiveItem(Entity),
  AddMessage(String)
}
#[derive(Component)]
pub struct Combat {
  pub hp: u32,
  pub damage: u32
}

#[derive(Component, Default)]
pub struct Container(pub HashSet<Entity>);
impl Container {
  pub fn empty() -> Container { Container::default() }
}
#[derive(PartialEq, Eq)]
enum Item {
  Loot,
  Wood,
  Fish
}
#[derive(Component)]
pub struct ItemStack(Item, u32);

#[derive(Component, Hash, Eq, PartialEq, Default, Copy, Clone)]
pub struct Coord(pub [i32; 2]);
const ORIGIN: Coord = Coord([0, 0]);
impl std::ops::Add<Dir> for Coord {
  type Output = Self;
  fn add(self, [a, b]: Dir) -> Self { change(self, |[x, y]| [a + x, b + y]) }
}
impl std::ops::Sub<Dir> for Coord {
  type Output = Self;
  fn sub(self, [a, b]: Dir) -> Self { change(self, |[x, y]| [a - x, b - y]) }
}
impl From<(i32, i32)> for Coord {
  fn from((a, b): (i32, i32)) -> Self { Coord([a, b]) }
}

pub fn name(s: &'static str) -> Name { Name::new(s) }
// #[derive(Component, Default)]
// pub struct Name(String);
#[derive(Component, Default)]
pub struct Tile {
  walkable: bool,
  color: Color
}
#[derive(Component)]
pub struct Fire {
  pub dir: (i8, i8)
}
