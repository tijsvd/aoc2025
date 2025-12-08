use std::collections::{BinaryHeap, HashMap};

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {:?}", run(&inp, 1000));
    println!("answer 2: {:?}", run2(&inp));
}

type Coord = [u64; 3];

fn parse(inp: &str) -> impl Iterator<Item = Coord> {
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|part| part.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
}

fn distance(p1: Coord, p2: Coord) -> u64 {
    p1.into_iter()
        .zip(p2)
        .map(|(l, r)| (l as i64 - r as i64).pow(2) as u64)
        .sum()
}

fn find_connections(
    circuits: impl Iterator<Item = Coord> + Clone,
    n_conns: usize,
) -> Vec<(usize, usize)> {
    let mut numbered = circuits.enumerate();
    // heap contains the top n_conns distances and ids
    let mut heap = BinaryHeap::new();
    while let Some((left_id, left)) = numbered.next() {
        for (right_id, right) in numbered.clone() {
            let dist = distance(left, right);
            if heap.len() < n_conns || heap.peek().is_none_or(|&(max_d, _)| dist < max_d) {
                heap.push((dist, (left_id, right_id)));
                if heap.len() > n_conns {
                    heap.pop();
                }
            }
        }
    }
    // if we make all the connections, it shouldn't matter in which order
    heap.drain().map(|(_, ids)| ids).collect()
}

fn run(inp: &str, n_conns: usize) -> usize {
    let circuits = parse(inp).collect::<Vec<_>>();
    let conns = find_connections(circuits.iter().copied(), n_conns);
    let mut set_ids = (0..circuits.len()).collect::<Vec<_>>();
    for (left, right) in conns {
        let left_set = set_ids[left];
        let right_set = set_ids[right];
        // join right set to left set
        for s in &mut set_ids {
            if *s == right_set {
                *s = left_set;
            }
        }
    }
    let set_id_to_size = set_ids
        .into_iter()
        .fold(HashMap::<usize, usize>::new(), |mut h, sid| {
            *h.entry(sid).or_default() += 1;
            h
        });
    let mut set_sizes = set_id_to_size.into_values().collect::<Vec<_>>();
    set_sizes.sort_unstable();
    set_sizes.iter().copied().rev().take(3).product::<usize>()
}

fn run2(inp: &str) -> u64 {
    let circuits = parse(inp).collect::<Vec<_>>();
    let mut dists = circuits
        .iter()
        .enumerate()
        .flat_map(|(left_id, &left)| {
            circuits
                .iter()
                .enumerate()
                .skip(left_id + 1)
                .map(move |(right_id, &right)| (distance(left, right), left_id, right_id))
        })
        .collect::<Vec<_>>();
    dists.sort_unstable();
    let mut set_ids = (0..circuits.len()).collect::<Vec<_>>();
    for (_, left, right) in dists {
        let left_set = set_ids[left];
        let right_set = set_ids[right];
        let mut single_set = true;
        for s in &mut set_ids {
            if *s == right_set {
                *s = left_set;
            } else if *s != left_set {
                single_set = false;
            }
        }
        if single_set {
            return circuits[left][0] * circuits[right][0];
        }
    }
    unreachable!()
}

#[test]
fn example() {
    let inp = "
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    ";
    assert_eq!(run(inp, 10), 40);
    assert_eq!(run2(inp), 25272);
}
