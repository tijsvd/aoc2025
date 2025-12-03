fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {} {}", run(&inp, 2), run(&inp, 12));
    println!("(alternative): {} {}", run_2(&inp, 2), run_2(&inp, 12));
}

// solution 1 -- figure out the best next digit, one by one
fn run(inp: &str, n_digits: usize) -> u64 {
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let bank = s.as_bytes();
            let mut position = 0;
            let mut outcome = 0;
            for n in 1..=n_digits {
                let remain = n_digits - n;
                let (p, c) = bank
                    .iter()
                    .copied()
                    .enumerate()
                    .take(bank.len() - remain)
                    .skip(position)
                    .max_by_key(|&(i, c)| (c, std::cmp::Reverse(i)))
                    .unwrap();
                outcome = outcome * 10 + (c - b'0') as u64;
                position = p + 1;
            }
            outcome
        })
        .sum()
}

// solution 2 -- run through the "bank" once, keeping track of
// the best outcome for all possible lengths so far
fn run_2(inp: &str, n_digits: usize) -> u64 {
    let mut outcomes = vec![0u64; n_digits + 1];
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            outcomes.fill(0);
            for &c in s.as_bytes() {
                let c = c - b'0';
                // reverse otherwise we'll read back newly calculated values
                for i in (1..=n_digits).rev() {
                    // a state of len i may be formed by appending to state of len (i-1)
                    let nw = outcomes[i - 1] * 10 + c as u64;
                    // replace if better than known
                    let tgt = &mut outcomes[i];
                    if nw > *tgt {
                        *tgt = nw;
                    }
                }
            }
            outcomes[n_digits]
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
