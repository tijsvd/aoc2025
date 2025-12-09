fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {}", run(&inp));
    println!("answer 2: {}", run2(&inp));
}

fn parse(inp: &str) -> impl Iterator<Item = i32> + '_ {
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (dir, cnt) = s.split_at(1);
            let cnt: i32 = cnt.parse().unwrap();
            match dir {
                "L" => -cnt,
                "R" => cnt,
                other => panic!("unexpected: {other}"),
            }
        })
}

fn run(inp: &str) -> usize {
    let mut dial = 50;
    let mut ans = 0;
    for cnt in parse(inp) {
        dial = (dial + cnt).rem_euclid(100);
        if dial == 0 {
            ans += 1;
        }
    }
    ans
}

fn run2(inp: &str) -> usize {
    let mut dial = 50;
    let mut ans = 0;
    for cnt in parse(inp) {
        if cnt == 0 {
            continue;
        }
        if cnt > 0 {
            ans += (dial + cnt) as usize / 100;
        } else if dial == 0 {
            ans += -cnt as usize / 100;
        } else {
            ans += (100 - dial - cnt) as usize / 100;
        }
        dial = (dial + cnt).rem_euclid(100);
    }
    ans
}

#[test]
fn example() {
    let inp = "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    ";
    assert_eq!(run(inp), 3);
    assert_eq!(run2(inp), 6);
}
