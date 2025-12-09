use std::collections::HashMap;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {:?}", run(&inp));
}

fn parse(inp: &str) -> (usize, impl Iterator<Item = &[u8]> + '_) {
    let mut lines = inp
        .split('\n')
        .map(|s| s.trim().as_bytes())
        .filter(|s| !s.is_empty());
    let start = lines
        .by_ref()
        .find_map(|line| line.iter().position(|&c| c == b'S'))
        .unwrap();
    (start, lines)
}

fn run(inp: &str) -> (usize, usize) {
    let (start, lines) = parse(inp);
    let mut beams = std::iter::once((start, 1)).collect::<HashMap<_, _>>();
    let mut split_count = 0;
    for line in lines {
        beams = beams.into_iter().fold(HashMap::new(), |mut m, (x, cnt)| {
            if line[x] == b'^' {
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
