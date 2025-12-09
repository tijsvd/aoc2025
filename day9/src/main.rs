fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {:?}", run(&inp));
    println!("answer 2: {:?}", run2(&inp));
}

type Point = (u64, u64);

fn parse(inp: &str) -> impl Iterator<Item = Point> + '_ {
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
}

fn squares(corners: &[Point]) -> impl Iterator<Item = (Point, Point)> + '_ {
    corners
        .iter()
        .enumerate()
        .flat_map(move |(i, &sq1)| corners.iter().skip(i + 1).map(move |&sq2| (sq1, sq2)))
}

fn lines(corners: &[Point]) -> impl Iterator<Item = (Point, Point)> + '_ {
    corners
        .iter()
        .copied()
        .zip(corners.iter().copied().cycle().skip(1))
}

fn intersects(square: (Point, Point), line: (Point, Point)) -> bool {
    let ((x1, y1), (x2, y2)) = square;
    let left = std::cmp::min(x1, x2);
    let right = std::cmp::max(x1, x2);
    let top = std::cmp::min(y1, y2);
    let bottom = std::cmp::max(y1, y2);
    let ((x1, y1), (x2, y2)) = line;
    if x1 == x2 {
        if x1 <= left || x2 >= right {
            return false;
        }
        let line_top = std::cmp::min(y1, y2);
        let line_bottom = std::cmp::max(y1, y2);
        line_top < bottom && line_bottom > top
    } else {
        assert!(y1 == y2);
        if y1 <= top || y2 >= bottom {
            return false;
        }
        let line_left = std::cmp::min(x1, x2);
        let line_right = std::cmp::max(x1, x2);
        line_left < right && line_right > left
    }
}

fn area(square: (Point, Point)) -> u64 {
    let ((x1, y1), (x2, y2)) = square;
    let dx = x1.abs_diff(x2) + 1;
    let dy = y1.abs_diff(y2) + 1;
    dx * dy
}

fn run(inp: &str) -> u64 {
    let corners = parse(inp).collect::<Vec<_>>();
    squares(&corners).map(area).max().unwrap()
}

fn run2(inp: &str) -> u64 {
    let corners = parse(inp).collect::<Vec<_>>();
    squares(&corners)
        .filter(|&sq| !lines(&corners).any(|line| intersects(sq, line)))
        .map(area)
        .max()
        .unwrap()
}

#[test]
fn example() {
    let inp = "
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    ";
    assert_eq!(run(inp), 50);
    assert_eq!(run2(inp), 24);
}
