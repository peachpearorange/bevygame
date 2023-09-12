use {bevy::{ecs::{schedule,
                  system::{EntityCommands, WithEntity},
                  world::EntityMut},
            gltf::Gltf,
            pbr::{ClusterConfig, ClusterFarZMode},
            prelude::*,
            render::RenderPlugin,
            scene::SceneInstance,
            utils::{petgraph::algo::matching, HashMap},
            window::WindowResolution},
     bevy_rapier3d::{control::KinematicCharacterController,
                     parry::query::sat::triangle_segment_find_local_separating_normal_oneway,
                     prelude::*, render::RapierDebugRenderPlugin},
     rand::thread_rng,
     rust_utils::{add_array, aint, change, comment, sub_array},
     std::{fmt::Debug,
           ops::{Add, DerefMut, Sub}}};

mod spell {
  use std::marker::PhantomData;

  use bevy::prelude::Component;

  struct Effect<C: Component, F: FnOnce(C) -> C>(F);
  struct Effects<C: Component, F: FnOnce(C) -> C>(Vec<Effect<C, F>>);
  enum Target {
    Other,
    SelfTarget,
    NoTarget
  }
  struct Spell<C: Component> {
    effects: Effects<C, _>,
    target: Target
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
    vert_ids.map(|id| {
              (id, (pick_adj(&id), Quat::from_array(default().map(|_| rand())).normalize()))
            })
            .collect();
  let insert = |mut m, k, v| {
    m.insert(k, v);
    m
  };
  let good_quats = iterate_n_times(id_to_adj_and_quat,
                                   |q| {
                                     let (id, (adj, quat)) =
                                       q.get_key_value(pick(vert_ids)).unwrap();
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
#[derive(Resource, Default, Deref, DerefMut)]
struct TileMap(HashMap<Coord, Tile>);
impl TileMap {
  fn add_entity(&mut self, c: &Coord, e: Entity) {
    match self.get_mut(c) {
      Some(tile) => {
        tile.contents.insert(e);
      }
      None => {
        println!("tile does not exist");
      }
    }
  }
  fn remove_entity(&mut self, c: &Coord, e: Entity) {
    self.0.get_mut(c).unwrap().contents.remove(&e);
  }
  fn transfer(&mut self, c1: &Coord, c2: &Coord, e: Entity) {
    self.remove_entity(c1, e);
    self.add_entity(c2, e)
  }
  fn is_wall(&self, pos: Coord) -> bool { self.0.get(&pos).unwrap().is_wall }
}
fn prob(p: f32) -> bool { p > rand::random::<f32>() }
fn dist(a: [i32; 2], b: [i32; 2]) -> f32 {
  a.iter()
   .zip(b.iter())
   .map(|(w, e)| ((w - e) as f32).powi(2))
   .sum::<f32>()
   .sqrt()
}

use {bevy::prelude::Entity, bevy_rapier3d::prelude::Collider};

use crate::{components::*,
            gamething::{generate_2d_level, iterate_n_times, iterate_until, pick,
                        pick_multiple, random_movement, try_to_move, TryToMove}};
// 2d physics???

pub fn spawn_meshes(mut c: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<StandardMaterial>>) {
  for i in 0..3 {
    let rand = || rand::random::<f32>();
    for (id, _) in meshes.iter() {
      let h = meshes.get_handle(id);
      c.spawn((RigidBody::Dynamic,
               Collider::from_bevy_mesh(meshes.get(&h).unwrap(), &ComputedColliderShape::ConvexDecomposition(default())).unwrap(),
               Restitution::coefficient(0.7),
               PbrBundle { mesh: h,
                           material: materials.add(Color::rgb(rand(), rand(), rand()).into()),
                           transform: Transform::from_xyz(10.0 + rand(),
                                                          rand() * 8.0 - 4.0,
                                                          rand() * 8.0 - 4.0),
                           ..default() }));
    }
  }
}

pub fn game_plugin(app: &mut App) {
  app.init_resource::<TileMap>()
     .add_event::<TryToMove>()
     .add_startup_system(generate_2d_level)
     .fn_plugin(random_movement)
     .fn_plugin(try_to_move);
}
