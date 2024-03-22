use bevy::prelude::*;

use crate::{level, state::GameState};

pub fn dead_screen_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Dead), setup)
        .add_systems(Update, (restart_button_pressed, esc_button_pressed))
        .add_systems(OnExit(GameState::Dead), cleanup);
}

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct EscButton;

#[derive(Component)]
struct RootNode;

fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
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
        .insert(RootNode)
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

            parent
                .spawn(ButtonBundle {
                    style: Style { ..default() },
                    background_color: BackgroundColor(Color::BLACK),
                    border_color: BorderColor(Color::WHITE),
                    ..default()
                })
                .insert(EscButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Back to Start Screen",
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

fn cleanup(
    mut commands: Commands,
    dead_screen_root_node: Query<Entity, With<RootNode>>,
    level_entities: Query<Entity, With<crate::level::LevelEntity>>,
) {
    // Despawn the dead screen
    let root_node = dead_screen_root_node.single();
    commands.entity(root_node).despawn_recursive();

    // Despawn the level
    level::despawn_entities(&mut commands, level_entities);
}

fn restart_button_pressed(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, &RestartButton)>,
) {
    for (interaction, _button) in interaction_query.iter_mut() {
        match *interaction {
            // FIXME: game crashes when clicked, because it tries to re-create all
            //        entities, but they already exist (since they were never despawned).
            // Possible solution:
            //  - despawn all entities when the game state changes out of Dead
            //        (*not* after InGame, because then you can't see the render of the
            //         game underneath the dead screen ui)
            Interaction::Pressed => next_state.set(GameState::InGame),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn esc_button_pressed(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, &EscButton)>,
) {
    for (interaction, _button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => next_state.set(GameState::StartScreen),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
