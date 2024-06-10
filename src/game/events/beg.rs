use crate::game::game_state::GameState;

pub fn play(game_state: &mut GameState) {
    if game_state.get_chips() == 0 {
        game_state.set_chips(1);
    }
}