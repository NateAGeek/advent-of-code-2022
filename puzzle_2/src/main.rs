use std::{fs::File, io::{BufRead, BufReader}, str::FromStr};
use scan_fmt::scan_fmt_some;

#[derive(Debug, Clone, PartialEq)]
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
            "A" => Ok(GameHand::Rock),
            "B" => Ok(GameHand::Paper),
            "C" => Ok(GameHand::Scissors),
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

#[derive(Debug)]
struct GameParseError;
impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Game::Loss),
            "Y" => Ok(Game::Draw),
            "Z" => Ok(Game::Win),
            _ => Err(GameParseError)
        }
    }
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

    // Returns the hand that will beat the input hand
    fn winning_hand(hand: &GameHand) -> GameHand {
        match hand {
            GameHand::Rock => GameHand::Paper,
            GameHand::Paper => GameHand::Scissors,
            GameHand::Scissors => GameHand::Rock,
        }
    }

    // Returns the hand that will lose to the input hand
    fn losing_hand(hand: &GameHand) -> GameHand {
        match hand {
            GameHand::Paper => GameHand::Rock,
            GameHand::Scissors => GameHand::Paper,
            GameHand::Rock => GameHand::Scissors,
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
    fn player_expected_hand(opponent_hand: &GameHand, game_result: &Game) -> GameHand {
        match game_result {
            Game::Draw => opponent_hand.clone(),
            Game::Win  => Game::winning_hand(opponent_hand),
            Game::Loss => Game::losing_hand(opponent_hand)
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
        
        let (opponent_hand_input, game_outcome_input) = scan_fmt_some!(
            line.as_str(),
            "{} {}",
            String, String
        );

        let game_outcome = Game::from_str(game_outcome_input.unwrap().as_str()).unwrap();
        let game_outcome_score = Game::game_score(&game_outcome);
        let opponent_hand = GameHand::from_str(opponent_hand_input.unwrap().as_str()).unwrap();
        
        let player_hand = Game::player_expected_hand(&opponent_hand, &game_outcome);
        let player_hand_score = GameHand::hand_score(&player_hand);
        
        total_score += player_hand_score + game_outcome_score;
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

    mod example_games_part_one {
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
    mod example_games_part_two {
        use std::str::FromStr;
        use crate::{GameHand, Game};

        #[test]
        fn example_one() {
            let opponent_hand = GameHand::from_str("A").unwrap();
            let game_result = Game::from_str("Y").unwrap();
            let player_hand = Game::player_expected_hand(&opponent_hand, &game_result);

            let game_score = Game::game_score(&game_result);
            let player_score = GameHand::hand_score(&player_hand);
            assert_eq!(game_score + player_score, 4);
        }

        #[test]
        fn example_two() {
            let opponent_hand = GameHand::from_str("B").unwrap();
            let game_result = Game::from_str("X").unwrap();
            let player_hand = Game::player_expected_hand(&opponent_hand, &game_result);

            let game_score = Game::game_score(&game_result);
            let player_score = GameHand::hand_score(&player_hand);
            assert_eq!(game_score + player_score, 1);
        }
        #[test]

        fn example_three() {
            let opponent_hand = GameHand::from_str("C").unwrap();
            let game_result = Game::from_str("Z").unwrap();
            let player_hand = Game::player_expected_hand(&opponent_hand, &game_result);

            let game_score = Game::game_score(&game_result);
            let player_score = GameHand::hand_score(&player_hand);
            assert_eq!(game_score + player_score, 7);
        }
    }
}