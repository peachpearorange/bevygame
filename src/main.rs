#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_braces)]
#![feature(const_trait_impl)]
#![feature(type_alias_impl_trait)]
#![allow(unused_mut)]

use loading::loading_plugin;

use {bevy::asset::LoadState, seldom_fn_plugin::FnPluginExt};

use {crate::game::game_plugin,
     bevy::{gltf::Gltf, window},
     bevy_asset_loader::prelude::*,
     bevy_panorbit_camera::PanOrbitCamera};

// pub mod foxtrot;
pub mod components;
pub mod game;
pub mod gamething;
pub mod tests;
// pub mod gol;
// pub mod game2d;
pub mod input;
// pub mod lunarlander3d;
// pub mod tiles;
// // mod audio;
// // mod menu;
pub mod loading;
pub mod lunarlander3d;
pub mod menu;
mod trait_extension;
// pub mod text2d;

// use bevy_game::GamePlugin; // ToDo: Replace bevy_game with your new crate name.
use {bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows, DefaultPlugins},
     // bevy_fps_controller,
     bevy_panorbit_camera,
     bevy_rapier3d::plugin::RapierPhysicsPlugin,
     // bundles::player,
     rust_utils::comment,
     winit::window::Icon};
// todo: figure out how to load gltf files
#[bevy_main]
pub fn main() {
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
          // .insert_resource(ClearColor(Color::SALMON))
          // .insert_resource(game::generate_level())
          // .add_startup_system(game::init)
          // .add_system(rotate_camera)
          .add_system(bevy::window::close_on_esc)
          .fn_plugin(input::keylogger)
          .fn_plugin(input::get_pressed_keys_plugin)
          .fn_plugin(tests::tests_plugin)
          .fn_plugin(loading_plugin)
          .fn_plugin(game_plugin)
          .add_plugins(RapierPhysicsPlugin::default())
          .add_plugin(bevy_panorbit_camera::PanOrbitCameraPlugin)
          // .add_plugin(bevy_fps_controller::controller::FpsControllerPlugin)
          // .add_plugin(lunarlander3d::LunarLander)
          // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
          .add_startup_system(spawn_planets_and_lunar_lander)
          .add_startup_system(spawn_stuff)
          .add_plugin(game_plugin)
          // .add_system(game::ui)
          // .add_startup_system(load_lunar_lander)
          // .add_startup_system(load_gltf)
          // .add_startup_system(spawn_gltf_objects)
          .run();
}
fn rotate_camera(mut c: Query<&mut Transform, With<Camera>>) {
  c.for_each_mut(|mut c| c.rotate(Quat::from_array([0.3, 0.5, 0.2, 0.6]).normalize()));
}
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
  // c.get_or_spawn(entity)
  /* Create the ground. */
  c.spawn((Collider::cuboid(100.0, 0.1, 100.0),
           TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0))));
  /* Create the bouncing ball. */
  let TransformBundle { local, .. } =
    TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0));
  let ico = Mesh::try_from(shape::Icosphere { radius: 0.5,
                                              subdivisions: 20 }).unwrap();
  c.spawn((RigidBody::Dynamic,
           Collider::ball(0.5),
           // TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)),
           PbrBundle { mesh: meshes.add(ico),
                       transform: local,
                       ..default() }));
  // plane
  c.spawn(PbrBundle { mesh: meshes.add(shape::Plane::from_size(5.0).into()),
                      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                      ..default() });
  // cube
  // let player_shape = shape::Capsule{ radius: todo!(), rings: todo!(), depth: todo!(), latitudes: todo!(), longitudes: todo!(), uv_profile: todo!() }
  let cube = |size| Mesh::from(shape::Cube { size });
  let cube_mesh_handle = meshes.add(cube(1.0));
  c.spawn(PbrBundle { mesh: cube_mesh_handle,
                      material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                      transform: Transform::from_xyz(0.0, 0.5, 0.0),
                      ..default() });
  // light
  c.spawn(PointLightBundle { point_light: PointLight { intensity: 1500.0,
                                                       shadows_enabled: true,
                                                       ..default() },
                             transform: Transform::from_xyz(4.0, 8.0, 4.0),
                             ..default() });
  c.spawn((Camera3dBundle { transform:
                              Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO,
                                                                              Vec3::Y),
                            ..Default::default() },
           PanOrbitCamera { orbit_sensitivity: 1.5,
                            orbit_smoothness: 0.5,
                            pan_sensitivity: 1.1,
                            ..default() }));
}
