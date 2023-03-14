use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let p1 = do_part1(&input);
    println!("P1: {}", p1);
}

fn do_part1(input: &str) -> String {
    let scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();
    return input
        .lines()
        .map(|line| {
            let (c1, c2) = line.split_at(line.len() / 2);
            let found = c1.chars().find(|c| c2.contains(*c)).unwrap();
            return scores.get(&found).unwrap();
        })
        .sum::<usize>()
        .to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn p1_works() {
        assert_eq!(do_part1(INPUT), "157");
    }
}
