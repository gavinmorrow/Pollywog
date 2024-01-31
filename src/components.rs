pub mod character;
pub mod damage;
pub mod dead_screen_plugin;
pub mod health;
pub mod jump;
pub mod kills_player;
pub mod npc_movement;
pub mod player_win;

pub struct ComponentsPlugin;
impl bevy::prelude::Plugin for ComponentsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            character::CharacterPlugin,
            kills_player::KillsPlayerPlugin,
            dead_screen_plugin::DeadScreenPlugin,
            npc_movement::NpcMovementPlugin,
            player_win::PlayerWinPlugin,
            damage::DamagePlugin,
        ));
    }
}
