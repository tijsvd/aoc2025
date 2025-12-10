use regex::Regex;
use std::cmp::Ordering;

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

fn min_presses(machine: &Machine) -> u64 {
    println!("searching {machine:?}");
    let mut joltage_affected = machine
        .joltage
        .iter()
        .enumerate()
        .map(|(i, &j)| {
            let mask = 1 << i;
            let buttons = machine
                .buttons
                .iter()
                .enumerate()
                .filter(|&(_, &b)| b & mask != 0)
                .map(|(j, _)| j)
                .collect::<Vec<_>>();
            (j, buttons)
        })
        .collect::<Vec<_>>();
    // it feels prudent to start searching with the joltage that has the lowest
    // number of button combinations
    joltage_affected.sort_unstable_by_key(|(_, buttons)| buttons.len());
    // let's optimise the search range -- e.g. if some joltage is affected only by one button,
    // then we already know the exact number of presses, and doing this recursively can severely
    // limit the search.
    let mut button_ranges = vec![(0u64, u64::MAX); machine.buttons.len()]; // inclusive
    loop {
        let mut updated = false;
        for &(tgt_j, ref buttons) in &joltage_affected {
            for &b in buttons {
                let (min_ex, max_ex) = buttons
                    .iter()
                    .copied()
                    .filter(|&other_button| other_button != b)
                    .map(|ndx| button_ranges[ndx])
                    .fold((0u64, 0u64), |(min, max), (bmin, bmax)| {
                        (min + bmin, max.saturating_add(bmax))
                    });
                let (current_min, current_max) = button_ranges[b];
                // if the other buttons add up to at least x, then we can't press this button more than
                // tgt_j - x
                let this_max = tgt_j.checked_sub(min_ex).unwrap();
                if this_max < current_max {
                    button_ranges[b].1 = this_max;
                    updated = true;
                }
                // if the other buttons add up to at most x, then we must press this button at least tgt_j - x
                let this_min = tgt_j.saturating_sub(max_ex);
                if this_min > current_min {
                    button_ranges[b].0 = this_min;
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }
    println!("ranges optimised: {button_ranges:?}");
    let mut presses = button_ranges
        .iter()
        .map(|&(min, _)| min)
        .collect::<Vec<_>>();
    let mut min_presses = u64::MAX;
    search(
        &joltage_affected,
        &button_ranges,
        &mut presses,
        0,
        &mut min_presses,
    );
    min_presses
}

fn search(
    joltage_affected: &[(u64, Vec<usize>)],
    ranges: &[(u64, u64)],
    presses: &mut [u64],
    i: usize,
    min: &mut u64,
) {
    if i >= presses.len() {
        return;
    }
    let (min_p, max_p) = ranges[i];
    debug_assert_eq!(presses[i], min_p);
    for c in min_p..=max_p {
        presses[i] = c;
        let total = presses.iter().copied().sum();
        if total >= *min {
            break;
        }
        match presses_result(joltage_affected, presses) {
            Ordering::Greater => break,
            Ordering::Less => search(joltage_affected, ranges, presses, i + 1, min),
            Ordering::Equal => {
                *min = total;
                break;
            }
        }
    }
    presses[i] = min_p;
}

fn presses_result(joltage_affected: &[(u64, Vec<usize>)], presses: &[u64]) -> Ordering {
    let mut all_eq = true;
    for &(tgt_j, ref buttons) in joltage_affected {
        let sum: u64 = buttons.iter().map(|&b| presses[b]).sum();
        if sum > tgt_j {
            return Ordering::Greater;
        }
        if sum != tgt_j {
            all_eq = false;
        }
    }
    if all_eq {
        Ordering::Equal
    } else {
        Ordering::Less
    }
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
