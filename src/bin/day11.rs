use std::collections::HashMap;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {:?}", run(&inp));
}

type Device = [u8; 3];
type Graph = HashMap<Device, Vec<Device>>;

fn str_to_dev(s: &str) -> Device {
    s.trim().as_bytes().try_into().unwrap()
}

const YOU: Device = [b'y', b'o', b'u'];
const OUT: Device = [b'o', b'u', b't'];

fn parse(inp: &str) -> Graph {
    inp.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (dev, rest) = s.split_once(':').unwrap();
            let dev = str_to_dev(dev);
            let outputs = rest.split_whitespace().map(str_to_dev).collect();
            (dev, outputs)
        })
        .collect()
}

fn run(inp: &str) -> usize {
    let graph = parse(inp);
    search_n_paths(&graph, YOU, &mut Default::default())
}

fn search_n_paths(graph: &Graph, dev: Device, cache: &mut HashMap<Device, usize>) -> usize {
    if dev == OUT {
        return 1;
    }
    if let Some(&ans) = cache.get(&dev) {
        return ans;
    }
    let ans = graph
        .get(&dev)
        .into_iter()
        .flat_map(|outs| outs.iter().copied())
        .map(|out| search_n_paths(graph, out, cache))
        .sum();
    cache.insert(dev, ans);
    ans
}

#[test]
fn example() {
    let inp = "
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    ";
    assert_eq!(run(inp), 5);
}
