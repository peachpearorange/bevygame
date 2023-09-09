use {crate::{actions::{set_movement_actions, Actions},
             loading::AudioAssets,
             GameState},
     bevy::prelude::*,
     bevy_kira_audio::prelude::*};

// This plugin is responsible to control the game audio

#[bevy_plugin]
pub fn InternalAudioPlugin(app: &mut App) {
  app.add_plugin(AudioPlugin)
     .add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
     .add_system(control_flying_sound.after(set_movement_actions)
                                     .in_set(OnUpdate(GameState::Playing)));
}
#[derive(Resource)]
struct FlyingAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
  audio.pause();
  let handle = audio.play(audio_assets.flying.clone())
                    .looped()
                    .with_volume(0.3)
                    .handle();
  commands.insert_resource(FlyingAudio(handle));
}

fn control_flying_sound(actions: Res<Actions>,
                        audio: Res<FlyingAudio>,
                        mut audio_instances: ResMut<Assets<AudioInstance>>) {
  if let Some(instance) = audio_instances.get_mut(&audio.0) {
    match instance.state() {
      PlaybackState::Paused { .. } =>
        if actions.player_movement.is_some() {
          instance.resume(AudioTween::default());
        },
      PlaybackState::Playing { .. } =>
        if actions.player_movement.is_none() {
          instance.pause(AudioTween::default());
        },
      _ => {}
    }
  }
}
