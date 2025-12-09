fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {}", run(&inp));
    println!("answer 2: {}", run2(&inp));
}

fn parse(inp: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    inp.trim().split(',').map(|s| {
        let (first, last) = s.split_once('-').unwrap();
        (first.parse().unwrap(), last.parse().unwrap())
    })
}

fn is_valid_id(x: u64) -> bool {
    let s = format!("{x}");
    let s = s.as_bytes();
    let l = s.len();
    if !l.is_multiple_of(2) {
        return true;
    }
    let m = l / 2;
    s[0..m] != s[m..l]
}

fn run(inp: &str) -> u64 {
    parse(inp)
        .flat_map(|(first, last)| first..=last)
        .filter(|&id| !is_valid_id(id))
        .sum()
}

fn is_valid_id_2(x: u64) -> bool {
    let s = format!("{x}");
    let s = s.as_bytes();
    let l = s.len();
    'outer: for m in 1..l {
        if !l.is_multiple_of(m) {
            continue;
        }
        let c1 = &s[..m];
        for c in s.chunks(m).skip(1) {
            if c != c1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

#[test]
fn test_valid() {
    assert!(!is_valid_id_2(12341234));
    assert!(!is_valid_id_2(123123123));
    assert!(!is_valid_id_2(1212121212));
    assert!(!is_valid_id_2(1111111));

    assert!(is_valid_id_2(5));
    assert!(is_valid_id_2(57));
    assert!(is_valid_id_2(525));
}

fn run2(inp: &str) -> u64 {
    parse(inp)
        .flat_map(|(first, last)| first..=last)
        .filter(|&id| !is_valid_id_2(id))
        .sum()
}

#[test]
fn example() {
    let inp = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(run(inp), 1227775554);
    assert_eq!(run2(inp), 4174379265);
}
