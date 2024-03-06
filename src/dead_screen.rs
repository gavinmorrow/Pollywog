use bevy::prelude::*;

use crate::state::GameState;

pub struct DeadScreenPlugin;
impl Plugin for DeadScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Dead), setup)
            .add_systems(Update, restart_button_pressed);
    }
}

#[derive(Component)]
struct RestartButton;

fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::Hsla {
                hue: 0.0,
                saturation: 0.0,
                lightness: 0.0,
                alpha: 0.5,
            }),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "You died. Sorry!",
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..default()
            });
            parent
                .spawn(ButtonBundle {
                    style: Style { ..default() },
                    background_color: BackgroundColor(Color::BLACK),
                    border_color: BorderColor(Color::WHITE),
                    ..default()
                })
                .insert(RestartButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Restart",
                            TextStyle {
                                font_size: 42.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn restart_button_pressed(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, &RestartButton)>,
) {
    for (interaction, _button) in interaction_query.iter_mut() {
        match *interaction {
            // FIXME: game crashes when clicked, because it tries to re-create all
            //        entities, but they already exist (since they were never despawned).
            Interaction::Pressed => next_state.set(GameState::InGame),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
