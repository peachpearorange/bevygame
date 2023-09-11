// loading::FontAssets,
use {bevy::prelude::*, rust_utils::comment};

#[derive(Resource)]
struct ButtonColors {
  normal: Color,
  hovered: Color
}

impl Default for ButtonColors {
  fn default() -> Self {
    ButtonColors { normal: Color::rgb(0.15, 0.15, 0.15),
                   hovered: Color::rgb(0.25, 0.25, 0.25) }
  }
}

comment! {
  fn click_play_button(button_colors: Res<ButtonColors>,
                       mut state: ResMut<NextState<GameState>>,
                       mut interaction_query: Query<(&Interaction, &mut BackgroundColor),
                                                    (Changed<Interaction>, With<Button>)>) {
    for (interaction, mut color) in &mut interaction_query {
      match *interaction {
        Interaction::Clicked => {
          state.set(GameState::Playing);
        }
        Interaction::Hovered => {
          *color = button_colors.hovered.into();
        }
        Interaction::None => {
          *color = button_colors.normal.into();
        }
      }
    }
  }

  fn cleanup_menu(mut c: Commands, button: Query<Entity, With<Button>>) {
    c.entity(button.single()).despawn_recursive();
  }

  /// This plugin is responsible for the game menu (containing only one button...)
  /// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
  pub fn menu_plugin(app: &mut App) {
    app.init_resource::<ButtonColors>()
      .add_systems(OnEnter(GameState::Menu),
                   (setup_menu, click_play_button, cleanup_menu));
  }

}
