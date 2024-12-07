use std::fs;

const INPUT_FILE: &str = "input";

fn main() {
    let mut reports: Vec<_> = fs::read_to_string(INPUT_FILE).expect("could not read input file!")
        .lines().map(|line| { line.split_whitespace().map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>() }).collect();

    let f = |r:&mut Vec<_> | { if r.first() > r.last() { r.reverse() }
        r.iter().fold(Some(r[0] - 1), |acc, n| { if (n - acc? >= 1) && (n - acc? <= 3) { Some(*n) } else { None} }).is_some()};

    let part1 = reports.iter_mut().map(f).filter(|x| *x).count();
    let part2 = reports.iter_mut().map(|r| { if r.first() > r.last() { r.reverse() }
         let (_,i,d) = r.iter().fold((r[0] - 1,0,true), |(a,i,t), n| { if (n - a >= 1) && (n - a <= 3) && t {(*n,i+1,t) } else {(i, i,false)} });
           let i = i as usize; if d {true} else { let s = r.remove(i); if !f(r) {r.insert(i,s);r.remove(i-1); f(r)} else {true} } })
        .filter(|x| *x).count();

    println!("part1: {}\npart2: {}", part1, part2);
}
