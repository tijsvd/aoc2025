fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {}", run(&inp));
    println!("answer 2: {} ?= {}", run2(&inp), run3(&inp));
}

fn run(inp: &str) -> u64 {
    let lines = inp.split('\n').map(|s| s.trim()).filter(|s| !s.is_empty());
    let mut sums = vec![];
    let mut prods = vec![];
    for line in lines {
        if line.starts_with(['+', '*']) {
            let ops = line.split_whitespace();
            return ops
                .zip(sums)
                .zip(prods)
                .map(|((op, sum), prod)| match op {
                    "+" => sum,
                    "*" => prod,
                    other => panic!("unknown op {other}"),
                })
                .sum();
        }
        let vals = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());
        if sums.is_empty() {
            sums = vals.collect();
            prods = sums.clone();
        } else {
            for ((val, sum), prod) in vals.zip(&mut sums).zip(&mut prods) {
                *sum += val;
                *prod *= val;
            }
        }
    }
    panic!("bad input")
}

fn get_num(lines: &[&[u8]], i: usize) -> Option<u64> {
    lines
        .iter()
        .filter_map(|line| line.get(i).copied())
        .filter(|c| c.is_ascii_digit())
        .fold(None, |acc, c| {
            Some(acc.unwrap_or(0) * 10 + (c - b'0') as u64)
        })
}

fn run2(inp: &str) -> u64 {
    let lines = inp
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>();
    let (ops, lines) = lines.split_last().unwrap();
    let mut total = 0;
    let mut i = 0;
    loop {
        let Some(&op) = ops.get(i) else {
            break;
        };
        if op == b' ' {
            i += 1;
            continue;
        }
        assert!([b'+', b'*'].contains(&op));
        let mut val = get_num(lines, i).unwrap();
        i += 1;
        while let Some(num) = get_num(lines, i) {
            if op == b'+' {
                val += num;
            } else {
                val *= num;
            }
            i += 1;
        }
        total += val;
    }
    total
}

// more text-oriented (slower) solution, transpose the entire input into
// strings where each string is a column
fn run3(inp: &str) -> u64 {
    let lines = inp
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>();
    let n_cols = lines.iter().map(|line| line.len()).max().unwrap();
    let mut transposed = (0..n_cols)
        .map(|i| {
            lines
                .iter()
                .map(|line| line.get(i).copied().unwrap_or(b' '))
                .collect::<Vec<_>>()
        })
        .map(|v| std::str::from_utf8(&v).unwrap().trim().to_string());
    let mut total = 0;
    loop {
        let Some(line) = transposed.next() else {
            break;
        };
        if line.is_empty() {
            continue;
        }
        let (line, op) = line.split_at(line.len() - 1);
        let is_add = match op {
            "+" => true,
            "*" => false,
            other => panic!("invalid op {other}"),
        };
        let mut val: u64 = line.trim().parse().unwrap();
        for line in transposed.by_ref() {
            if line.is_empty() {
                break;
            }
            let num: u64 = line.parse().unwrap();
            if is_add {
                val += num;
            } else {
                val *= num;
            }
        }
        total += val;
    }
    total
}

#[test]
fn example() {
    let inp = "
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +  
    ";
    assert_eq!(run(inp), 4277556);
    assert_eq!(run2(inp), 3263827);
    assert_eq!(run3(inp), 3263827);
}
