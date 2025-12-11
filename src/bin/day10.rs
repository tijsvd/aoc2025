use regex::Regex;
use std::cmp::Ordering;
use arrayvec::ArrayVec;
use std::collections::HashMap;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    assert!(parse(&inp)
        .all(|machine| machine.joltage.len() < 16 && machine.joltage.into_iter().all(|j| j < 256)));
    println!("answer: {:?}", run(&inp));
    println!("answer 2: {:?}", run2(&inp));
}

#[derive(Debug)]
struct Machine {
    desired: u64,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
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
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect()
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
    let buttons = machine.buttons.iter().map(|b| {
        b.iter().fold(0u64, |a, i| a + (1 << i))
    }).collect::<Vec<_>>();
    (0..(1u32 << buttons.len() as u32))
        .filter(|p| {
            let result = buttons
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

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

type Jolts = ArrayVec<u32, 25>;

fn shortest_path(
    buttons: &[Vec<usize>],
    jolts: &[u32],
    related_buttons: &[Vec<usize>],
    cache: &mut HashMap<Jolts, Option<u32>>,
    stats: &mut (usize, usize, usize),
) -> Option<u32> {
    // println!("search {jolts:?}");
    if jolts.iter().all(|&j| j == 0) {
        stats.2 += 1;
        return Some(0);
    }
    let d = jolts.iter().fold(0, |g, &j| gcd(g, j));
    let mut jolts = jolts.iter().map(|&j| j / d).collect::<Jolts>();
    // println!(" d={d} upd={jolts:?}");
    if let Some(&ans) = cache.get(&jolts) {
        // println!(" cached {ans:?}");
        stats.0 += 1;
        return ans.map(|c| c * d);
    }
    stats.1 += 1;

    let mut ranges = std::iter::repeat_n((0u32, u32::MAX), buttons.len()).collect::<Ranges>();
    optimize_ranges(&jolts, related_buttons, &mut ranges);
    // println!("optimized ranges: {ranges:?}");

    let mut cost = None;
    for (button, &(min, max)) in buttons.iter().zip(&ranges) {
        if min == 0 && max == 0 {
            continue;
        }
        let n = min.max(1);
        if button.iter().any(|&ndx| jolts[ndx] < n) {
            continue;
        }
        for &ndx in button {
            jolts[ndx] -= n;
        }
        if let Some(p) = shortest_path(buttons, &jolts, related_buttons, cache, stats) && cost.is_none_or(|c| p + n < c) {
            cost = Some(p + n);
        }
        for &ndx in button {
            jolts[ndx] += n;
        }
    }
    // println!(" d={d} tgt={jolts:?} answ={cost:?}");
    if cache.len().is_multiple_of(1000000) {
        let (hits, misses, sols) = *stats;
        println!("cache {} @ {jolts:?} hits={hits} misses={misses} sols={sols}", cache.len());
    }
    cache.insert(jolts, cost);
    cost.map(|c| c * d)
}

type Ranges = ArrayVec<(u32, u32), 25>;

fn get_related_buttons(machine: &Machine) -> Vec<Vec<usize>> {
    (0..machine.joltage.len())
        .map(|i| {
            machine
                .buttons
                .iter()
                .enumerate()
                .filter(|(_, b)| b.contains(&i))
                .map(|(j, _)| j)
                .collect()
        })
        .collect()
}

fn min_presses(machine: &Machine) -> u32 {
    println!("searching {machine:?}");
    let related_buttons = get_related_buttons(machine);
    // let's optimise the search range -- e.g. if some joltage is affected only by one button,
    // then we already know the exact number of presses, and doing this recursively can severely
    // limit the search.
    let mut button_ranges = std::iter::repeat_n((0u32, u32::MAX), machine.buttons.len()).collect::<Ranges>();
    optimize_ranges(&machine.joltage, &related_buttons, &mut button_ranges);
    println!("ranges optimised: {button_ranges:?}");
    let mut presses = vec![0; button_ranges.len()];
    let mut min_presses = u32::MAX;
    search(
        &machine.joltage, &related_buttons,
        &button_ranges,
        &mut presses,
        0,
        &mut min_presses,
    );
    assert!(min_presses < u32::MAX);
    min_presses
}

fn optimize_ranges(
    joltage: &[u32],
    related_buttons: &[Vec<usize>],
    button_ranges: &mut [(u32, u32)],
) {
    loop {
        let mut updated = false;
        // println!("optimizing {button_ranges:?} {joltage_affected:?}");
        for (&tgt_j, buttons) in std::iter::zip(joltage, related_buttons) {
            for &b in buttons {
                let (current_min, current_max) = button_ranges[b];
                if current_min == current_max {
                    continue;
                }
                let (min_ex, max_ex) = buttons
                    .iter()
                    .copied()
                    .filter(|&other_button| other_button != b)
                    .map(|ndx| button_ranges[ndx])
                    .fold((0u32, 0u32), |(min, max), (bmin, bmax)| {
                        (min + bmin, max.saturating_add(bmax))
                    });
                let (current_min, current_max) = button_ranges[b];
                // println!(" tgt={tgt_j} button {b} ex=({min_ex}, {max_ex}) current=({current_min}, {current_max})");
                // if the other buttons add up to at least x, then we can't press this button more than
                // tgt_j - x
                // and if this_max is now < 0, it means that no solution is possible
                let this_max = tgt_j.saturating_sub(min_ex);
                if this_max < current_max {
                    button_ranges[b].1 = this_max;
                    // println!(" changed max to {this_max} {button_ranges:?}");
                    updated = true;
                }
                // if the other buttons add up to at most x, then we must press this button at least tgt_j - x
                let this_min = tgt_j.saturating_sub(max_ex);
                if this_min > current_min {
                    button_ranges[b].0 = this_min;
                    // println!(" changed min to {this_min} {button_ranges:?}");
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }
}

fn search(
    joltage: &[u32],
    related_buttons: &[Vec<usize>],
    ranges: &Ranges,
    presses: &mut [u32],
    i: usize,
    min: &mut u32,
) {
    if i >= presses.len() {
        return;
    }
    /*
    if i < 2 {
        println!("searching i={i} ranges={ranges:?} {presses:?}");
    }
    */
    let (min_p, max_p) = ranges[i];
    for c in min_p..=max_p {
        // println!("try i={i} c={c} {ranges:?} {presses:?}");
        presses[i] = c;
        let total = presses.iter().copied().sum();
        if total >= *min {
            break;
        }
        match presses_result(joltage, related_buttons, presses) {
            Ordering::Greater => break,
            Ordering::Less => {
                let mut ranges = ranges.clone();
                ranges[i] = (c, c);
                optimize_ranges(joltage, related_buttons, &mut ranges);
                search(joltage, related_buttons, &ranges, presses, i + 1, min);
            }
            Ordering::Equal => {
                println!("solved {presses:?}");
                *min = total;
                break;
            }
        }
    }
    presses[i] = 0;
    // println!("done i={i} {ranges:?} {presses:?}");
}

fn presses_result(joltage: &[u32], related_buttons: &[Vec<usize>], presses: &[u32]) -> Ordering {
    let mut all_eq = true;
    for (&tgt_j, buttons) in std::iter::zip(joltage, related_buttons) {
        let sum: u32 = buttons.iter().map(|&b| presses[b]).sum();
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

fn run2(inp: &str) -> u32 {
    parse(inp).map(|machine| {
        println!("searching {machine:?}");
        shortest_path(&machine.buttons, &machine.joltage, &get_related_buttons(&machine), &mut Default::default(), &mut Default::default()).unwrap()
        // min_presses(&machine)
    }).sum()
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
