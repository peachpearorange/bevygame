use {crate::loading::{Images, Meshes},
     bevy::{gltf::Gltf, prelude::*, render::render_resource::AsBindGroupShaderType},
     bevy_panorbit_camera::PanOrbitCamera,
     bevy_rapier3d::prelude::*};

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
  for transform in positions.iter() {
    println!("Ball altitude: {}", transform.translation.y);
  }
}
pub fn spawn_planets_and_lunar_lander(mut c: Commands,
                                      // mut meshes: ResMut<Assets<Mesh>>,
                                      mut images: ResMut<Images>,
                                      // mut meshes: ResMut<Meshes>,
                                      // assets_gltf: Res<Assets<Gltf>>,
                                      asset_server: Res<AssetServer>,
                                      mut meshes: ResMut<Assets<Mesh>>,
                                      mut materials: ResMut<Assets<StandardMaterial>>) {
  let planet = |[x, y, z]: [f32; 3], radius: f32| {
    (RigidBody::Fixed,
     Collider::ball(radius),
     PbrBundle { mesh: meshes.add(Mesh::from(shape::Icosphere { radius,
                                                                subdivisions: 20 })),
                 transform: Transform::from_xyz(x, y, z),
                 material: materials.add(StandardMaterial::from(images.get("planet.png"))),
                 ..default() })
  };
  c.spawn(planet([3.4, 5.6, 7.8], 12.0));

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
           Restitution::coefficient(0.7),
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
