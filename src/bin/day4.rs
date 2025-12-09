use std::collections::HashSet;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {}", run(&inp));
    println!("answer 2: {}", run2(&inp));
}

fn parse(inp: &str) -> HashSet<(i32, i32)> {
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '@')
                .map(move |(x, _)| (y as i32, x as i32))
        })
        .collect()
}

fn removable(positions: &HashSet<(i32, i32)>) -> impl Iterator<Item = (i32, i32)> {
    positions.iter().copied().filter(|&(y, x)| {
        (-1..=1)
            .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
            .filter(|&(dy, dx)| dy != 0 || dx != 0)
            .filter(|&(dy, dx)| positions.contains(&(y + dy, x + dx)))
            .count()
            < 4
    })
}

fn run(inp: &str) -> usize {
    let positions = parse(inp);
    removable(&positions).count()
}

fn run2(inp: &str) -> usize {
    let mut positions = parse(inp);
    let mut removed = 0;
    loop {
        let rm = removable(&positions).collect::<Vec<_>>();
        if rm.is_empty() {
            break;
        }
        removed += rm.len();
        for pos in rm {
            positions.remove(&pos);
        }
    }
    removed
}

#[test]
fn example() {
    let inp = "
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    ";
    assert_eq!(run(inp), 13);
    assert_eq!(run2(inp), 43);
}
