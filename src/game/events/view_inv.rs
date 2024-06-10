use crate::game::game_state::GameState;

pub fn play(game_state: &mut GameState) {
    println!("======Inventory======");
        if game_state.get_inventory(0) {
            println!("Highroller License");
        }
        if game_state.get_inventory(1) {
            println!("Lucky shirt (+1 to Luck)");
        }
        if game_state.get_inventory(2) {
            println!("Lucky pants (+2 to Luck)");
        }
    println!("=====================");
}