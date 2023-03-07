use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(String::from("Failed to parse to move")),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let p1 = do_part1(&input);
    let p2 = do_part2(&input);
    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn use_enum_version(input: &str) -> String {
    return input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                .map(|char| char.parse::<Move>().unwrap())
                .collect();
            return moves[1] as u32
                + match (moves[0], moves[1]) {
                    (Move::Rock, Move::Rock)
                    | (Move::Paper, Move::Paper)
                    | (Move::Scissors, Move::Scissors) => 3,
                    (Move::Rock, Move::Scissors)
                    | (Move::Scissors, Move::Paper)
                    | (Move::Paper, Move::Rock) => 0,
                    (Move::Paper, Move::Scissors)
                    | (Move::Scissors, Move::Rock)
                    | (Move::Rock, Move::Paper) => 6,
                };
        })
        .sum::<u32>()
        .to_string();
}

fn do_part1(input: &str) -> String {
    let mut scores = HashMap::new();

    scores.insert("A X", 4);
    scores.insert("A Y", 8);
    scores.insert("A Z", 3);
    scores.insert("B X", 1);
    scores.insert("B Y", 5);
    scores.insert("B Z", 9);
    scores.insert("C X", 7);
    scores.insert("C Y", 2);
    scores.insert("C Z", 6);

    return input
        .lines()
        .map(|line| scores.get(&line).copied().unwrap())
        .sum::<u32>()
        .to_string();
}

fn do_part2(input: &str) -> String {
    let mut scores = HashMap::new();

    scores.insert("A X", 3);
    scores.insert("A Y", 4);
    scores.insert("A Z", 8);
    scores.insert("B X", 1);
    scores.insert("B Y", 5);
    scores.insert("B Z", 9);
    scores.insert("C X", 2);
    scores.insert("C Y", 6);
    scores.insert("C Z", 7);

    return input
        .lines()
        .map(|line| scores.get(&line).copied().unwrap())
        .sum::<u32>()
        .to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn p1_works() {
        assert_eq!(do_part1(INPUT), "15");
    }

    #[test]
    fn p2_works() {
        assert_eq!(do_part2(INPUT), "12");
    }

    #[test]
    fn use_enum_version_works() {
        assert_eq!(use_enum_version(INPUT), "15");
    }
}
