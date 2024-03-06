use bevy::prelude::*;

use crate::state::GameState;

pub struct DeadScreenPlugin;
impl Plugin for DeadScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Dead), setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
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
        });
}
