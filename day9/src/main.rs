fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {:?}", run(&inp));
}

fn parse(inp: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
}

fn run(inp: &str) -> u64 {
    let squares = parse(inp).collect::<Vec<_>>();
    squares
        .iter()
        .enumerate()
        .flat_map(|(i, &sq1)| squares.iter().skip(i + 1).map(move |&sq2| (sq1, sq2)))
        .map(|((x1, y1), (x2, y2))| {
            let dx = x1.abs_diff(x2) + 1;
            let dy = y1.abs_diff(y2) + 1;
            dx * dy
        })
        .max()
        .unwrap()
}

#[test]
fn example() {
    let inp = "
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    ";
    assert_eq!(run(inp), 50);
}
