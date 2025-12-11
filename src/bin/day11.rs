use std::collections::HashMap;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("answer: {:?}", run(&inp));
    println!("answer 2: {:?}", run2(&inp));
}

type Device = [u8; 3];
type Graph = HashMap<Device, Vec<Device>>;

fn str_to_dev(s: &str) -> Device {
    s.trim().as_bytes().try_into().unwrap()
}

const YOU: Device = *b"you";
const OUT: Device = *b"out";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    dev: Device,
    seen_dac: bool,
    seen_fft: bool,
}

const SVR: Device = *b"svr";
const DAC: Device = *b"dac";
const FFT: Device = *b"fft";

fn run2(inp: &str) -> usize {
    let graph = parse(inp);
    search_2(
        &graph,
        State {
            dev: SVR,
            seen_dac: false,
            seen_fft: false,
        },
        &mut Default::default(),
    )
}

fn search_2(graph: &Graph, mut state: State, cache: &mut HashMap<State, usize>) -> usize {
    match state.dev {
        OUT => {
            return if state.seen_dac && state.seen_fft {
                1
            } else {
                0
            };
        }
        DAC => state.seen_dac = true,
        FFT => state.seen_fft = true,
        _ => (),
    }
    if let Some(&ans) = cache.get(&state) {
        return ans;
    }
    let ans = graph
        .get(&state.dev)
        .into_iter()
        .flat_map(|outs| outs.iter().copied())
        .map(|out| search_2(graph, State { dev: out, ..state }, cache))
        .sum();
    cache.insert(state, ans);
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

#[test]
fn example2() {
    let inp = "
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    ";
    assert_eq!(run2(inp), 2);
}
