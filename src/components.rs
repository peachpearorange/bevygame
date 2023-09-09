use bevy::{ecs::system::{SystemParam, SystemState},
           prelude::{Component, Entity, Input, KeyCode, Name, Query, Res, Transform, World},
           utils::HashMap};
#[derive(Component)]
pub struct Crafter;
#[derive(Component)]
pub struct Conveyor;
enum KeyFunction {}
struct KeyMap(HashMap<KeyCode, KeyFunction>);
struct Action {}
enum InputState {
  None,
  Number(i32),
  Loop,
  Search,
  Action(Action)
}
// #[derive(Hash, Eq, PartialEq)]
// enum EntityId {
//   DynamicEntity(u32),
//   ItemEntity(ItemID),
//   Tile(i32, i32),
// }
// components
// #[derive(Component)]
// pub struct Name<'t>(pub &'t str);
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
pub struct Tile {
  pub is_wall: bool,
  pub bg_color: &'static str
}
#[derive(Component)]
pub struct Fire {
  pub dir: (i8, i8)
}
#[derive(Component)]
pub struct Combat {
  pub hp: u32,
  pub damage: u32
}
#[derive(Default, Component)]
pub struct Container(pub HashMap<Entity, u32>);
// pub const CONTAINEREMPTY: Container = Container::default();
impl Container {
  pub fn empty() -> Container { Container::default() }
}
