mod game;
mod test;

use game::input;
use game::pvp;
use game::pve;

use test::minmax_tests;

fn main() {
    minmax_tests();
    println!("Welcome to Tic Tac Toe!\nChoose a mode:\n1. Player vs Player\n2. Player vs Computer");
    loop {
        let mode = input("Enter mode: ");
        match mode.as_str() {
            "1" => pvp(),
            "2" => pve(),
            _ => println!("Invalid input"),
        }
        println!("Thanks for playing!\nDo you want to play again? (y/n) (n)");
        let mut inp = input("> ");
        inp.make_ascii_lowercase();
        if inp != "y" {
            break;
        }
    }
}