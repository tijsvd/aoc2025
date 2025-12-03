fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {} {}", run(&inp, 2), run(&inp, 12));
    println!("(alternative): {} {}", run_2(&inp, 2), run_2(&inp, 12));
}

fn parse(inp: &str) -> impl Iterator<Item = Vec<u8>> + '_ {
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.as_bytes().iter().map(|&c| c - b'0').collect())
}

// this is O(bank_len^2 * n_digits)
// (it happens to be fine for the given inputs)
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

fn run_2(inp: &str, n_digits: usize) -> u64 {
    // state[i] = n -> the maximum outcome so far for i digits is n
    let mut states = vec![0u64; n_digits + 1];
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            states.fill(0);
            for &c in s.as_bytes() {
                let c = c - b'0';
                // reverse otherwise we're reading back newly calculated values
                for i in (1..=n_digits).rev() {
                    // a state of len i may be formed by appending to state of len (i-1)
                    let nw = states[i - 1] * 10 + c as u64;
                    // replace if better than known
                    let tgt = &mut states[i];
                    if nw > *tgt {
                        *tgt = nw;
                    }
                }
            }
            states[n_digits]
        })
        .sum()
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

    assert_eq!(run_2(inp, 2), 357);
    assert_eq!(run_2(inp, 12), 3121910778619);
}
