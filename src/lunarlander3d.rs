// use {crate::{bundles::{self, player},
//              components::{self, RandomMovement},
//              input,
//              tiles::{self, Tile, *}},
//      bevy::{ecs::{schedule, system::WithEntity, world::EntityMut},
//             gltf::Gltf,
//             pbr::{ClusterConfig, ClusterFarZMode},
//             prelude::*,
//             render::RenderPlugin,
//             scene::SceneInstance,
//             utils::HashMap,
//             window::WindowResolution},
//      bevy_fn_plugin::bevy_plugin,
//      bevy_fps_controller::controller::*,
//      bevy_rapier3d::{control::KinematicCharacterController,
//                      parry::query::sat::triangle_segment_find_local_separating_normal_oneway,
//                      prelude::{NoUserData, RapierPhysicsPlugin},
//                      render::RapierDebugRenderPlugin},
//      cascade::cascade,
//      ndarray::{Array3, ArrayBase},
//      rand::thread_rng,
//      rust_utils::{change, coll_max}};

use bevy_panorbit_camera::PanOrbitCamera;

use {bevy::prelude::*, bevy_fn_plugin::bevy_plugin, bevy_rapier3d::prelude::*};

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
  for transform in positions.iter() {
    println!("Ball altitude: {}", transform.translation.y);
  }
}

#[bevy_plugin]
pub(crate) fn LunarLander(app: &mut App) {
  app
     // .add_startup_system(spawn_planets_and_lunar_lander)
     .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
}

fn spawn_planets_and_lunar_lander(mut c: Commands,
                                  mut meshes: ResMut<Assets<Mesh>>,
                                  mut materials: ResMut<Assets<StandardMaterial>>) {
}
