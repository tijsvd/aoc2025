fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("solution 1: {} {}", run(&inp, 2), run(&inp, 12));
    println!("solution 2: {} {}", run_2(&inp, 2), run_2(&inp, 12));
    println!("solution 3: {} {}", run_3(&inp, 2), run_3(&inp, 12));
    println!("solution 4: {} {}", run_4::<2>(&inp), run_4::<12>(&inp));
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

// this is the same as solution 2, not relying on input being in memory at all
#[allow(unused)]
fn run_3(inp: &str, n_digits: usize) -> u64 {
    let inp = inp.as_bytes().iter().copied();
    let mut outcomes = vec![0u64; n_digits];
    let mut answer = 0;
    for c in inp {
        if c == b'\n' {
            answer += outcomes[n_digits - 1];
            outcomes.fill(0);
            continue;
        }
        if !c.is_ascii_digit() {
            continue;
        }
        let c = c - b'0';
        for i in (0..n_digits).rev() {
            let prev = if i == 0 { 0 } else { outcomes[i - 1] };
            let nw = prev * 10 + c as u64;
            let tgt = &mut outcomes[i];
            if nw > *tgt {
                *tgt = nw;
            }
        }
    }
    answer
}

// same solution ready for fpga (purely functional)
struct State<const N: usize> {
    outcomes: [u64; N],
    answer: u64,
    position: usize,
    done: bool,
}

fn initialize<const N: usize>() -> State<N> {
    State {
        outcomes: [0; N],
        answer: 0,
        position: 0,
        // if input is empty, we're done, but tick() is written defensively
        done: false,
    }
}

fn tick<const N: usize>(state: State<N>, inp: &str) -> State<N> {
    let inp = inp.as_bytes();
    let c = inp.get(state.position).copied().unwrap_or(0);
    let position = state.position + 1;
    let done = position >= inp.len();
    let is_nl = c == b'\n';
    let is_digit = c.is_ascii_digit();
    let c_val = if is_digit { (c - b'0') as u64 } else { 0 };
    let answer = if is_nl {
        state.answer + state.outcomes[N - 1]
    } else {
        state.answer
    };
    let outcomes = std::array::from_fn(|i| {
        if is_nl {
            0
        } else if is_digit {
            let prev = if i == 0 { 0 } else { state.outcomes[i - 1] };
            std::cmp::max(state.outcomes[i], prev * 10 + c_val)
        } else {
            state.outcomes[i]
        }
    });
    State {
        answer,
        outcomes,
        position,
        done,
    }
}

fn run_4<const N: usize>(inp: &str) -> u64 {
    let mut state = initialize::<N>();
    while !state.done {
        state = tick(state, inp);
    }
    state.answer
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

    assert_eq!(run_3(inp, 2), 357);
    assert_eq!(run_3(inp, 12), 3121910778619);

    assert_eq!(run_4::<2>(inp), 357);
    assert_eq!(run_4::<12>(inp), 3121910778619);
}
