#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::main;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_u32(n: u32) -> Move {
        match n {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => panic!("Invalid move!"),
        }
    }

    fn beats(&self, other: &Move) -> bool {
        match (self, other) {
            (Move::Rock, Move::Scissors) => true,
            (Move::Paper, Move::Rock) => true,
            (Move::Scissors, Move::Paper) => true,
            _ => false,
        }
    }
}

#[main]
fn main() {
    let player_move = Move::from_u32(rand::thread_rng().gen_range(0..3));

    let computer_move = Move::from_u32(rand::thread_rng().gen_range(0..3));

    // Print moves
    println!("Player move: {:?}", player_move);
    println!("Computer move: {:?}", computer_move);

    // Determine the outcome
    let outcome = if player_move == computer_move {
        "It's a tie!"
    } else if player_move.beats(&computer_move) {
        "You win!"
    } else {
        "You lose!"
    };

    println!("{}", outcome);

    // Assert the outcome for testing
    assert!(player_move == computer_move || player_move.beats(&computer_move) || computer_move.beats(&player_move));
}