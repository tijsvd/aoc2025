use regex::Regex;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    assert!(
        parse(&inp)
            .all(|machine| machine.joltage.len() < 16
                && machine.joltage.into_iter().all(|j| j < 256))
    );
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
    let buttons = machine
        .buttons
        .iter()
        .map(|b| b.iter().fold(0u64, |a, i| a + (1 << i)))
        .collect::<Vec<_>>();
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

fn leading_zeroes(line: &[i32]) -> usize {
    line.iter().take_while(|&&i| i == 0).count()
}

fn reduce(matrix: &mut Vec<Vec<i32>>) {
    // https://en.wikipedia.org/wiki/Gaussian_elimination

    loop {
        let mut changed = false;

        // more leading zeroes on top
        matrix.sort_unstable_by_key(|line| std::cmp::Reverse(leading_zeroes(line)));
        for i in 0..matrix.len() - 1 {
            let (row1, rest) = matrix[i..].split_first_mut().unwrap();
            let row2 = &rest[0];
            let lz1 = leading_zeroes(row1);
            let lz2 = leading_zeroes(row2);
            if lz1 > lz2 || lz1 == row1.len() {
                continue;
            }
            debug_assert_eq!(lz1, lz2);
            let ndx = lz1;
            let c1 = row1[ndx];
            let c2 = row2[ndx];
            for (v1, &v2) in std::iter::zip(row1, row2) {
                *v1 = (*v1 * c2) - (v2 * c1);
            }
            debug_assert_eq!(matrix[i][ndx], 0);
            changed = true;
        }
        if !changed {
            break;
        }
    }
    matrix.retain(|row| row.iter().any(|&v| v != 0));
}

fn solve_subst_matrix(matrix: &[Vec<i32>]) -> Result<u32, ()> {
    // println!("solving {matrix:?}");
    let nvars = matrix.len();
    assert_eq!(matrix[0].len(), nvars + 1);
    let mut vars = vec![0u32; nvars];
    for (i, row) in matrix.iter().enumerate() {
        let ndx = nvars - i - 1;
        assert_eq!(leading_zeroes(row), ndx);
        let n = row[nvars]
            - (ndx + 1..nvars)
                .map(|j| vars[j] as i32 * row[j])
                .sum::<i32>();
        let d = row[ndx];
        if d < 0 && n > 0 || d > 0 && n < 0 {
            // println!("ndx={ndx} d = {d} n={n} bad");
            return Err(());
        }
        let n = n.unsigned_abs();
        let d = d.unsigned_abs();
        if !n.is_multiple_of(d) {
            // println!("ndx={ndx} n={n} d={d} not integer");
            return Err(());
        }
        // println!(" ndx={ndx} n={n} d={d} v={}", n/d);
        vars[ndx] = n / d;
    }
    // println!("done vars={vars:?}");
    let cost = vars.iter().copied().sum();
    Ok(cost)
}

fn search_matrix(matrix: &[Vec<i32>], max_p: u32, _depth: usize) -> Option<u32> {
    assert!(_depth <= matrix[0].len());
    let mut matrix = matrix.to_vec();
    //println!("search d={_depth} (original): {matrix:?}");
    reduce(&mut matrix);
    // println!("search d={_depth} (reduced): {matrix:?}");
    if matrix
        .iter()
        .any(|row| leading_zeroes(row) == row.len() - 1)
    {
        // println!("  -> invalid");
        // 0 = x
        return None;
    }
    if matrix.len() == matrix[0].len() - 1 {
        // println!("  -> solvable");
        // fully solveable
        return solve_subst_matrix(&matrix).ok();
    }
    // ok we'll have to do some actual searching
    let (_i, row) = matrix
        .iter()
        .enumerate()
        .find(|&(i, row)| leading_zeroes(row) < row.len() - i - 2)
        .unwrap();
    let var_ndx = leading_zeroes(row) + 1;
    // FIXME can we determine min/max from the matrix? I think we can, but let's not for now
    let mut extra_row = vec![0; matrix[0].len()];
    extra_row[var_ndx] = 1i32;
    matrix.push(extra_row);
    let mut ans = None;
    // println!("  -> search ndx={var_ndx}");
    for v in 0..max_p {
        *matrix.last_mut().unwrap().last_mut().unwrap() = v as i32;
        if let Some(p) = search_matrix(&matrix, max_p, _depth + 1)
            && ans.is_none_or(|m| p < m)
        {
            ans = Some(p);
        }
    }
    ans
}

fn run2(inp: &str) -> u32 {
    parse(inp)
        .map(|machine| {
            println!("searching {machine:?}");
            let matrix: Vec<Vec<i32>> = machine
                .joltage
                .iter()
                .enumerate()
                .map(|(i, &jlt)| {
                    machine
                        .buttons
                        .iter()
                        .map(|b| if b.contains(&i) { 1 } else { 0 })
                        .chain(Some(jlt as i32))
                        .collect()
                })
                .collect();
            let max_p = machine.joltage.iter().copied().max().unwrap();
            search_matrix(&matrix, max_p, 0).unwrap()
            // shortest_path(&machine.buttons, &machine.joltage, &get_related_buttons(&machine), &mut Default::default(), &mut Default::default()).unwrap()
            // min_presses(&machine)
        })
        .sum()
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
