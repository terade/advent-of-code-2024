use std::fs;

const INPUT_FILE: &str = "input";

fn solve(
    directions: Vec<Vec<(isize, isize)>>,
    needle: &str,
    input: &Vec<Vec<char>>,
    part2: bool,
) -> usize {
    let width = input[0].len();
    let height = input.len();
    let mut sum = 0;
    let needle_len = needle.len();
    for i in 0..height as isize {
        for j in 0..width as isize {
            let im = directions
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|(x, y)| (*x + j, *y + i))
                        .filter(|(x, y)| {
                            *x >= 0 && *y >= 0 && *x < width as isize && *y < height as isize
                        })
                        .collect::<Vec<_>>()
                })
                .filter(|r| r.len() == needle_len)
                .map(|r| {
                    r.iter()
                        .map(|(x, y)| input.get(*y as usize).unwrap().get(*x as usize).unwrap())
                        .collect::<String>()
                });
            if !part2 {
                sum += im.filter(|w| w == needle).count();
            } else {
                sum += im
                    .collect::<Vec<_>>()
                    .windows(4)
                    .filter(|v| v.iter().filter(|w| *w == needle).count() == 2)
                    .count();
            }
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let directions1: Vec<Vec<(isize, isize)>> = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
    .iter()
    .map(|(a, b)| (0..4).map(|s| (s * a, s * b)).collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let directions2: Vec<Vec<(isize, isize)>> = vec![
        vec![(-1, -1), (0, 0), (1, 1)],
        vec![(1, 1), (0, 0), (-1, -1)],
        vec![(1, -1), (0, 0), (-1, 1)],
        vec![(-1, 1), (0, 0), (1, -1)],
    ];

    let part1 = solve(directions1, "XMAS", &input, false);
    let part2 = solve(directions2, "MAS", &input, true);
    println!("part1: {}\npart2: {}", part1, part2);
}
