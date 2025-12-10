use regex::Regex;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {:?}", run(&inp));
    println!("answer 2: {:?}", run2(&inp));
}

#[derive(Debug)]
struct Machine {
    desired: u64,
    buttons: Vec<u64>,
    joltage: Vec<u64>,
}

fn parse(inp: &str) -> impl Iterator<Item = Machine> {
    let re_line = Regex::new(r"\[([#.]+)\](.*)\{(.*)\}").unwrap();
    let re_buttons = Regex::new(r"\(([\d,]+)\)").unwrap();
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(move |s| {
            let mo = re_line.captures(s).unwrap();
            let (_, [desired, buttons, joltage]) = mo.extract();
            let desired = desired
                .as_bytes()
                .iter()
                .enumerate()
                .fold(0u64, |a, (i, &c)| if c == b'#' { a + (1 << i) } else { a });
            let buttons = re_buttons
                .captures_iter(buttons)
                .map(|mo| {
                    mo.get(1)
                        .unwrap()
                        .as_str()
                        .split(',')
                        .map(|s| s.parse::<u64>().unwrap())
                        .fold(0u64, |a, i| a + (1 << i))
                })
                .collect();
            let joltage = joltage.split(',').map(|s| s.parse().unwrap()).collect();
            Machine {
                desired,
                buttons,
                joltage,
            }
        })
}

fn min_toggles(machine: &Machine) -> u32 {
    // println!("checking {machine:?}");
    // since it's binary toggles, it never makes sense to toggle a button twice.
    (0..(1u32 << machine.buttons.len() as u32))
        .filter(|p| {
            let result = machine
                .buttons
                .iter()
                .enumerate()
                .filter(|&(i, _)| p & (1 << i) != 0)
                .fold(0, |a, (_, &b)| a ^ b);
            // println!("combination {p:b} result={result}");
            result == machine.desired
        })
        .map(|p| p.count_ones())
        .min()
        .unwrap()
}

fn run(inp: &str) -> u32 {
    parse(inp).map(|machine| min_toggles(&machine)).sum()
}

use std::cmp::Ordering;

fn presses_check(counts: &[u64], machine: &Machine) -> Ordering {
    let mut all_eq = true;
    for (i, &tgt_j) in machine.joltage.iter().enumerate() {
        let mask = 1 << i;
        let j: u64 = std::iter::zip(counts, &machine.buttons)
            .filter(|&(_, b)| b & mask != 0)
            .map(|(c, _)| c)
            .sum();
        if j > tgt_j {
            return Ordering::Greater;
        }
        if j < tgt_j {
            all_eq = false;
        }
    }
    if all_eq {
        Ordering::Equal
    } else {
        Ordering::Less
    }
}

fn search_presses(machine: &Machine, counts: &mut [u64], i: usize, min: &mut u64) {
    if i >= counts.len() {
        return;
    }
    debug_assert_eq!(counts[i], 0);
    for c in 0.. {
        counts[i] = c;
        let presses = counts.iter().copied().sum();
        if presses >= *min {
            break;
        }
        match presses_check(counts, machine) {
            Ordering::Less => {
                search_presses(machine, counts, i + 1, min);
            }
            Ordering::Equal => {
                *min = presses;
            }
            Ordering::Greater => {
                break;
            }
        }
    }
    counts[i] = 0;
}

fn min_presses(machine: &Machine) -> u64 {
    // println!("searching {machine:?}");
    let mut counts = vec![0; machine.buttons.len()];
    let mut min = u64::MAX;
    search_presses(machine, &mut counts, 0, &mut min);
    min
}

fn run2(inp: &str) -> u64 {
    parse(inp).map(|machine| min_presses(&machine)).sum()
}

#[test]
fn example() {
    let inp = "
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ";
    assert_eq!(run(inp), 7);
    assert_eq!(run2(inp), 33);
}
