use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

const INPUT_FILE: &str = "input";

struct LocationsList {
    left: Vec<isize>,
    right: Vec<isize>,
}

impl LocationsList {
    fn parse(input: &str) -> Option<Self> {
        let lines = input.lines();
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in lines {
            let words = line.split_whitespace().collect::<Vec<_>>();

            if words.is_empty() {
                continue;
            }
            if words.len() != 2 {
                return None;
            }

            left.push(words[0].parse::<isize>().ok()?);
            right.push(words[1].parse::<isize>().ok()?);
        }

        Some(Self { left, right })
    }

    fn part1(&mut self) -> Option<isize> {
        let left = &mut self.left;
        let right = &mut self.right;
        left.sort();
        right.sort();

        let combined = left.iter().zip(right.iter()).collect::<Vec<(_, _)>>();

        Some(combined.into_iter().map(|(x, y)| (*x - *y).abs()).sum())
    }

    fn part2(&self) -> Option<isize> {
        let mut right_count = HashMap::new();

        for i in &self.right {
            *right_count.entry(i).or_default() += 1;
        }
        Some(
            self.left
                .iter()
                .map(|x| x * *right_count.get(&x).unwrap_or(&0))
                .sum(),
        )
    }
}

impl FromStr for LocationsList {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match LocationsList::parse(input) {
            Some(v) => Ok(v),
            _ => Err("could not parse input".to_string()),
        }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("could not read input file!");

    println!(
        "part1: {}",
        input
            .parse::<LocationsList>()
            .unwrap()
            .part1()
            .expect("could not parse input")
    );
    println!(
        "part2: {}",
        input
            .parse::<LocationsList>()
            .unwrap()
            .part2()
            .expect("could not parse input")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = "
            3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(input.parse::<LocationsList>().unwrap().part1().unwrap(), 11);
    }
    #[test]
    fn part1_test2() {
        let input = "
            3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(input.parse::<LocationsList>().unwrap().part2().unwrap(), 31);
    }
}
