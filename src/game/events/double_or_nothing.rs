use rand::Rng;

use crate::game::{game_state::GameState, rustino_util};

pub fn play(game_state: &mut GameState) {
    loop{
        let stakes = rustino_util::user_select_stakes(game_state.get_highroller(), game_state.get_chips());

        if stakes != -1 {
            let result = coinflip(game_state.get_luck());
            if result {
                game_state.set_chips(game_state.get_chips() + stakes);
            }
            else {
                game_state.set_chips(game_state.get_chips() - stakes);
            }
        } else{
            break;
        }
    }
}

fn coinflip(luck: i32) -> bool {
    let mut rng = rand::thread_rng();
    let result = rng.gen_bool(0.5+(luck as f64/100.0));
    return result;
}