#[cfg(feature = "dev")]
use bevy_editor_pls::default_windows::cameras::EditorCamera;
use {crate::foxtrot::{level_instantiation::spawning::GameObject,
                      player_control::{actions::create_camera_action_input_manager_bundle,
                                       camera::IngameCamera}},
     bevy::prelude::*,
     bevy_dolly::prelude::*};

pub(crate) fn spawn(In(transform): In<Transform>, mut commands: Commands) {
  commands.spawn((IngameCamera::default(),
                  Camera3dBundle { transform,
                                   ..default() },
                  Rig::builder().with(Position::new(default()))
                                .with(YawPitch::new())
                                .with(Smooth::new_position_rotation(default(), default()))
                                .with(Arm::new(default()))
                                .with(LookAt::new(default()).tracking_predictive(true))
                                .build(),
                  create_camera_action_input_manager_bundle(),
                  Name::new("Main Camera"),
                  GameObject::Camera,
                  #[cfg(feature = "dev")]
                  EditorCamera));
}
