fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {}", run(&inp));
    println!("answer 2: {}", run2(&inp));
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
    let digits = lines
        .iter()
        .filter_map(|line| line.get(i).copied())
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>();
    (!digits.is_empty()).then(|| std::str::from_utf8(&digits).unwrap().parse().unwrap())
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
}
