use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {:?}", run(&inp));
}

type Splits = BTreeMap<usize, HashSet<usize>>;

fn parse(inp: &str) -> ((usize, usize), Splits) {
    let mut start = None;
    let mut splits = Splits::new();
    for (y, line) in inp
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .enumerate()
    {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            if c == b'S' {
                start = Some((y, x));
            } else if c == b'^' {
                assert!(x > 0, "can't have split at edge");
                splits.entry(y).or_default().insert(x);
            }
        }
    }
    (start.unwrap(), splits)
}

fn run(inp: &str) -> (usize, usize) {
    let ((start_y, start_x), splits) = parse(inp);
    let mut beams = std::iter::once((start_x, 1)).collect::<HashMap<_, _>>();
    let mut split_count = 0;
    for (&y, split_xs) in &splits {
        if y < start_y {
            continue;
        }
        beams = beams.into_iter().fold(HashMap::new(), |mut m, (x, cnt)| {
            if split_xs.contains(&x) {
                *m.entry(x - 1).or_default() += cnt;
                *m.entry(x + 1).or_default() += cnt;
                split_count += 1;
            } else {
                *m.entry(x).or_default() += cnt;
            }
            m
        });
    }
    let n_beams = beams.into_values().sum();
    (split_count, n_beams)
}

#[test]
fn example() {
    let inp = "
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    ";
    assert_eq!(run(inp), (21, 40));
}
