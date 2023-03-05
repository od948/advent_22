fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let p1 = do_part1(&input);
    let p2 = do_part2(&input);
    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn do_part1(input: &str) -> String {
    return input
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|food| food.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string();
}

fn do_part2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|food| food.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    return result.iter().take(3).sum::<u32>().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn p1_works() {
        assert_eq!(do_part1(INPUT), "24000");
    }

    #[test]
    fn p2_works() {
        assert_eq!(do_part2(INPUT), "45000");
    }
}
