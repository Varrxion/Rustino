mod game;

fn main() {
    println!("Welcome to Rustino!");
    game::event_loop::start();
}
