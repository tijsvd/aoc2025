fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {}", run(&inp));
    println!("answer 2: {}", run2(&inp));
}

fn parse(inp: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
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
        ranges.push((start, end + 1));
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
        .filter(|&&id| {
            ranges
                .iter()
                .any(|&(start, end)| (start..end).contains(&id))
        })
        .count()
}

fn run2(inp: &str) -> u64 {
    let (mut ranges, _) = parse(inp);
    ranges.sort_unstable();
    let mut i = 0;
    while i < ranges.len() {
        let ((_, i_end), rest) = ranges[i..].split_first_mut().unwrap();
        let n_rm = rest
            .iter()
            .position(|&(j_start, j_end)| {
                if j_start > *i_end {
                    return true;
                }
                *i_end = (*i_end).max(j_end);
                false
            })
            .unwrap_or(rest.len());
        ranges.drain(i + 1..i + 1 + n_rm);
        i += 1;
    }
    ranges.into_iter().map(|(start, end)| end - start).sum()
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
    assert_eq!(run2(inp), 14);
}
