fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {}", run(&inp));
}

fn parse(inp: &str) -> (Vec<std::ops::RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges = vec![];
    let mut lines = inp.split('\n').map(|line| line.trim());
    for line in lines.by_ref() {
        let line = line.trim();
        if line.is_empty() {
            if ranges.is_empty() {
                continue;
            } else {
                break;
            }
        }
        let (start, end) = line.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        ranges.push(start..=end);
    }
    let ids = lines
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    (ranges, ids)
}

fn run(inp: &str) -> usize {
    let (ranges, ids) = parse(inp);
    ids.iter()
        .filter(|&&id| ranges.iter().any(|range| range.contains(&id)))
        .count()
}

#[test]
fn example() {
    let inp = "
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    ";
    assert_eq!(run(inp), 3);
}
