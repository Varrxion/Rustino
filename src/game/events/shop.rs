use std::io::stdin;

use crate::game::game_state::GameState;

pub fn play(game_state: &mut GameState) {
    loop {
        println!("You currently have {} chips", game_state.get_chips());
        if !game_state.get_inventory(0) {
            println!("[1] Cost 100, Highroller License");
        }
        if !game_state.get_inventory(1) {
            println!("[2] Cost 25, Lucky shirt (+1 to Luck)");
        }
        if !game_state.get_inventory(2) {
            println!("[3] Cost 50, Lucky pants (+2 to Luck)");
        }
        println!("[0] Exit");

        let mut rawinput = String::new();
        stdin().read_line(&mut rawinput).expect("Failed to read user input");
        let input = rawinput.trim();

        if input.eq_ignore_ascii_case("1") && game_state.get_chips() >= 100 && !game_state.get_inventory(0) {
            game_state.set_inventory(0, true);
            game_state.set_highroller(true);
            game_state.set_chips(game_state.get_chips() - 100);
        }
        else if input.eq_ignore_ascii_case("2") && game_state.get_chips() >= 25 && !game_state.get_inventory(1) {
            game_state.set_inventory(1, true);
            game_state.set_luck(game_state.get_luck() + 1);
            game_state.set_chips(game_state.get_chips() - 25);
        }
        else if input.eq_ignore_ascii_case("3") && game_state.get_chips() >= 50 && !game_state.get_inventory(2) {
            game_state.set_inventory(2, true);
            game_state.set_luck(game_state.get_luck() + 2);
            game_state.set_chips(game_state.get_chips() - 50);
        }
        else if input.eq_ignore_ascii_case("0") {
            break;
        }
        else {
            println!("Invalid Input")
        }
    }
}