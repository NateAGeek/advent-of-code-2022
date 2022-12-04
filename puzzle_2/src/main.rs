use std::{fs::File, io::{BufRead, BufReader}, str::FromStr};
use scan_fmt::scan_fmt_some;

#[derive(Debug, PartialEq)]
enum GameHand {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
struct GameHandParseError;
impl FromStr for GameHand {
    type Err = GameHandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(GameHand::Rock),
            "B" | "Y" => Ok(GameHand::Paper),
            "C" | "Z" => Ok(GameHand::Scissors),
            _ => Err(GameHandParseError)
        }
    }
}
impl GameHand {
    fn hand_score(hand: &GameHand) -> i32 {
        match hand {
            GameHand::Rock => 1,
            GameHand::Paper => 2,
            GameHand::Scissors => 3
        }
    }
}

#[derive(Debug, PartialEq)]
enum Game {
    Win,
    Draw,
    Loss
}

impl Game {
    fn play(player_hand: &GameHand, opponent_hand: &GameHand) -> Game {
        match (player_hand, opponent_hand) {
            (GameHand::Rock, GameHand::Scissors) => Game::Win,
            (GameHand::Paper, GameHand::Rock) => Game::Win,
            (GameHand::Scissors, GameHand::Paper) => Game::Win,
            (player_hand, opponent_hand) 
                if player_hand == opponent_hand => Game::Draw,
            _ => Game::Loss
        }
    }
    fn game_score(game_result: &Game) -> i32 {
        if game_result == &Game::Win {
            6
        } else if game_result == &Game::Draw {
            3
        } else {
            0
        }
    }
}

// 1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won

fn main() {
    let file = File::open("./input.txt").unwrap();
    let file_reader = BufReader::new(file);

    let mut total_score = 0;

    for line in file_reader.lines() {
        // Read input from line
        let line = line.unwrap();
        
        let (opponent_hand_input, player_hand_input) = scan_fmt_some!(
            line.as_str(),
            "{} {}",
            String, String
        );

        let player_hand = GameHand::from_str(player_hand_input.unwrap().as_str()).unwrap();
        let opponent_hand = GameHand::from_str(opponent_hand_input.unwrap().as_str()).unwrap();
        
        let game_results = Game::play(&player_hand, &opponent_hand);
        let win_loss_score = Game::game_score(&game_results);

        let player_hand_score = GameHand::hand_score(&player_hand);
        
        total_score += player_hand_score + win_loss_score;
    }
    println!("File total score: {}", total_score);
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::GameHand;

    #[test]
    fn hands() {
        let rock = GameHand::from_str("A").unwrap();
        assert_eq!(rock, GameHand::Rock);
        let rock = GameHand::from_str("X").unwrap();
        assert_eq!(rock, GameHand::Rock);

        let paper = GameHand::from_str("B").unwrap();
        assert_eq!(paper, GameHand::Paper);
        let paper = GameHand::from_str("Y").unwrap();
        assert_eq!(paper, GameHand::Paper);

        let scissors = GameHand::from_str("C").unwrap();
        assert_eq!(scissors, GameHand::Scissors);
        let scissors = GameHand::from_str("Z").unwrap();
        assert_eq!(scissors, GameHand::Scissors);
    }

    mod example_games {
        use std::str::FromStr;
        use crate::{GameHand, Game};

        #[test]
        fn example_one() {
            let opponent_hand = GameHand::from_str("A").unwrap();
            let player_hand = GameHand::from_str("Y").unwrap();
            let game_result = Game::play(&player_hand, &opponent_hand);
            assert_eq!(game_result, Game::Win);
            let game_score = Game::game_score(&game_result);
            assert_eq!(game_score, 6)
        }
    }
}