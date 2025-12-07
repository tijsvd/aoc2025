use std::collections::BTreeMap;
use std::collections::HashSet;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {}", run(&inp));
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
                splits.entry(y).or_default().insert(x);
            }
        }
    }
    (start.unwrap(), splits)
}

fn run(inp: &str) -> usize {
    let ((start_y, start_x), splits) = parse(inp);
    let mut beams = std::iter::once(start_x).collect::<HashSet<_>>();
    let mut split_count = 0;
    for (&y, split_xs) in &splits {
        if y < start_y {
            continue;
        }
        let mut nw_beams = HashSet::new();
        for x in beams {
            if split_xs.contains(&x) {
                nw_beams.insert(x - 1);
                nw_beams.insert(x + 1);
                split_count += 1;
            } else {
                nw_beams.insert(x);
            }
        }
        beams = nw_beams;
    }
    split_count
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
    assert_eq!(run(inp), 21);
}
