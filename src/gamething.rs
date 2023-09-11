#![allow(unused_imports)]
#![allow(dead_code)]

use {bevy::{ecs::{query::WorldQuery,
                  system::{SystemParam, SystemState}},
            prelude::*,
            window::ThreadLockedRawWindowHandleWrapper},
     rand::{thread_rng, Rng},
     rust_utils::{change, eager, first, fold, if_match, inc, iproduct, HashMapTrait, Tap,
                  TypeKeyMap},
     std::{collections::{BTreeMap, HashMap, HashSet},
           hash::Hash,
           marker::PhantomData,
           ops::{Deref, DerefMut}}};

use std::cmp::Ordering;

use bevy::prelude::{Event, EventReader, EventWriter};

use crate::components::*;

fn coolmesh() -> Mesh {
  // Vertex
  let color = Color::from_rgba(155, 155, 155, 155);
  let uv = Vec2::from([0.6, 0.7]);
  let rng = || rand::gen_range(-5.0, 5.0);
  let rand_pos = || Vec3::from([rng(), rng(), rng()]);
  let rand_vertex = || Vertex { position: rand_pos(),
                                uv,
                                color };
  Mesh { vertices: vec![rand_vertex(),
                        rand_vertex(),
                        rand_vertex(),
                        rand_vertex(),
                        rand_vertex()],
         indices: vec![0, 1, 2, 0, 1, 3],
         texture: None }
}
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

#[derive(Event)]
struct ContainerTransfer {
  source: Entity,
  dest: Entity,
  item: Entity
}
fn container_transfer(r: EventReader<ContainerTransfer>, mut q: Query<&mut Container>) {
  for ContainerTransfer { source, dest, item } in r.iter() {
    if let (Ok(source_cont), Ok(dest_cont)) = (q.get_mut(source), q.get_mut(dest)) {
      source_cont.0.remove(item);
      dest_cont.0.insert(item);
    }
  }
}

// impl Default for State {
//   fn default() -> Self {
//     Self { mouse_over_relative_coord: None,
//            scroll_pos: 0,
//            entity_count: 0,
//            cev: TypeKeyMap::<Entity>::default(),
//            selected_entity: None,
//            message_log: vec![],
//            pressed_keys: HashSet::with_capacity(4),
//            new_pressed_keys: Keys::default(),
//            newest_pressed_y: None,
//            newest_pressed_x: None,
//            current_view: CurrentView::WorldView }
//   }
// }
const DIMS: usize = 2;

// systems
// use {ndarray::Array2, rapl::Ndarr};

pub fn adjacents(pos: &Coord) -> Vec<Coord> {
  let Coord([x, y]) = pos;
  let around = |v| v - 1..=v + 1;
  iproduct!(around(x), around(y)).map(Coord::from)
                                 .filter(aint(pos))
                                 .collect()
}
pub fn pick<T>(coll: impl IntoIterator<Item = T>) -> T {
  rand::seq::IteratorRandom::choose(coll.into_iter(), &mut thread_rng()).unwrap()
}
pub fn pick_multiple<T>(coll: impl IntoIterator<Item = T>, n: usize) -> Vec<T> {
  rand::seq::IteratorRandom::choose_multiple(coll.into_iter(), &mut thread_rng(), n)
}
pub fn iterate_until<T>(mut val: T,
                        f: impl Fn(T) -> T,
                        conditionf: impl Fn(&T) -> bool)
                        -> T {
  while !conditionf(&val) {
    val = f(val);
  }
  val
}
pub fn iterate_n_times<T>(val: T, f: impl Fn(T) -> T, n: u32) -> T {
  let (val, _) = iterate_until((val, 0), |(val, k)| (f(val), k + 1), |(_, k)| *k == n);
  val
}
use rust_utils::comment;
pub fn prob(p: f32) -> bool { p > rand::random::<f32>() }
pub fn dist(a: [i32; 2], b: [i32; 2]) -> f32 {
  a.iter()
   .zip(b.iter())
   .map(|(w, e)| ((w - e) as f32).powi(2))
   .sum::<f32>()
   .sqrt()
}
const LEVEL_RADIUS: i32 = 100;
const VIEW_RADIUS: i32 = 12;

pub fn random_movement(q: Query<Entity, With<RandomMovement>>,
                       mut ew: EventWriter<TryToMove>) {
  ew.send_batch(q.iter().map(|e| TryToMove(e, Dir::random_lateral())));
}
pub fn clamp<T: Ord>(min: T, x: T, max: T) -> T { x.clamp(min, max) }

#[derive(Event)]
pub struct MoveTowardsCoord(Entity, Coord);
fn move_towards_coord(mut er: EventReader<MoveTowardsCoord>,
                      mut ew: EventWriter<TryToMove>,
                      q: Query<&Coord>) {
  ew.send_batch(er.iter().filter_map(|MoveTowardsCoord(e, dest_pos)| {
                           q.get(e)
                            .ok()
                            .map(|pos| TryToMove(e, Dir::from_to(pos, &dest_pos)))
                         }));
}
fn enemy_movement(mut ewttm: EventWriter<TryToMove>,
                  mut ewmtc: EventWriter<MoveTowardsCoord>,
                  affected: Query<Entity, With<EnemyMovement>>,
                  player_pos_q: Query<&Coord, With<Player>>) {
  let player_pos = player_pos_q.get_single().unwrap();
  for e in affected {
    if prob(0.5) {
      ewmtc.send(MoveTowardsCoord(e, player_pos));
    } else {
      ewttm.send(TryToMove(e, Dir::random_lateral()));
    }
  }
}
fn player_movement() {}
pub enum Dir {
  NORTH,
  NORTHEAST,
  EAST,
  SOUTHEAST,
  SOUTH,
  SOUTHWEST,
  WEST,
  NORTHWEST,
  HERE
}
const NORTH: Dir = Dir::NORTH;
const NORTHEAST: Dir = Dir::NORTHEAST;
const EAST: Dir = Dir::EAST;
const SOUTHEAST: Dir = Dir::SOUTHEAST;
const SOUTH: Dir = Dir::SOUTH;
const SOUTHWEST: Dir = Dir::SOUTHWEST;
const WEST: Dir = Dir::WEST;
const NORTHWEST: Dir = Dir::NORTHWEST;
const HERE: Dir = Dir::HERE;
impl Dir {
  fn random_lateral() -> Dir { pick([Dir::EAST, Dir::NORTH, Dir::WEST, Dir::SOUTH]) }
  fn random() -> Dir {
    pick([NORTH, NORTHEAST, EAST, SOUTHEAST, SOUTH, SOUTHWEST, WEST, NORTHWEST, HERE])
  }
  fn from_to(orig: &Coord, dest: &Coord) -> Self {
    let Coord([ox, oy]) = orig;
    let Coord([dx, dy]) = dest;
    type O = Ordering;
    match (dx.cmp(ox), dy.cmp(ox)) {
      (O::Less, O::Less) => Dir::SOUTHWEST,
      (O::Less, O::Equal) => Dir::SOUTH,
      (O::Less, O::Greater) => Dir::NORTHEAST,
      (O::Equal, O::Less) => Dir::SOUTH,
      (O::Equal, O::Equal) => Dir::HERE,
      (O::Equal, O::Greater) => Dir::NORTH,
      (O::Greater, O::Less) => Dir::SOUTHEAST,
      (O::Greater, O::Equal) => Dir::EAST,
      (O::Greater, O::Greater) => Dir::NORTHEAST
    }
  }
}
impl From<Dir> for Coord {
  fn from(value: Dir) -> Self {
    Coord(match value {
            Dir::NORTH => [0, 1],
            Dir::NORTHEAST => [1, 1],
            Dir::EAST => [1, 0],
            Dir::SOUTHEAST => [1, -1],
            Dir::SOUTH => [0, -1],
            Dir::SOUTHWEST => [-1, -1],
            Dir::WEST => [-1, 0],
            Dir::NORTHWEST => [-1, 1],
            Dir::HERE => [0, 0]
          })
  }
}
#[derive(Event)]
pub struct TryToMove(Entity, Dir);
enum EntityBundle {
  Enemy((Char, Name, EnemyMovement, AttackPlayer, Combat)),
  Dragon((Char, Name, EnemyMovement, DragonAttack, AttackPlayer, Combat)),
  Fire((Char, Name, Fire)),
  Animal((Char, Name, RandomMovement))
}
type EB = EntityBundle;
const ENEMY: EB = EB::Enemy((Char('👿'),
                             name("enemy"),
                             EnemyMovement,
                             AttackPlayer,
                             Combat { hp: 30, damage: 1 }));
const SPIDER: EB = EB::Enemy((Char('🕷'),
                              name("spider"),
                              EnemyMovement,
                              AttackPlayer,
                              Combat { hp: 40, damage: 1 }));
const DRAGON: EB = EB::Dragon((Char('🐉'),
                               name("dragon"),
                               EnemyMovement,
                               DragonAttack,
                               AttackPlayer,
                               Combat { hp: 60, damage: 1 }));
const FIRE: EB = EB::Fire((Char('🔥'), name("fire"), Fire { dir: (1, 0) }));
const SNOWMAN: EB = EB::Animal((Char('⛄'), name("snowman"), RandomMovement));
const SHEEP: EB = EB::Animal((Char('🐑'), name("sheep"), RandomMovement));
const DUCK: EB = EB::Animal((Char('🦆'), name("duck"), RandomMovement));
const RABBIT: EB = EB::Animal((Char('🐇'), name("rabbit"), RandomMovement));

enum TileBundle {
  WithChar((Tile, Name, Char)),
  WithoutChar((Tile, Name))
}
type TB = TileBundle;
fn tb(walkable: bool,
      color: &'static str,
      name: &'static str,
      char: Option<char>)
      -> TileBundle {
  let color = color.to_string();
  let name = name(name);
  let tile: Tile = Tile { walkable, color };
  match char {
    Some(c) => TB::WithChar((tile, name, Char(c))),
    None => TB::WithoutChar((tile, name))
  }
}
const WALL: TB = tb(false, "#666666", "wall", Some('#'));
const TREE: TB = tb(false, "#27AD00", "tree", Some('🌲'));
const ROCK: TB = tb(false, "#71A269", "rock", Some('🪨'));
const WATER: TB = tb(false, "#5961FF", "water", None);
const SAND: TB = tb(true, "#D9DC60", "sand", None);
const GRASS: TB = tb(true, "#22B800", "grass", None);

struct TileAndEntitySpawn(Coord, TileBundle, Option<EntityBundle>);

struct TileMap(pub HashMap<Coord, Entity>);

pub fn generate_2d_level(mut c: Commands, tm: ResMut<TileMap>) {
  let range = -LEVEL_RADIUS..=LEVEL_RADIUS;
  let coords = iproduct!(range, range).map(Coord::from);
  let things_to_spawn: impl Iterator<Item = TileAndEntitySpawn> =
    coords.map(|pos| {
            let (t, e) = if_match! {
            pos == Coord::default() => (GRASS, Some(PLAYER)),
            dist(*pos, [0, 0]) >= (LEVEL_RADIUS - VIEW_RADIUS) as f32 => (WATER, None),
            dist(*pos, [0, 0]) >= (LEVEL_RADIUS - VIEW_RADIUS - 3) as f32 => (SAND, None),
            prob(0.03) => (TREE,None),
            prob(0.01) => (ROCK,None),
            else => (GRASS, if_match!{
              prob(0.001) => Some(SHEEP),
              prob(0.001) => Some(RABBIT),
              prob(0.001) => Some(DUCK),
              prob(0.001) => Some(ENEMY),
              prob(0.0001) => Some(DRAGON),
              else => None })};
            TileAndEntitySpawn(pos, t, e)
          });
  for TileAndEntitySpawn(pos, tb, oeb) in things_to_spawn {
    let mut tecom = match tb {
      TB::WithChar(b) => c.spawn(b),
      TB::WithoutChar(b) => c.spawn(b)
    };
    let mut cont = Container::empty();
    tm.0.insert(pos, tecom.id());
    if let Some(eb) = oeb {
      let e = match eb {
                EB::Enemy(b) => c.spawn(b),
                EB::Dragon(b) => c.spawn(b),
                EB::Fire(b) => c.spawn(b),
                EB::Animal(b) => c.spawn(b)
              }.insert(pos)
               .id();
      cont.0.insert(e);
    }
    tecom.insert(cont);
  }
}

comment! {
  #[derive(Hash, Eq, PartialEq, Copy, Clone)]
  enum CurrentView {
    WorldView,
    EntityView,
    InventoryView,
    CraftingView
  }
  fn name(s: &'static str) -> Name { Name(s.to_string()) }

  #[derive(Hash, Eq, PartialEq)]
  enum Key {
    Left,
    Right,
    Up,
    Down
  }
  #[derive(Default)]
  struct Keys {
    left: bool,
    right: bool,
    up: bool,
    down: bool
  }
  struct State {
    // bevy::wrapper::bevy_utils::BevyWrapper,
    // world: hecs::World,
    tiles: HashMap<Coord, Entity>,
    mouse_over_relative_coord: Option<Coord>,
    scroll_pos: u8,
    // entity_count: u32,
    // cev: TypeKeyMap<Entity>,
    selected_entity: Option<Entity>,
    message_log: Vec<String>,
    pressed_keys: HashSet<Key>,
    new_pressed_keys: Keys,
    newest_pressed_y: Option<Key>,
    newest_pressed_x: Option<Key>,
    current_view: CurrentView
  }
  fn scroll(self, d: i8) -> Self {
    let Self { scroll_pos,
               mouse_over_relative_coord,
               .. } = self;
    let number_of_entities_on_relative_coord =
      |rel| self.get_entities_on_relative_coord(mouse_over_relative_coord.unwrap_or([0, 0]));
    let l = number_of_entities_on_relative_coord(mouse_over_relative_coord.unwrap_or([0, 0]));
    Self { scroll_pos: (scroll_pos.clamp(0, l) + d).clamp(0, l),
           ..self }
  }

  fn tick(self) -> Self {
    let Self { cev, current_view, .. } = self;
    // let player = self.get_player_id().unwrap();

    if current_view == CurrentView::WorldView
      && first(self.0.query::<With<&Combat, &Player>>().into_iter()).unwrap()
                                                                    .1
                                                                    .hp
      > 0
    {
      Self { cev: cev.random_movement(),
             ..self.player_movement()
             .fire_move()
             .fire_damage()
             .dragon_attack()
             .enemy_movement()
             .combat() }
    } else {
      self
    }
  }

}

pub fn try_to_move(mut self, pos: &mut Coord, dir: Dir, e: Entity) {
  let walkable = |c: Coord| self.get_tile::<Tile>(c).unwrap().walkable;
  // let &pos: &Coord = self.get(e);
  let m = pos + dir;
  let dest = if dir.contains(&0) {
    if self.get_tile::<&Tile>(*pos).unwrap().walkable {
      m
    } else {
      pos
    }
  } else {
    let [l, r] = match dir {
                   NORTHEAST => [NORTH, EAST],
                   NORTHWEST => [NORTH, WEST],
                   SOUTHEAST => [SOUTH, EAST],
                   SOUTHWEST => [SOUTH, WEST],
                   _ => panic!()
                 }.map(|v| *pos + v);
    let [lw, mw, rw] = [l, m, r].map(walkable);
    if !lw || !rw {
      pos
    } else if !mw {
      pick([(l, !lw), (r, !rw)].iter().filter(|(_, w)| !w)).0
    } else {
      m
    }
  };
  // Aaaaaaaa
  container_transfer_entity(self.tiles.get_mut(pos), self.tiles.get_mut(dest), e);
  rust_utils::set(pos, *dest);
  // self.container_transfer_entity(self.get_tile(pos), self.get_tile(dest), e)
  //     .set::<Coord>(e, dest)
  // tm.transfer(&pos, &dest, e);

  // *es.get_component_mut::<Pos>(e).unwrap().into_inner() = dest;
}
comment! {}
