use std::collections::HashSet;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer : {}", run(&inp));
}

fn run(inp: &str) -> usize {
    let positions = inp
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '@')
                .map(move |(x, _)| (y as i32, x as i32))
        })
        .collect::<HashSet<_>>();
    positions
        .iter()
        .filter(|&(y, x)| {
            (-1..=1)
                .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
                .filter(|&(dy, dx)| dy != 0 || dx != 0)
                .filter(|&(dy, dx)| positions.contains(&(y + dy, x + dx)))
                .count()
                < 4
        })
        .count()
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
}
