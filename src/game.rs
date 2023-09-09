use {crate::foxtrot::{bundles::*,
             components::{self, Char, Container, EnemyMovement, Player, RandomMovement},
             input,
             tiles::{self, Tile, *}},
     bevy::{ecs::{schedule,
                  system::{EntityCommands, WithEntity},
                  world::EntityMut},
            gltf::Gltf,
            pbr::{ClusterConfig, ClusterFarZMode},
            prelude::*,
            render::RenderPlugin,
            scene::SceneInstance,
            utils::{petgraph::algo::matching, HashMap},
            window::WindowResolution},
     bevy_ecs_tilemap::{tiles::TileBundle, TilemapBundle},
     bevy_egui::{self, egui, EguiContexts, EguiPlugin},
     bevy_fn_plugin::bevy_plugin,
     bevy_fps_controller::controller::*,
     bevy_inspector_egui::egui::style::Visuals,
     bevy_rapier2d::prelude::*,
     bevy_rapier3d::{control::KinematicCharacterController,
                     parry::query::sat::triangle_segment_find_local_separating_normal_oneway,
                     prelude::{NoUserData, RapierPhysicsPlugin},
                     render::RapierDebugRenderPlugin},
     bevy_xpbd_3d::prelude::*,
     cascade::cascade,
     itertools::{iterate, Itertools},
     ndarray::{Array3, ArrayBase},
     rand::thread_rng,
     rust_utils::{add_array, aint, change, coll_max, comment, sub_array},
     std::{fmt::Debug,
           ops::{Add, DerefMut, Sub}}};

mod spell {
  struct Effect<C: Component>(Box<dyn Fn(C) -> C>);
  struct Effects(Vec<Effect>);
  enum Target {
    Other,
    Self,
    NoTarget
  }
  struct Spell {
    effects: Effects,
    target: ActionTarget
  }
}

// pub fn ui(mut c: EguiContexts) {
//   let gray = |n: u8| egui::Color32::from_gray(n);
//   let v = Visuals { //   dark_mode: todo!(),
//                     // override_text_color: todo!(),
//                     // widgets: todo!(),
//                     // selection: todo!(),
//                     // hyperlink_color: todo!(),
//                     // faint_bg_color: todo!(),
//                     // extreme_bg_color: todo!(),
//                     // code_bg_color: todo!(),
//                     // warn_fg_color: todo!(),
//                     // error_fg_color: todo!(),
//                     // window_rounding: todo!(),
//                     // window_shadow: todo!(),
//                     window_fill: gray(90),
//                     window_stroke: egui::Stroke::new(1.0, gray(20)),
//                     // menu_rounding: todo!(),
//                     // panel_fill: todo!(),
//                     // popup_shadow: todo!(),
//                     // resize_corner_size: todo!(),
//                     // text_cursor_width: todo!(),
//                     // text_cursor_preview: todo!(),
//                     // clip_rect_margin: todo!(),
//                     // button_frame: todo!(),
//                     // collapsing_header_frame: todo!(),
//                     // indent_has_left_vline: todo!(),
//                     // striped: todo!(),
//                     // slider_trailing_fill: todo!(),
//                     ..default() };
//   // text color???
//   c.ctx_mut().set_visuals(v);
//   // egui::Ui
//   egui::Window::new("Hello").show(c.ctx_mut(), |ui| {
//                               ui.label("world");
//                             });
// }
// NUMBER OF DIMENSIONS???
const DIMS: usize = 2;

#[derive(Component, Hash, PartialEq, Eq, Copy, Clone, Deref, DerefMut, Default)]
pub struct Pos([i32; DIMS]);
impl Pos {
  fn map(self, f: impl Fn(i32) -> i32) -> Self { rust_utils::change(self, |inner| inner.map(f)) }
}
impl Add<Dir> for Pos {
  type Output = Self;
  fn add(self, rhs: Dir) -> Self::Output { change(self, |inner| add_array(inner, rhs)) }
}
impl Sub<Pos> for Pos {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self { change(self, |inner| rust_utils::sub_array(inner, *rhs)) }
}
impl From<(i32, i32)> for Pos {
  fn from((a, b): (i32, i32)) -> Self { Pos([a, b]) }
}
const CHUNK_SIDE_LENGTH: usize = 16;
pub struct Chunk(Array2<crate::foxtrot::tiles::Tile>);
impl Default for Chunk {
  fn default() -> Self { Self(Array2::from_elem([CHUNK_SIDE_LENGTH; DIMS], default())) }
}
#[derive(Default, Resource)]
pub struct ChunkMap(HashMap<[i32; DIMS], Chunk>);
pub fn generate_level() -> ChunkMap {
  ChunkMap(HashMap::from([([0, 0],
                           Chunk(cascade! {Chunk::default().0;
                           ..iter_mut().for_each(|t| rust_utils::set(t,tiles::rock()) );}))]))
}

// fn update<C: Component>(c: &mut C, f: impl FnOnce(C) -> C) { take(c, f); }
// fn set<C: Component>(c: &mut C, v: C) { *c = v; }

// systems
fn player_movement() {}
use {itertools::{self, iproduct},
     ndarray::Array2};

fn adjacents(pos: &Pos) -> Vec<Pos> {
  let Pos([x, y]) = pos;
  let around = |v| v - 1..=v + 1;
  iproduct!(around(x), around(y)).map(Pos::from)
                                 .filter(aint(pos))
                                 .collect()
}
fn pick<T>(coll: impl IntoIterator<Item = T>) -> T {
  rand::seq::IteratorRandom::choose(coll.into_iter(), &mut thread_rng()).unwrap()
}
fn pick_multiple<T>(coll: impl IntoIterator<Item = T>, n: usize) -> Vec<T> {
  rand::seq::IteratorRandom::choose_multiple(coll.into_iter(), &mut thread_rng(), n)
}
fn iterate_until<T>(mut val: T, f: impl Fn(T) -> T, conditionf: impl Fn(&T) -> bool) -> T {
  while !conditionf(&val) {
    val = f(val);
  }
  val
}
fn iterate_n_times<T>(val: T, f: impl Fn(T) -> T, n: u32) -> T {
  let (val, _) = iterate_until((val, 0), |(val, k)| (f(val), k + 1), |(_, k)| *k == n);
  val
}
fn generate_low_poly_3d_planet() -> Mesh {
  type Id = i32;
  let vert_ids = 0..30;
  let pick_adj = |id| {
    iterate_until(pick_multiple(vert_ids, 3),
                  |_| pick_multiple(vert_ids, 3),
                  |&adj| !adj.contains(id))
  };
  let rand = || rand::random::<f32>();
  let id_to_adj_and_quat: HashMap<Id, (Vec<Id>, Quat)> =
    vert_ids.map(|id| (id, (pick_adj(&id), Quat::from_array(default().map(|_| rand())).normalize())))
            .collect();
  let insert = |mut m, k, v| {
    m.insert(k, v);
    m
  };
  let good_quats = iterate_n_times(id_to_adj_and_quat,
                                   |q| {
                                     let (id, (adj, quat)) = q.get_key_value(pick(vert_ids)).unwrap();
                                     let adj_quats = adj.iter().map(|id| q.get(id).unwrap());
                                     let q1 = q1.normalize();
                                     let q2 = q2.normalize();
                                     let q3 = q3.normalize();

                                     let q12 = q1.slerp(q2, 0.5);
                                     let q23 = q2.slerp(q3, 0.5);

                                     insert(q, id, q12.slerp(q23, 0.5))
                                   },
                                   100);
  // random points on a sphere, turned into vertices and triangles...
  // Mesh::ATTRIBUTE_NORMAL
  Mesh::from(good_quats)
}

impl HelperDataStructure {
  fn new() -> Self { Self::default() }
  fn build(things: impl IntoIterator<Item = Thing>) -> Self {
    things.into_iter().fold(Self::new(), Self::add_thing)
  }
  fn compute_property_for_place(self, p: Place) -> Property {
    self.lookup_rounded(place).compute_property()
  }
}
// #[derive(Component, Default)]
// struct TileMap;
// // use parenting???
// fn spawn_tilemap(commands: Commands,
//                  material: Handle<ColorMaterial>,
//                  tile_size: f32,
//                  map_width: u32,
//                  map_height: u32) {
//   // Create the tilemap entity
//   let fs = [|n| n + 1, |n| n - 1];
//   let tilemap = commands.spawn(tiles::grass()).id();
//   // Create the tile entities as children
//   iproduct!(0..map_width, 0..map_height).for_each(|(x, y)| {
//                                           commands.spawn(TileBundle { ..default() })
//                                                   .insert_children()
//                                                   .with_children(|thing| {
//                                                     thing.spawn((Tile { is_wall: todo!(),
//                                                                         bg_color: todo!() },
//                                                                  Name,
//                                                                  Char,
//                                                                  Container))
//                                                   });
//                                         });
// }
use {list_comprehension::comp, macro_utils::if_match};
#[derive(Resource, Default, Deref, DerefMut)]
struct TileMap(HashMap<Pos, Tile>);
impl TileMap {
  fn add_entity(&mut self, c: &Pos, e: Entity) {
    match self.get_mut(c) {
      Some(tile) => {
        tile.contents.insert(e);
      }
      None => {
        println!("tile does not exist");
      }
    }
  }
  fn remove_entity(&mut self, c: &Pos, e: Entity) { self.0.get_mut(c).unwrap().contents.remove(&e); }
  fn transfer(&mut self, c1: &Pos, c2: &Pos, e: Entity) {
    self.remove_entity(c1, e);
    self.add_entity(c2, e)
  }
  fn is_wall(&self, pos: Pos) -> bool { self.0.get(&pos).unwrap().is_wall }
}
fn prob(p: f32) -> bool { p > rand::random::<f32>() }
fn dist(a: [i32; 2], b: [i32; 2]) -> f32 {
  a.iter()
   .zip(b.iter())
   .map(|(w, e)| ((w - e) as f32).powi(2))
   .sum::<f32>()
   .sqrt()
}
const LEVEL_RADIUS: i32 = 100;
const VIEW_RADIUS: i32 = 12;

// impl From<(i32, i32)> for RelPos {
//   fn from((x, y): (i32, i32)) -> Self { Self([x, y]) }
// }
struct TryToMove(Entity, Dir);
fn random_movement(es: Query<Entity, With<RandomMovement>>, mut ev: EventWriter<TryToMove>) {
  es.for_each(|e| ev.send(TryToMove(e, pick([[1, 0], [-1, 0], [0, 1], [0, -1]]))));
}
fn clamp<T: Ord>(min: T, x: T, max: T) -> T { x.clamp(min, max) }
use {bevy::prelude::Entity,
     rust_utils::{set, Coll, CollConditions}};
// 2d physics???

pub fn game_plugin(app: &mut App) {
  app.init_resource::<TileMap>()
     .add_event::<TryToMove>()
     .add_startup_system(generate_2d_level)
     .add_system(random_movement)
     .add_system(try_to_move);
}
