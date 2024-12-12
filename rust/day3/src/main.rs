use regex::Regex;
use std::fs;

const INPUT_FILE: &str = "input";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("could not read from file");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(don't)()|(do)()").unwrap();

    let c = re
        .captures_iter(&input)
        .map(|c| {
            let (_, a): (&str, [&str; 2]) = c.extract();

            println!("{:?}", a);
            match a {
                ["do", _] => (-1, -1),
                ["don't", _] => (-1, -2),
                [n, m] => (n.parse::<isize>().unwrap(), m.parse::<isize>().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    let part1: isize = c.iter().filter(|(a, _)| *a >= 0).map(|(a, b)| a * b).sum();
    let part2: isize = c
        .iter()
        .fold((true, 0), |(t, sum), (a, b)| {
            if *a < 0 {
                (a == b, sum)
            } else if t {
                (true, sum + (*a * *b))
            } else {
                (false, sum)
            }
        })
        .1;
    println!("part1: {}\npart2: {}", part1, part2);
}
