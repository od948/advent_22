use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let p1 = do_part1(&input);
    let p2 = do_part2(&input);
    println!("P1: {}", p1);
    println!("P2: {}", p2);
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
}
