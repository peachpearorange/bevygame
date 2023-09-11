use rust_utils::comment;

use {crate::components::InputState,
     bevy::{ecs::event::Event,
            input::{keyboard::keyboard_input_system, InputPlugin},
            prelude::{Input, KeyCode, Res, *},
            utils::HashSet}};

fn log_inputs(keys: Res<Input<KeyCode>>) {
  keys.get_just_pressed().for_each(|k| {
                           println!("{} was pressed!",
                                    if k == &KeyCode::F { "F" } else { "Some key" })
                         });
}
pub fn keylogger(app: &mut App) { app.add_system(log_inputs); }

use {bevy::prelude::*, leafwing_input_manager::prelude::*};

// fn main() {
//   App::new().add_plugins(DefaultPlugins)
//             // This plugin maps inputs to an input-type agnostic action-state
//             // We need to provide it with an enum which stores the possible actions a player could take
//             .add_plugin(InputManagerPlugin::<Action>::default())
//             // The InputMap and ActionState components will be added to any entity with the Player component
//             .add_startup_system(spawn_player)
//             // Read the ActionState in your systems using queries!
//             .add_system(jump)
//             .run();
// }

// fn spawn_player(mut commands: Commands) {
//   commands.spawn(InputManagerBundle::<Action> { // Stores "which actions are currently pressed"
//                                                 action_state: ActionState::default(),
//                                                 // Describes how to convert from player inputs into those actions
//                                                 input_map:
//                                                   InputMap::new([(KeyCode::Space,
//                                                                   Action::Jump)]) })
//           .insert(Player);
// }

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
// #[bevy_plugin]
// pub fn ActionsPlugin(app: &mut App) {
//   app.init_resource::<Actions>()
//      .add_system(set_movement_actions.in_set(OnUpdate(GameState::Playing)));
// }
// #[derive(Default, Resource)]
// pub struct Actions {
//   pub player_movement: Option<Vec2>,
// }
// pub fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
//   let player_movement = Vec2::new(get_movement(GameControl::Right, &keyboard_input)
//                                   - get_movement(GameControl::Left, &keyboard_input),
//                                   get_movement(GameControl::Up, &keyboard_input)
//                                   - get_movement(GameControl::Down, &keyboard_input));

//   if player_movement != Vec2::ZERO {
//     actions.player_movement = Some(player_movement.normalize());
//   } else {
//     actions.player_movement = None;
//   }
// }

#[derive(Resource, Default)]
pub struct PressedKeys(pub HashSet<KeyCode>);
fn get_pressed_keys_system(mut r: ResMut<PressedKeys>, i: Res<Input<KeyCode>>) {
  *r.0 = i.get_pressed().collect();
}
pub fn get_pressed_keys_plugin(app: &mut App) {
  app.init_resource::<PressedKeys>()
     .add_system(get_pressed_keys_system);
}

comment! {
  impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
      let p = |k| keyboard_input.pressed(k);
      match self {
        GameControl::Up => p(KeyCode::W) || p(KeyCode::Up),
        GameControl::Down => p(KeyCode::S) || p(KeyCode::Down),
        GameControl::Left => p(KeyCode::A) || p(KeyCode::Left),
        GameControl::Right => p(KeyCode::D) || p(KeyCode::Right)
      }
    }
  }
}
