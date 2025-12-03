fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {}", run(&inp, 2));
    println!("answer 2: {}", run(&inp, 12));
}

fn parse(inp: &str) -> impl Iterator<Item = Vec<u8>> + '_ {
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.as_bytes().iter().map(|&c| c - b'0').collect())
}

fn max_j(bank: &[u8], n_digits: usize) -> u64 {
    let mut start = 0;
    let mut outcome = 0;
    for n in 1..=n_digits {
        let remain = n_digits - n;
        let (p, c) = bank
            .iter()
            .copied()
            .enumerate()
            .take(bank.len() - remain)
            .skip(start)
            .max_by_key(|&(i, c)| (c, std::cmp::Reverse(i)))
            .unwrap();
        outcome = outcome * 10 + c as u64;
        start = p + 1;
    }
    outcome
}

fn run(inp: &str, n_digits: usize) -> u64 {
    parse(inp).map(|bank| max_j(&bank, n_digits)).sum()
}

#[test]
fn example() {
    let inp = "
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ";
    assert_eq!(run(inp, 2), 357);
    assert_eq!(run(inp, 12), 3121910778619);
}
