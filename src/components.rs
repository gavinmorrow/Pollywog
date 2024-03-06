pub mod character;
pub mod collect_coin;
pub mod damage;
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
            npc_movement::NpcMovementPlugin,
            player_win::PlayerWinPlugin,
            damage::DamagePlugin,
            collect_coin::CoinPlugin,
        ));
    }
}
