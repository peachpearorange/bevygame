// Spawn a spaceship
fn spawn_spaceship(commands: &mut Commands) {
  commands.spawn_bundle(SpriteBundle { sprite: Sprite { color: Color::rgb(1.0, 1.0, 1.0),
                                                        ..default() },
                                       ..default() });
}

// Spawn an asteroid
fn spawn_asteroid(commands: &mut Commands, size: f32) {
  commands.spawn_bundle(SpriteBundle { sprite: Sprite { color: Color::rgb(0.5, 0.5, 0.5),
                                                        size,
                                                        ..default() },
                                       ..default() });
}

// Spawn a planet
fn spawn_planet(commands: &mut Commands, radius: f32) {
  commands.spawn_bundle(SpriteBundle { sprite: Sprite { color: Color::rgb(0.2, 0.4, 0.6),
                                                        size: radius * 2.0,
                                                        ..default() } });
}

// Add a skybox
fn setup_skybox(commands: &mut Commands) { commands.spawn_bundle(SkyboxBundle::default()); }

// Apply silly physics to ships and asteroids
fn space_physics(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity), With<SpaceObject>>) {
  for (mut transform, mut velocity) in query.iter_mut() {
    let direction = velocity.clone().normalize();
    velocity += direction * time.delta_seconds * 5.0;

    transform.translation += velocity * time.delta_seconds;
  }
}
