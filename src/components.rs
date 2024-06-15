pub mod animated_sprite;
pub mod background;
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
            animated_sprite::animated_sprite_plugin,
            background::background_plugin,
            character::character_plugin,
            collect_coin::coin_plugin,
            damage::damage_plugin,
            kills_player::kills_player_plugin,
            npc_movement::npc_movement_plugin,
            player_win::player_win_plugin,
        ));
    }
}

pub fn cleanup(
    next_grapple_state: bevy::prelude::ResMut<
        bevy::prelude::NextState<character::grapple::GrappleState>,
    >,
) {
    character::cleanup(next_grapple_state);
}
