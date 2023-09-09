// loading::TextureAssets,
use {crate::{actions::Actions, GameState},
     bevy::prelude::*,
     bevy_fn_plugin::bevy_plugin};

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`

#[bevy_plugin]
pub fn PlayerPlugin(app: &mut App) {
  app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
     .add_system(move_player.in_set(OnUpdate(GameState::Playing)));
}
fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
  commands.spawn(SpriteBundle { texture: textures.texture_bevy.clone(),
                                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                                ..Default::default() })
          .insert(Player);
}
fn move_player(time: Res<Time>,
               actions: Res<Actions>,
               mut player_query: Query<&mut Transform, With<Player>>) {
  if !actions.player_movement.is_none() {
    let speed = 150.;
    let movement = Vec3::new(actions.player_movement.unwrap().x * speed * time.delta_seconds(),
                             actions.player_movement.unwrap().y * speed * time.delta_seconds(),
                             0.);
    for mut player_transform in &mut player_query {
      player_transform.translation += movement;
    }
  }
}
