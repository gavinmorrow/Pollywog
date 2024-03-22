use bevy::prelude::*;

use crate::state::GameState;
use crate::BACKGROUND_COLOR;

const BUTTON_BG_PRESSED: BackgroundColor = BackgroundColor(Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.75,
    alpha: 1.0,
});
const BUTTON_BG_HOVER: BackgroundColor = BackgroundColor(Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.9,
    alpha: 1.0,
});
const BUTTON_BG_NORMAL: BackgroundColor = BackgroundColor(Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 1.0,
    alpha: 1.0,
});

pub fn start_screen_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::StartScreen), setup)
        .add_systems(Update, (button_interaction_style, start_button_pressed))
        .add_systems(OnExit(GameState::StartScreen), cleanup);
}

fn setup(mut commands: Commands) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(BACKGROUND_COLOR),
            ..default()
        })
        .insert(RootNode)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style { ..default() },
                    background_color: BackgroundColor(Color::WHITE),
                    ..default()
                })
                .insert(StartButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Start",
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

// LINT ALLOW: Ok because it's a bevy Query, and not actually very complex
#[allow(clippy::type_complexity)]
fn button_interaction_style(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BUTTON_BG_PRESSED;
            }
            Interaction::Hovered => {
                *bg_color = BUTTON_BG_HOVER;
            }
            Interaction::None => {
                *bg_color = BUTTON_BG_NORMAL;
            }
        }
    }
}

fn start_button_pressed(
    interactions: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if interaction == &Interaction::Pressed {
            debug!("Start button pressed");
            next_state.set(GameState::InGame);
        }
    }
}

fn cleanup(mut commands: Commands, root_node: Query<Entity, With<RootNode>>) {
    let root_node = root_node.single();
    commands.entity(root_node).despawn_recursive();
}

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct RootNode;
