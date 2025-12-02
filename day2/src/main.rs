fn main() {
    let inp = std::fs::read_to_string("input.txt").unwrap();
    println!("anser: {}", run(&inp));
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

#[test]
fn example() {
    let inp = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(run(inp), 1227775554);
}
