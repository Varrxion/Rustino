use std::io::stdin;
use crate::game::events;
use crate::game::game_state;

pub fn start() {
    let mut game_state = game_state::GameState::new(false, 100, 0, );

    loop {
        println!("You currently have {} chips", game_state.get_chips());
        println!("[0] Beg for money");
        println!("[1] Visit the Gift Shop");
        println!("[2] Play Double or Nothing");
        println!("[inv] View Inventory");
        println!("[quit] Exit the game");

        let mut rawinput = String::new();
        stdin().read_line(&mut rawinput).expect("Failed to read user input");
        let input = rawinput.trim();

        if input.eq_ignore_ascii_case("0") {
            events::beg::play(&mut game_state);
        }
        else if input.eq_ignore_ascii_case("1") {
            events::shop::play(&mut game_state);
        }
        else if input.eq_ignore_ascii_case("2") {
            events::double_or_nothing::play(&mut game_state);
        }
        else if input.eq_ignore_ascii_case("inv") {
            events::view_inv::play(&mut game_state);
        }
        else if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("stop") {
            break;
        }
        else {
            println!("Invalid Selection\n")
        }
    }
    println!("GAME OVER!")
}