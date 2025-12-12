use std::collections::{HashMap, HashSet};

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {:?}", run(&inp));
}

type Shape = [u8; 3];

#[derive(Debug, Clone)]
struct Puzzle {
    wx: u32,
    wy: usize,
    presents: Vec<usize>,
}

fn parse(inp: &str) -> (Vec<Shape>, impl Iterator<Item = Puzzle>) {
    let mut lines = inp
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .peekable();
    let mut shapes = vec![];
    while let Some(line) = lines.next_if(|line| !line.contains('x')) {
        let ndx: usize = line.strip_suffix(":").unwrap().parse().unwrap();
        assert_eq!(ndx, shapes.len());
        let shape = std::array::from_fn(|_| {
            let line = lines.next().unwrap();
            assert_eq!(line.len(), 3);
            line.as_bytes()
                .iter()
                .fold(0, |a, &c| (a << 1) | (if c == b'#' { 1 } else { 0 }))
        });
        shapes.push(shape);
    }
    let puzzles = lines.map(|line| {
        let (shape, indices) = line.split_once(':').unwrap();
        let (wx, wy) = shape.split_once('x').unwrap();
        let wx: u32 = wx.parse().unwrap();
        let wy: usize = wy.parse().unwrap();
        assert!(wx <= 64);
        let presents = indices
            .split_whitespace()
            .enumerate()
            .flat_map(|(index, count)| {
                let count: usize = count.parse().unwrap();
                std::iter::repeat_n(index, count)
            })
            .collect();
        Puzzle { wx, wy, presents }
    });
    (shapes, puzzles)
}

fn run(inp: &str) -> usize {
    let (shapes, puzzles) = parse(inp);
    puzzles.filter(|p| can_solve(&shapes, p)).count()
}

fn rot1(shape: Shape) -> Shape {
    //  1 2 3
    //  4 5 6
    //  7 8 9
    //  ->
    //  7 4 1
    //  8 5 2
    //  9 6 3
    fn d(l: u8) -> [u8; 3] {
        std::array::from_fn(|i| (l >> i) & 1)
    }
    fn c(b1: u8, b2: u8, b3: u8) -> u8 {
        (b3 << 2) | (b2 << 1) | b1
    }
    let [v1, v2, v3] = d(shape[0]);
    let [v4, v5, v6] = d(shape[1]);
    let [v7, v8, v9] = d(shape[2]);
    [c(v7, v4, v1), c(v8, v5, v2), c(v9, v6, v3)]
}

fn flip([l1, l2, l3]: Shape) -> Shape {
    [l3, l2, l1]
}

fn transforms(mut shape: Shape) -> impl Iterator<Item = Shape> {
    // 8 options: 4 rots, and their flips
    // (flip_v is the same as rot2 + flip_h)
    std::iter::repeat_n((), 4).flat_map(move |_| {
        shape = rot1(shape);
        [shape, flip(shape)]
    })
}

fn can_solve(shapes: &[Shape], puzzle: &Puzzle) -> bool {
    println!("solving {puzzle:?}");
    if puzzle.presents.len() <= (puzzle.wx as usize / 3 * puzzle.wy / 3) {
        return true;
    }
    if puzzle
        .presents
        .iter()
        .map(|&p| shapes[p].into_iter().map(|l| l.count_ones()).sum::<u32>() as usize)
        .sum::<usize>()
        > puzzle.wx as usize * puzzle.wy
    {
        return false;
    }
    let mut grid = vec![0; puzzle.wy];
    search_placement(
        &mut grid,
        puzzle.wx,
        shapes,
        &puzzle.presents,
        &mut Default::default(),
    )
}

fn search_placement(
    grid: &mut [u64],
    wx: u32,
    shapes: &[Shape],
    presents: &[usize],
    cache: &mut HashMap<usize, HashSet<Vec<u64>>>,
) -> bool {
    // println!("search {grid:?} {presents:?}");
    let Some((&present, rest)) = presents.split_first() else {
        return true;
    };
    if cache.get(&presents.len()).is_some_and(|s| s.contains(grid)) {
        return false;
    }
    let shape = shapes[present];
    let wy = grid.len();
    for shape in transforms(shape) {
        for x in 0..wx - 2 {
            let mask: [u64; 3] = std::array::from_fn(|i| (shape[i] as u64) << x);
            for y in 0..wy - 2 {
                if std::iter::zip(&grid[y..y + 3], mask).all(|(line, m)| line & m == 0) {
                    for (line, m) in std::iter::zip(&mut grid[y..y + 3], mask) {
                        debug_assert!(*line & m == 0);
                        *line |= m;
                    }
                    if search_placement(grid, wx, shapes, rest, cache) {
                        return true;
                    }
                    for (line, m) in std::iter::zip(&mut grid[y..y + 3], mask) {
                        debug_assert!(*line & m == m);
                        *line ^= m;
                    }
                }
            }
        }
    }
    cache
        .entry(presents.len())
        .or_default()
        .insert(grid.to_vec());
    false
}

#[test]
fn example() {
    let inp = "
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
    ";
    assert_eq!(run(inp), 2);
}
