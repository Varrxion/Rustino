use std::io::stdin;

use crate::game::varr_util;

pub fn user_select_stakes(highroller: bool, chips: i32) -> i32 {
    let mut stakes = 0;
    loop {
        println!("You currently have {} chips", chips);
        println!("[1] Bet 1");
        println!("[2] Bet 3");
        println!("[3] Bet 5");
        println!("[4] Bet 10");
        println!("[5] Bet 25");
        println!("[6][HR] Bet 50");
        println!("[7][HR] Bet 100");
        println!("[8][HR] Bet 250");
        println!("[9][HR] Custom Bet");
        println!("[0] Exit");

        let mut rawinput = String::new();
        stdin().read_line(&mut rawinput).expect("Failed to read user input");
        let input = rawinput.trim();

        if input.eq_ignore_ascii_case("1") && chips >= 1 {
            stakes = 1;
        }
        else if input.eq_ignore_ascii_case("2") && chips >= 3 {
            stakes = 3;
        }
        else if input.eq_ignore_ascii_case("3") && chips >= 5 {
            stakes = 5;
        }
        else if input.eq_ignore_ascii_case("4") && chips >= 10 {
            stakes = 10;
        }
        else if input.eq_ignore_ascii_case("5") && chips >= 25 {
            stakes = 25;
        }
        else if input.eq_ignore_ascii_case("6") && highroller && chips >= 50 {
            stakes = 50;
        }
        else if input.eq_ignore_ascii_case("7") && highroller && chips >= 100 {
            stakes = 100;
        }
        else if input.eq_ignore_ascii_case("8") && highroller && chips >= 250 {
            stakes = 250;
        }
        else if input.eq_ignore_ascii_case("9") && highroller {
            let user_bet = varr_util::get_integer_input();
            if user_bet <= chips && user_bet >= 0 {
                stakes = user_bet;
            }
            else if user_bet < 0 {
                println!("Invalid Selection (Nice try)");
            }
            else {
                println!("Invalid Selection (Do you have enough chips?)");
            }
        }
        else if input.eq_ignore_ascii_case("0") {
            stakes = -1;
        }
        else {
            println!("Invalid Selection (Do you have enough chips? Are you a highroller? Was the input mistyped?)");
        }

        if stakes != 0 {
            break;
        }
    }
    return stakes;
}