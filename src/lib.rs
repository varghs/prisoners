pub mod simulation;

enum Choice {
    COOPERATE,
    CHEAT,
}

#[derive(Clone, Copy)]
struct PayoffMatrix {
    payoffs: [f64; 4],
}

pub struct Game {
    player_one: Box<dyn Prisoner>,
    player_two: Box<dyn Prisoner>,
    num_rounds: usize,
}

impl Game {
    fn play(&mut self) -> Vec<RoundResult> {
        
    }
}

struct Round {
    player_one: Box<dyn Prisoner>,
    player_two: Box<dyn Prisoner>,
    matrix: PayoffMatrix,
}

struct RoundResult {
    player_one: String,
    player_two: String,
    result: (Choice, Choice)
}

impl Round {
    fn play(&mut self) -> RoundResult {
        let p1_choice = self.player_one.choose(self.matrix, self.player_two.name());
        let p2_choice = self.player_two.choose(self.matrix, self.player_one.name());

        RoundResult {
            player_one: self.player_one.name(),
            player_two: self.player_two.name(),
            result: (p1_choice, p2_choice)
        }
    }
}

trait Prisoner {
    fn name(&self) -> String;
    fn choose(&mut self, matrix: PayoffMatrix, opp: String) -> Choice;
}