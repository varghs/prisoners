struct Game {
    rounds: Vec<Round>,
}

enum Choice {
    COOPERATE,
    CHEAT,
}

struct PayoffMatrix {
    payoffs: &[f64],
}

struct Round {
    player_one: Prisoner,
    player_two: Prisoner,
    result: (Choice, Choice),
}

struct Prisoner {}

fn main() {
    println!("Hello, world!");
}
