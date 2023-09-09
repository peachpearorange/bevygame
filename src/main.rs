#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_braces)]
#![feature(const_trait_impl)]
#![feature(type_alias_impl_trait)]
#![allow(unused_mut)]

use bevy::asset::LoadState;
use seldom_fn_plugin::FnPluginExt;
// use bevy_utils::{BevyWrapper, Events, EventWriter};
// use bevy_xpbd_3d::plugins::PhysicsPlugins;

use crate::{game::game_plugin, loading::LoadingPlugin};
use {bevy::gltf::Gltf, bevy_asset_loader::prelude::*};
use {bevy::window, bevy_panorbit_camera::PanOrbitCamera};

pub mod foxtrot;
pub mod gamething;
// pub mod tests;

// pub mod bundles;
pub mod components;
pub mod game;
// pub mod gol;
// pub mod game2d;
pub mod input;
// pub mod lunarlander3d;
// pub mod tiles;
// // mod audio;
// // mod menu;
pub mod loading;
// pub mod text2d;

// use bevy_game::GamePlugin; // ToDo: Replace bevy_game with your new crate name.
use {bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows, DefaultPlugins},
     bevy_fn_plugin::bevy_plugin,
     bevy_fps_controller, bevy_panorbit_camera,
     bevy_rapier3d::prelude::*,
     // bundles::player,
     rust_utils::comment,
     std::io::Cursor,
     winit::window::Icon};
// todo: figure out how to load gltf files
#[bevy_main]
pub fn main() {
  println!("{}", reverse_string("asdfg".into()));
  App::new()
          .add_plugins(DefaultPlugins.set(AssetPlugin { watch_for_changes: true,
                                                        ..default() })
                       .set(WindowPlugin { primary_window:
                                           Some(window::Window { resolution:
                                                                 window::WindowResolution::new(1024., 768.),
                                                                 title:
                                                                 "bevy_game".to_string(),
                                                                 ..default() }),
                                           ..default() }))
          .add_system(bevy::window::close_on_esc)
          // .insert_resource(ClearColor(Color::SALMON))
          // .insert_resource(game::generate_level())
          // .add_startup_system(game::init)
          // .add_system(rotate_camera)
          .fn_plugin(game_plugin)
          .fn_plugin(input::keylogger)
          // .add_plugin(input::KeyLogger)
          .add_plugins(PhysicsPlugins::default())
          // .add_plugin(bevy_fps_controller::controller::FpsControllerPlugin)
          .add_plugin(bevy_panorbit_camera::PanOrbitCameraPlugin)
          // .add_plugin(lunarlander3d::LunarLander)
          // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
          .add_startup_system(spawn_stuff)
          .add_plugin(loading_plugin)
          .add_plugin(game_plugin)
          .add_plugin(bevy_egui::EguiPlugin)
          .add_system(game::ui)
          // .add_startup_system(load_lunar_lander)
          // .add_startup_system(spawn_all_meshes)
          // .add_startup_system(spawn_all_meshes)
          // .add_startup_system(load_gltf)
          // .add_startup_system(spawn_gltf_objects)
          .run();
}
// fn a() {
//   let mut a: String = "aaa".into();
//   let mut b: String = "aka".into();
//   (&mut a, &mut b) = ("a".to_string(), "5".to_string());
// }
fn rotate_camera(mut c:Query::<&mut Transform, With<Camera>>) {
  c.for_each_mut(|mut c| c.rotate(Quat::from_array([0.3, 0.5, 0.2, 0.6]).normalize()));
}
// fn rotate_camera(mut cameras: Query<&mut Transform, With<Camera>>) {
//   cameras.for_each_mut(|mut c| c.rotate(Quat::from_array([0.3, 0.5, 0.2, 0.6]).normalize()));
// }
// impl<T: Bundle> Bundle for Vec<Box<dyn Component>> {
//   fn component_ids(components: &mut bevy::ecs::component::Components,
//                    storages: &mut bevy::ecs::storage::Storages,
//                    ids: &mut impl FnMut(bevy::ecs::component::ComponentId)) {
//     Component::
//     todo!()
//   }

//   unsafe fn from_components<T, F>(ctx: &mut T, func: &mut F) -> Self
//     where F: for<'a> FnMut(&'a mut T) -> bevy::ptr::OwningPtr<'a>,
//           Self: Sized {
//     todo!()
//   }
// }
pub fn world_write_system(world: &mut World) {
  if let Some(player) = world.entity(entity).get_mut::<Player>() {
    // ...
  }
}
fn spawn_stuff(mut c: Commands,
               mut meshes: ResMut<Assets<Mesh>>,
               assets_gltf: Res<Assets<Gltf>>,
               asset_server: Res<AssetServer>,
               mut materials: ResMut<Assets<StandardMaterial>>) {
  c.get_or_spawn(entity)
  /* Create the ground. */
  c.spawn((Collider::cuboid(100.0, 0.1, 100.0),
           TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0))));
  /* Create the bouncing ball. */
  let TransformBundle { local, global } = TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0));
  c.spawn((RigidBody::Dynamic,
           Collider::ball(0.5),
           Restitution::coefficient(0.7),
           // TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)),
           PbrBundle { mesh:
                         meshes.add(Mesh::try_from(shape::Icosphere { radius: 0.5,
                                                                      subdivisions: 20 }).unwrap()),
                       transform: local,
                       global_transform: global,
                       ..default() }));
  // plane
  c.spawn(PbrBundle { mesh: meshes.add(shape::Plane::from_size(5.0).into()),
                      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                      ..default() });
  // cube
  c.spawn(PbrBundle { mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                      material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                      transform: Transform::from_xyz(0.0, 0.5, 0.0),
                      ..default() });
  // light
  c.spawn(PointLightBundle { point_light: PointLight { intensity: 1500.0,
                                                       shadows_enabled: true,
                                                       ..default() },
                             transform: Transform::from_xyz(4.0, 8.0, 4.0),
                             ..default() });
  // Events::drain()
  // EVENTWRITER::SEND_BATCH()
  C.SPAWN((Camera3dBundle { transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO,
                                                                                       Vec3::Y),
                            ..Default::default() },
           PanOrbitCamera { orbit_sensitivity: 1.5,
                            orbit_smoothness: 0.5,
                            pan_sensitivity: 1.1,
                            ..default() }));
}

fn reverse_string(mut s: String) -> String {
  let mut x = String::new();
  while let Some(c) = s.pop() {
    x.push(c);
  }
  x
}
