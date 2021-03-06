#![allow(dead_code)]

use std::cmp;
use std::cmp::Ordering;
use std::fs::File;
use std::iter;
use std::iter::FromIterator;
use std::io::{self, Read, BufReader, prelude::*};
use std::collections::{hash_map, HashMap, HashSet, BTreeMap, BTreeSet};
use vek::vec::repr_c::{Vec2, Vec3};
use vek::geom::repr_c::{Aabb, Rect};
use permutohedron::Heap;
use ordered_float::NotNan;
use itertools::Itertools;
use console::{Key, Term};

use sorted_bread_box::*;

mod intcode;
use intcode::*;

type Vec3i = Vec3<i32>;
type Vec2i = Vec2<i32>;

fn main() {
    // day1a();
    // day1b();
    // day2a();
    // day2b();
    // day3a();
    // day3b();
    // day4a();
    // // forgot to commit 4b.
    // // day4b();
    // day5a();
    // day5b();
    // day6a();
    // day6b();
    // day7a();
    // day7b();
    // day8a();
    // day8b();
    // day9a();
    // day9b();
    // day10a();
    // day10b();
    // day11a();
    // day11b();
    // day12a();
    // day12b();
    // day13a();
    // day13b();
    // day14a();
    // day14b();
    // day15();
    // day16a();
    // // day16b(); // too slow in debug
    // day17a();
    // day17b();
    // day18a();
    // day18b();
    day22a();
    day22b();
}

enum Shuffle22 {
    Cut(usize),
    CutBack(usize),
    Step(usize),
    Rev,
}

fn modular_pow(base:i128, exp:i128, mut modulus:i128) -> i128 {
    let mut powers_of_two = vec![base % modulus];
    let mut curr_power = 2;
    while curr_power < exp {
        let last_power = *powers_of_two.last().unwrap();
        powers_of_two.push((last_power * last_power) % modulus);
        curr_power <<= 1;
    }
    let mut res = 1;
    let (mut exponent_process, mut power_idx) = (exp, 0);
    while exponent_process != 0 {
        if (exponent_process & 1) == 1 { res = (res * powers_of_two[power_idx]) % modulus; }
        exponent_process >>= 1;
        power_idx += 1;
    }
    res
}
    
fn day22a() {
    use Shuffle22::*;
    let file = File::open("data/22.txt").unwrap();
    let read = BufReader::new(file);
    let mut ops = Vec::new();
    for line in read.lines() {
        let line = line.unwrap();
        const rev:&'static str = "deal into new stack";
        const step:&'static str = "deal with increment ";
        const cut:&'static str = "cut ";
        const cut_back:&'static str = "cut -";
        if line.starts_with(rev) {
            ops.push(Rev);
        }
        else if line.starts_with(step) {
            ops.push(Step(line[step.len()..].parse::<usize>().unwrap()));
        }
        else if line.starts_with(cut_back) {
            ops.push(CutBack(line[cut_back.len()..].parse::<usize>().unwrap()));
        }
        else if line.starts_with(cut) {
            ops.push(Cut(line[cut.len()..].parse::<usize>().unwrap()));
        }
    }

    let num = 10007;
    let mut deck:Vec<usize> = (0..num).collect();
    for op in ops.iter() {
        match op {
            Cut(n) => {
                deck.rotate_left(*n);
            },
            CutBack(n) => {
                deck.rotate_right(*n);
            },
            Step(n) => {
                let mut temp = Vec::new();
                temp.resize(deck.len(), 0);
                let mut idx = 0;
                for d in deck.into_iter() {
                    temp[idx] = d;
                    idx = (idx + n) % num;
                }
                deck = temp;
            },
            Rev => {
                deck.reverse();
            },
        }
    }
    // for n in deck.iter() {
    //     print!("{} ", n);
    // }
    // println!("");
    let pos = deck.iter().position(|x| *x == 2019).unwrap();
    println!("{}", pos);
    // assert_eq!(pos, 8775);
}

fn gcd128(mut x:i128, mut y:i128) -> i128 {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn day22b() {
    use Shuffle22::*;
    let file = File::open("data/22.txt").unwrap();
    let read = BufReader::new(file);
    let mut ops = Vec::new();
    for line in read.lines() {
        let line = line.unwrap();
        const rev:&'static str = "deal into new stack";
        const step:&'static str = "deal with increment ";
        const cut:&'static str = "cut ";
        const cut_back:&'static str = "cut -";
        if line.starts_with(rev) {
            ops.push(Rev);
        }
        else if line.starts_with(step) {
            ops.push(Step(line[step.len()..].parse::<usize>().unwrap()));
        }
        else if line.starts_with(cut_back) {
            ops.push(CutBack(line[cut_back.len()..].parse::<usize>().unwrap()));
        }
        else if line.starts_with(cut) {
            ops.push(Cut(line[cut.len()..].parse::<usize>().unwrap()));
        }
    }

    let num = 119315717514047;
    // let num = 10007;
    let mut idx = 2020;
    // let mut first = None;
    let mut ok = false;
    let mut times = 0;
    let mut mult = 1i128;
    let mut c = 0i128;
    for op in ops.iter() {
        match op {
            Cut(n) => {
                let n = *n as i128;
                c = (c + num - n) % num;
                
            },
            CutBack(n) => {
                let n = *n as i128;
                c = (c + n) % num;
            },
            Step(n) => {
                let n = *n as i128;
                mult = (mult * n) % num;
                c = (c * n) % num;
            },
            Rev => {
                mult = -mult;
                c = num - c - 1;
            },
        }
    }
    println!("n * {} + {}", mult, c);
    let rep = 101741582076661i128;
    // let rep = 52123i128;

    // let i1 = modular_pow(mult, rep, num) * idx % num;
    // let i2 = (modular_pow(mult, rep, num) + num - 1) % num;
    // let i3 = c * i2 % num;
    // let i4 = modular_pow(mult - 1, num - 2, num);
    // let ans = (i1 + i3 * i4) % num;
    // println!(". {}", ans);

    let i1 = modular_pow(mult, rep, num) * idx % num;
    let i2 = (modular_pow(mult, rep, num) + num - 1) % num;
    let i3 = c * i2 % num;
    let i4 = modular_pow(mult - 1, num - 2, num);
    let ans = (i1 + i3 * i4) % num;
    println!("{}", ans);

    // let mut idx = 2020;
    for _ in 0..rep {
        idx = (idx * mult + c) % num;
    }
    println!("{}", idx);
    
    // for n in deck.iter() {
    //     print!("{} ", n);
    // }
    // println!("");
    // println!("{}", idx);
    // assert_eq(pos, 8775);
}

// fn DAY 18 ---------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
enum Tile18 {
    None,
    Start,
    Key(char),
    Door(char),
}

fn door_key_mask(c: char) -> u32 {
    let idx = c as u8 - b'a';
    1 << idx
}

#[derive(Debug)]
struct Node18 {
    idx: i32,
    tile: Tile18,
    adj: u8,
    from: [i32; 4],
    dist: [u32; 4],
}

fn day18a() {
    let file = File::open("data/18.txt").unwrap();
    let mut map = Vec::new();
    let mut w = std::i32::MAX;
    let mut start = 0;
    for c in file.bytes().map(|r| r.expect("failed to read file") as char) {
        match c {
            '\r' => {},
            '\n' => w = w.min(map.len() as i32),
            '@' => {start = map.len() as i32; map.push(c)},
            _ => map.push(c),
        }
    }
    let adjecent = [(1i32, 0b0001), (-1, 0b0010), (w, 0b0100), (-w, 0b1000)];
    let rev_adjecent = [1, 0, 3, 2];

    let mut keys = Vec::new();
    let mut nodes = HashMap::<i32, Node18>::new();
    for idx in map.iter().enumerate().filter_map(|(idx, c)| if *c == '#' {None} else {Some(idx as i32)}) {
        let mut mask = 0;
        let mut count = 0;
        let mut valid_pos = !0;
        let mut node = Node18{
            idx,
            tile: match map[idx as usize] {
                c@'A'..='Z' => Tile18::Door(c.to_ascii_lowercase()),
                c@'a'..='z' => {keys.push((idx, c)); Tile18::Key(c)},
                '@' => Tile18::Start,
                '.' => Tile18::None,
                c => panic!("invalid map char! {} {}", c, idx),
            },
            adj: 0,
            from: [!0;4],
            dist: [!0;4],
        };
        for (off, adj_mask) in adjecent.iter() {
            let pos = idx + off;
            if pos > 0 && (pos as usize) < map.len() && map[pos as usize] != '#' {
                let at = node.adj as usize;
                node.from[at] = pos;
                node.dist[at] = 1;
                node.adj += 1;
            }
        }
        nodes.insert(idx, node);
    }
    println!("From {} tiles to {} nodes", map.len(), nodes.len());
    let num_nodes = nodes.len();
    // remove leaves
    let mut to_check:Vec<i32> = nodes.iter().filter_map(|(k, n)| {
        if n.tile == Tile18::None && n.adj == 1 {Some(*k)} else {None}
    }).collect();
    while !to_check.is_empty() {
        let k = to_check.pop().unwrap();
        let node = nodes.remove(&k).expect("trying to remove empty nodes that doesn't exist");
        let other = nodes.get_mut(&node.from[0]).unwrap();
        let at = other.from.iter().position(|v| *v == k).unwrap();
        other.from[at..].rotate_left(1);
        other.dist[at..].rotate_left(1);
        other.adj -= 1;
        if other.adj <= 1 && other.tile == Tile18::None {
            to_check.push(other.idx);
        }
    }

    println!("Remove leaves. From {} nodes to {} nodes", num_nodes, nodes.len());
    let num_nodes = nodes.len();
    // remove "paths" i.e. None tiles with 2 adj
    let mut to_check:Vec<i32> = nodes.iter().filter_map(|(k, n)| {
        if n.tile == Tile18::None && n.adj == 2 {Some(*k)} else {None}
    }).collect();
    while !to_check.is_empty() {
        let k = to_check.pop().unwrap();
        let node = nodes.remove(&k).unwrap();
        let dist = node.dist[0] + node.dist[1];

        let other = nodes.get_mut(&node.from[0]).unwrap();
        let at = other.from.iter().position(|v| *v == k).unwrap();
        other.from[at] = node.from[1];
        other.dist[at] = dist;

        let other = nodes.get_mut(&node.from[1]).unwrap();
        let at = other.from.iter().position(|v| *v == k).unwrap();
        other.from[at] = node.from[0];
        other.dist[at] = dist;
    }
    println!("Remove 'paths'. From {} nodes to {} nodes", num_nodes, nodes.len());
    // let num_nodes = nodes.len();
    // for (idx, mut c) in map.iter().enumerate() {
    //     let idx = idx as i32;
    //     if idx != 0 && idx % w == 0 {
    //         println!("");
    //     }
    //     c = if nodes.get(&idx).is_some() {&'*'} else {c}; 
    //     print!("{}", c);
    // }
    let mut node_dist = HashMap::new();
    node_dist.insert((0, start), 0);
    
    let all_keys = (1 << keys.len()) - 1;
    let mut to_check = Vec::new();
    to_check.push((0, 0, start));
    let comp = |r:(u32,u32,i32),l:(u32,u32,i32)| -> Ordering {r.0.cmp(&l.0).reverse().then(r.1.cmp(&l.1)).then(r.2.cmp(&l.2))};
    while !to_check.is_empty() {
        let (dist, mut key, idx) = to_check.pop().unwrap();

        let node = nodes.get(&idx).unwrap();
        if !match node.tile {
            Tile18::Door(c) => door_key_mask(c) & key != 0,
            Tile18::Key(c) => {key |= door_key_mask(c); true},
            _ => true,
        } {
            continue;
        }
        if key == all_keys {
            println!("Day18a: {}", dist);
            assert_eq!(dist, 5402);
            break;
        }
        for n in 0..node.adj as usize {
            let at = node.from[n];
            let alt = node.dist[n] + dist;
            let dist_ref = node_dist.entry((key, at)).or_insert(!0);
            let current_dist = *dist_ref;
            if current_dist > alt {
                *dist_ref = alt;
                // remove old
                to_check.binary_search_by(|o| comp(*o, (current_dist, key, at))).map(|n| to_check.remove(n));
                // add new
                let ins = to_check.binary_search_by(|o| comp(*o, (alt, key, at))).unwrap_err();
                to_check.insert(ins, (alt, key, at));
            }
        }
    }
}

fn day18b() {
    let file = File::open("data/18b.txt").unwrap();
    let mut map = Vec::new();
    let mut w = std::i32::MAX;
    let mut start = Vec::new();
    for c in file.bytes().map(|r| r.expect("failed to read file") as char) {
        match c {
            '\r' => {},
            '\n' => w = w.min(map.len() as i32),
            '@' => {start.push(map.len() as i32); map.push(c)},
            _ => map.push(c),
        }
    }
    let adjecent = [(1i32, 0b0001), (-1, 0b0010), (w, 0b0100), (-w, 0b1000)];

    let mut keys = Vec::new();
    let mut nodes = HashMap::<i32, Node18>::new();
    for idx in map.iter().enumerate().filter_map(|(idx, c)| if *c == '#' {None} else {Some(idx as i32)}) {
        let mut mask = 0;
        let mut count = 0;
        let mut valid_pos = !0;
        let mut node = Node18{
            idx,
            tile: match map[idx as usize] {
                c@'A'..='Z' => Tile18::Door(c.to_ascii_lowercase()),
                c@'a'..='z' => {keys.push((idx, c)); Tile18::Key(c)},
                '@' => Tile18::Start,
                '.' => Tile18::None,
                c => panic!("invalid map char! {} {}", c, idx),
            },
            adj: 0,
            from: [!0;4],
            dist: [!0;4],
        };
        for (off, adj_mask) in adjecent.iter() {
            let pos = idx + off;
            if pos > 0 && (pos as usize) < map.len() && map[pos as usize] != '#' {
                let at = node.adj as usize;
                node.from[at] = pos;
                node.dist[at] = 1;
                node.adj += 1;
            }
        }
        nodes.insert(idx, node);
    }
    println!("From {} tiles to {} nodes", map.len(), nodes.len());
    let num_nodes = nodes.len();
    // remove leaves
    let mut to_check:Vec<i32> = nodes.iter().filter_map(|(k, n)| {
        if n.tile == Tile18::None && n.adj == 1 {Some(*k)} else {None}
    }).collect();
    while !to_check.is_empty() {
        let k = to_check.pop().unwrap();
        let node = nodes.remove(&k).expect("trying to remove empty nodes that doesn't exist");
        let other = nodes.get_mut(&node.from[0]).unwrap();
        let at = other.from.iter().position(|v| *v == k).unwrap();
        other.from[at..].rotate_left(1);
        other.dist[at..].rotate_left(1);
        other.adj -= 1;
        if other.adj <= 1 && other.tile == Tile18::None {
            to_check.push(other.idx);
        }
    }

    println!("Remove leaves. From {} nodes to {} nodes", num_nodes, nodes.len());
    let num_nodes = nodes.len();
    // remove "paths" i.e. None tiles with 2 adj
    let mut to_check:Vec<i32> = nodes.iter().filter_map(|(k, n)| {
        if n.tile == Tile18::None && n.adj == 2 {Some(*k)} else {None}
    }).collect();
    while !to_check.is_empty() {
        let k = to_check.pop().unwrap();
        let node = nodes.remove(&k).unwrap();
        let dist = node.dist[0] + node.dist[1];

        let other = nodes.get_mut(&node.from[0]).unwrap();
        let at = other.from.iter().position(|v| *v == k).unwrap();
        other.from[at] = node.from[1];
        other.dist[at] = dist;

        let other = nodes.get_mut(&node.from[1]).unwrap();
        let at = other.from.iter().position(|v| *v == k).unwrap();
        other.from[at] = node.from[0];
        other.dist[at] = dist;
    }
    println!("Remove 'paths'. From {} nodes to {} nodes", num_nodes, nodes.len());
    
    
    let all_keys = (1 << keys.len()) - 1;
    let mut node_dist:HashMap<_, _> = start.iter().map(|i| ((0, *i), (0, (0, !0)))).collect();
    let mut to_check:Vec<_> = start.iter().enumerate().map(|(idx, i)| (0u32, 0u32, *i, idx)).collect();

    let mut term = Term::stdout();

    let mut pos_cache:HashMap<u32,Vec<i32>> = HashMap::new();
    pos_cache.insert(0, start);
    let comp = |r:(u32,u32,i32,usize),l:(u32,u32,i32,usize)| -> Ordering {r.0.cmp(&l.0).reverse().then(r.1.cmp(&l.1)).then(r.2.cmp(&l.2)).then(r.3.cmp(&l.3))};
    while !to_check.is_empty() {
        let (dist, mut key, idx, bot) = to_check.pop().unwrap();
        let old_key = key.clone();
        let node = nodes.get(&idx).unwrap();
        if !match node.tile {
            Tile18::Door(c) => door_key_mask(c) & key != 0,
            Tile18::Key(c) => {
                let at_key = door_key_mask(c);
                if key & at_key == 0 {
                    let mut bots_pos:Vec<i32> = pos_cache.get(&key).unwrap().clone();
                    bots_pos[bot] = idx;
                    key |= door_key_mask(c);
                    let (dist_ref, from_ref) = node_dist.entry((key, idx)).or_insert((!0, (0, !0)));
                    if *dist_ref > dist {
                        *dist_ref = dist;
                        *from_ref = (old_key, idx);
                    }
                    for (bot, at) in bots_pos.iter().enumerate() {
                        let at = *at;
                        if at != idx {
                            let (dist_ref, from_ref) = node_dist.entry((key, at)).or_insert((!0, (0, !0)));
                            *dist_ref = dist;
                            *from_ref = (key, idx);
                            let ins = to_check.binary_search_by(|o| comp(*o, (dist, key, at, bot))).unwrap_err();
                            to_check.insert(ins, (dist, key, at, bot));
                        }
                    }
                    pos_cache.insert(key, bots_pos);
                }
                true
            },
            _ => true,
        } {
            continue;
        }
        if key == all_keys {
            println!("Day18b: {}", dist);
            let mut at = (key, idx);
            let mut nd = *node_dist.get(&at).unwrap();
            let mut last = (key, idx);
            loop {
                println!("{:?} from {:?} (dist: {})", last, nd.1, nd.0);

                // for (idx, mut c) in map.iter().enumerate() {
                //     let idx = idx as i32;
                //     if idx != 0 && idx % w == 0 {
                //         println!("");
                //     }
                //     c = if idx == (nd.1).1 {&'*'} else if idx == last.1 {&'0'} else {c}; 
                //     print!("{}", c);
                // }
                // println!("");
                term.read_key();

                if (nd.1).1 == !0 || nd.0 == 38 && (nd.1).1 == 100 {
                    println!("Stopped");
                    break;
                }
                last =  nd.1;
                nd = *node_dist.get(&nd.1).unwrap();
            }
            break;
        }
        for n in 0..node.adj as usize {
            let at = node.from[n];
            let alt = node.dist[n] + dist;
            let (dist_ref, from_ref) = node_dist.entry((key, at)).or_insert((!0, (0, !0)));
            let current_dist = *dist_ref;
            if current_dist > alt {
                *from_ref = (key, idx);
                *dist_ref = alt;
                // remove old
                to_check.binary_search_by(|o| comp(*o, (current_dist, key, at, bot))).map(|n| to_check.remove(n));
                // add new
                let ins = to_check.binary_search_by(|o| comp(*o, (alt, key, at, bot))).unwrap_err();
                to_check.insert(ins, (alt, key, at, bot));
            }
        }
    }
}

// DAY 17 ---------------------------------------------------------------------------------------------

fn day17a() {
    let mut comp = Intcode::new(&[1,330,331,332,109,2650,1102,1,1182,15,1102,1429,1,24,1001,0,0,570,1006,570,36,102,1,571,0,1001,570,-1,570,1001,24,1,24,1105,1,18,1008,571,0,571,1001,15,1,15,1008,15,1429,570,1006,570,14,21102,58,1,0,1105,1,786,1006,332,62,99,21101,333,0,1,21102,1,73,0,1105,1,579,1102,0,1,572,1102,1,0,573,3,574,101,1,573,573,1007,574,65,570,1005,570,151,107,67,574,570,1005,570,151,1001,574,-64,574,1002,574,-1,574,1001,572,1,572,1007,572,11,570,1006,570,165,101,1182,572,127,1002,574,1,0,3,574,101,1,573,573,1008,574,10,570,1005,570,189,1008,574,44,570,1006,570,158,1106,0,81,21101,0,340,1,1105,1,177,21101,477,0,1,1105,1,177,21102,1,514,1,21101,0,176,0,1105,1,579,99,21101,184,0,0,1105,1,579,4,574,104,10,99,1007,573,22,570,1006,570,165,1002,572,1,1182,21102,1,375,1,21102,1,211,0,1105,1,579,21101,1182,11,1,21101,0,222,0,1106,0,979,21101,388,0,1,21102,233,1,0,1106,0,579,21101,1182,22,1,21102,244,1,0,1105,1,979,21102,1,401,1,21101,255,0,0,1105,1,579,21101,1182,33,1,21102,266,1,0,1105,1,979,21102,414,1,1,21102,277,1,0,1105,1,579,3,575,1008,575,89,570,1008,575,121,575,1,575,570,575,3,574,1008,574,10,570,1006,570,291,104,10,21101,1182,0,1,21101,0,313,0,1106,0,622,1005,575,327,1102,1,1,575,21102,1,327,0,1106,0,786,4,438,99,0,1,1,6,77,97,105,110,58,10,33,10,69,120,112,101,99,116,101,100,32,102,117,110,99,116,105,111,110,32,110,97,109,101,32,98,117,116,32,103,111,116,58,32,0,12,70,117,110,99,116,105,111,110,32,65,58,10,12,70,117,110,99,116,105,111,110,32,66,58,10,12,70,117,110,99,116,105,111,110,32,67,58,10,23,67,111,110,116,105,110,117,111,117,115,32,118,105,100,101,111,32,102,101,101,100,63,10,0,37,10,69,120,112,101,99,116,101,100,32,82,44,32,76,44,32,111,114,32,100,105,115,116,97,110,99,101,32,98,117,116,32,103,111,116,58,32,36,10,69,120,112,101,99,116,101,100,32,99,111,109,109,97,32,111,114,32,110,101,119,108,105,110,101,32,98,117,116,32,103,111,116,58,32,43,10,68,101,102,105,110,105,116,105,111,110,115,32,109,97,121,32,98,101,32,97,116,32,109,111,115,116,32,50,48,32,99,104,97,114,97,99,116,101,114,115,33,10,94,62,118,60,0,1,0,-1,-1,0,1,0,0,0,0,0,0,1,36,16,0,109,4,2102,1,-3,587,20102,1,0,-1,22101,1,-3,-3,21102,0,1,-2,2208,-2,-1,570,1005,570,617,2201,-3,-2,609,4,0,21201,-2,1,-2,1105,1,597,109,-4,2105,1,0,109,5,2102,1,-4,630,20101,0,0,-2,22101,1,-4,-4,21102,1,0,-3,2208,-3,-2,570,1005,570,781,2201,-4,-3,653,20102,1,0,-1,1208,-1,-4,570,1005,570,709,1208,-1,-5,570,1005,570,734,1207,-1,0,570,1005,570,759,1206,-1,774,1001,578,562,684,1,0,576,576,1001,578,566,692,1,0,577,577,21101,0,702,0,1106,0,786,21201,-1,-1,-1,1106,0,676,1001,578,1,578,1008,578,4,570,1006,570,724,1001,578,-4,578,21101,731,0,0,1105,1,786,1106,0,774,1001,578,-1,578,1008,578,-1,570,1006,570,749,1001,578,4,578,21101,756,0,0,1106,0,786,1106,0,774,21202,-1,-11,1,22101,1182,1,1,21102,1,774,0,1105,1,622,21201,-3,1,-3,1105,1,640,109,-5,2106,0,0,109,7,1005,575,802,20101,0,576,-6,20101,0,577,-5,1106,0,814,21102,0,1,-1,21101,0,0,-5,21101,0,0,-6,20208,-6,576,-2,208,-5,577,570,22002,570,-2,-2,21202,-5,37,-3,22201,-6,-3,-3,22101,1429,-3,-3,1202,-3,1,843,1005,0,863,21202,-2,42,-4,22101,46,-4,-4,1206,-2,924,21101,0,1,-1,1106,0,924,1205,-2,873,21101,35,0,-4,1105,1,924,2102,1,-3,878,1008,0,1,570,1006,570,916,1001,374,1,374,2102,1,-3,895,1102,1,2,0,2102,1,-3,902,1001,438,0,438,2202,-6,-5,570,1,570,374,570,1,570,438,438,1001,578,558,921,21001,0,0,-4,1006,575,959,204,-4,22101,1,-6,-6,1208,-6,37,570,1006,570,814,104,10,22101,1,-5,-5,1208,-5,33,570,1006,570,810,104,10,1206,-1,974,99,1206,-1,974,1101,0,1,575,21102,973,1,0,1106,0,786,99,109,-7,2106,0,0,109,6,21101,0,0,-4,21102,0,1,-3,203,-2,22101,1,-3,-3,21208,-2,82,-1,1205,-1,1030,21208,-2,76,-1,1205,-1,1037,21207,-2,48,-1,1205,-1,1124,22107,57,-2,-1,1205,-1,1124,21201,-2,-48,-2,1105,1,1041,21101,0,-4,-2,1105,1,1041,21102,-5,1,-2,21201,-4,1,-4,21207,-4,11,-1,1206,-1,1138,2201,-5,-4,1059,1201,-2,0,0,203,-2,22101,1,-3,-3,21207,-2,48,-1,1205,-1,1107,22107,57,-2,-1,1205,-1,1107,21201,-2,-48,-2,2201,-5,-4,1090,20102,10,0,-1,22201,-2,-1,-2,2201,-5,-4,1103,2101,0,-2,0,1105,1,1060,21208,-2,10,-1,1205,-1,1162,21208,-2,44,-1,1206,-1,1131,1106,0,989,21101,0,439,1,1105,1,1150,21101,0,477,1,1105,1,1150,21101,0,514,1,21102,1,1149,0,1106,0,579,99,21101,1157,0,0,1106,0,579,204,-2,104,10,99,21207,-3,22,-1,1206,-1,1138,1202,-5,1,1176,2102,1,-4,0,109,-6,2105,1,0,8,7,30,1,5,1,30,1,5,1,30,1,5,1,22,9,1,9,18,1,9,1,3,1,3,1,18,1,3,11,3,1,18,1,3,1,5,1,7,1,18,1,3,1,5,1,7,1,1,5,1,7,4,1,3,1,5,1,7,1,1,1,3,1,1,1,5,1,4,11,7,1,1,1,3,1,1,1,5,1,8,1,13,1,1,1,3,1,1,1,5,1,8,1,13,1,1,11,1,1,8,1,13,1,5,1,1,1,5,1,8,7,7,7,1,1,5,1,14,1,15,1,5,1,14,1,15,1,5,5,10,1,15,1,20,1,1,7,7,7,14,1,1,1,5,1,13,1,10,11,1,1,13,1,10,1,3,1,1,1,3,1,1,1,13,1,10,1,3,1,1,1,3,1,1,1,7,11,6,1,3,1,1,1,3,1,1,1,7,1,5,1,3,1,6,5,1,5,1,1,7,1,5,1,3,1,18,1,7,1,5,1,3,1,18,1,3,11,3,1,18,1,3,1,3,1,9,1,18,9,1,9,22,1,5,1,30,1,5,1,30,1,5,1,30,7,8]);
    let mut map = Vec::new();
    let mut w = 0i32;
    // let mut h = 0i32;
    while let Some(res) = comp.run_to_out(|| unimplemented!()) {
        // print!("{}", res as u8 as char);
        if res == 10 {
            // h += 1;
            if w == 0 {
                w = map.len() as i32;
            }
        }
        else {
            map.push(res != 46);
        }
    }
    // println!("");

    let adjecent = vec![1, -1, w, -w];
    let mut sum = 0;
    for (idx, _) in map.iter().enumerate().filter(|(_, c)| **c) {
        let idx = idx as i32;
        let range = 0..map.len() as i32;
        let intersection = adjecent.iter().all(|off| range.contains(&(idx + off)) && map[(idx + *off) as usize]);
        if intersection {
            let x = idx % w;
            let y = idx / w;
            sum += x * y;
        }
    }
    println!("Day17a: {}", sum); // 3192
}

fn day17b() {
    let mut comp = Intcode::new(&[2,330,331,332,109,2650,1102,1,1182,15,1102,1429,1,24,1001,0,0,570,1006,570,36,102,1,571,0,1001,570,-1,570,1001,24,1,24,1105,1,18,1008,571,0,571,1001,15,1,15,1008,15,1429,570,1006,570,14,21102,58,1,0,1105,1,786,1006,332,62,99,21101,333,0,1,21102,1,73,0,1105,1,579,1102,0,1,572,1102,1,0,573,3,574,101,1,573,573,1007,574,65,570,1005,570,151,107,67,574,570,1005,570,151,1001,574,-64,574,1002,574,-1,574,1001,572,1,572,1007,572,11,570,1006,570,165,101,1182,572,127,1002,574,1,0,3,574,101,1,573,573,1008,574,10,570,1005,570,189,1008,574,44,570,1006,570,158,1106,0,81,21101,0,340,1,1105,1,177,21101,477,0,1,1105,1,177,21102,1,514,1,21101,0,176,0,1105,1,579,99,21101,184,0,0,1105,1,579,4,574,104,10,99,1007,573,22,570,1006,570,165,1002,572,1,1182,21102,1,375,1,21102,1,211,0,1105,1,579,21101,1182,11,1,21101,0,222,0,1106,0,979,21101,388,0,1,21102,233,1,0,1106,0,579,21101,1182,22,1,21102,244,1,0,1105,1,979,21102,1,401,1,21101,255,0,0,1105,1,579,21101,1182,33,1,21102,266,1,0,1105,1,979,21102,414,1,1,21102,277,1,0,1105,1,579,3,575,1008,575,89,570,1008,575,121,575,1,575,570,575,3,574,1008,574,10,570,1006,570,291,104,10,21101,1182,0,1,21101,0,313,0,1106,0,622,1005,575,327,1102,1,1,575,21102,1,327,0,1106,0,786,4,438,99,0,1,1,6,77,97,105,110,58,10,33,10,69,120,112,101,99,116,101,100,32,102,117,110,99,116,105,111,110,32,110,97,109,101,32,98,117,116,32,103,111,116,58,32,0,12,70,117,110,99,116,105,111,110,32,65,58,10,12,70,117,110,99,116,105,111,110,32,66,58,10,12,70,117,110,99,116,105,111,110,32,67,58,10,23,67,111,110,116,105,110,117,111,117,115,32,118,105,100,101,111,32,102,101,101,100,63,10,0,37,10,69,120,112,101,99,116,101,100,32,82,44,32,76,44,32,111,114,32,100,105,115,116,97,110,99,101,32,98,117,116,32,103,111,116,58,32,36,10,69,120,112,101,99,116,101,100,32,99,111,109,109,97,32,111,114,32,110,101,119,108,105,110,101,32,98,117,116,32,103,111,116,58,32,43,10,68,101,102,105,110,105,116,105,111,110,115,32,109,97,121,32,98,101,32,97,116,32,109,111,115,116,32,50,48,32,99,104,97,114,97,99,116,101,114,115,33,10,94,62,118,60,0,1,0,-1,-1,0,1,0,0,0,0,0,0,1,36,16,0,109,4,2102,1,-3,587,20102,1,0,-1,22101,1,-3,-3,21102,0,1,-2,2208,-2,-1,570,1005,570,617,2201,-3,-2,609,4,0,21201,-2,1,-2,1105,1,597,109,-4,2105,1,0,109,5,2102,1,-4,630,20101,0,0,-2,22101,1,-4,-4,21102,1,0,-3,2208,-3,-2,570,1005,570,781,2201,-4,-3,653,20102,1,0,-1,1208,-1,-4,570,1005,570,709,1208,-1,-5,570,1005,570,734,1207,-1,0,570,1005,570,759,1206,-1,774,1001,578,562,684,1,0,576,576,1001,578,566,692,1,0,577,577,21101,0,702,0,1106,0,786,21201,-1,-1,-1,1106,0,676,1001,578,1,578,1008,578,4,570,1006,570,724,1001,578,-4,578,21101,731,0,0,1105,1,786,1106,0,774,1001,578,-1,578,1008,578,-1,570,1006,570,749,1001,578,4,578,21101,756,0,0,1106,0,786,1106,0,774,21202,-1,-11,1,22101,1182,1,1,21102,1,774,0,1105,1,622,21201,-3,1,-3,1105,1,640,109,-5,2106,0,0,109,7,1005,575,802,20101,0,576,-6,20101,0,577,-5,1106,0,814,21102,0,1,-1,21101,0,0,-5,21101,0,0,-6,20208,-6,576,-2,208,-5,577,570,22002,570,-2,-2,21202,-5,37,-3,22201,-6,-3,-3,22101,1429,-3,-3,1202,-3,1,843,1005,0,863,21202,-2,42,-4,22101,46,-4,-4,1206,-2,924,21101,0,1,-1,1106,0,924,1205,-2,873,21101,35,0,-4,1105,1,924,2102,1,-3,878,1008,0,1,570,1006,570,916,1001,374,1,374,2102,1,-3,895,1102,1,2,0,2102,1,-3,902,1001,438,0,438,2202,-6,-5,570,1,570,374,570,1,570,438,438,1001,578,558,921,21001,0,0,-4,1006,575,959,204,-4,22101,1,-6,-6,1208,-6,37,570,1006,570,814,104,10,22101,1,-5,-5,1208,-5,33,570,1006,570,810,104,10,1206,-1,974,99,1206,-1,974,1101,0,1,575,21102,973,1,0,1106,0,786,99,109,-7,2106,0,0,109,6,21101,0,0,-4,21102,0,1,-3,203,-2,22101,1,-3,-3,21208,-2,82,-1,1205,-1,1030,21208,-2,76,-1,1205,-1,1037,21207,-2,48,-1,1205,-1,1124,22107,57,-2,-1,1205,-1,1124,21201,-2,-48,-2,1105,1,1041,21101,0,-4,-2,1105,1,1041,21102,-5,1,-2,21201,-4,1,-4,21207,-4,11,-1,1206,-1,1138,2201,-5,-4,1059,1201,-2,0,0,203,-2,22101,1,-3,-3,21207,-2,48,-1,1205,-1,1107,22107,57,-2,-1,1205,-1,1107,21201,-2,-48,-2,2201,-5,-4,1090,20102,10,0,-1,22201,-2,-1,-2,2201,-5,-4,1103,2101,0,-2,0,1105,1,1060,21208,-2,10,-1,1205,-1,1162,21208,-2,44,-1,1206,-1,1131,1106,0,989,21101,0,439,1,1105,1,1150,21101,0,477,1,1105,1,1150,21101,0,514,1,21102,1,1149,0,1106,0,579,99,21101,1157,0,0,1106,0,579,204,-2,104,10,99,21207,-3,22,-1,1206,-1,1138,1202,-5,1,1176,2102,1,-4,0,109,-6,2105,1,0,8,7,30,1,5,1,30,1,5,1,30,1,5,1,22,9,1,9,18,1,9,1,3,1,3,1,18,1,3,11,3,1,18,1,3,1,5,1,7,1,18,1,3,1,5,1,7,1,1,5,1,7,4,1,3,1,5,1,7,1,1,1,3,1,1,1,5,1,4,11,7,1,1,1,3,1,1,1,5,1,8,1,13,1,1,1,3,1,1,1,5,1,8,1,13,1,1,11,1,1,8,1,13,1,5,1,1,1,5,1,8,7,7,7,1,1,5,1,14,1,15,1,5,1,14,1,15,1,5,5,10,1,15,1,20,1,1,7,7,7,14,1,1,1,5,1,13,1,10,11,1,1,13,1,10,1,3,1,1,1,3,1,1,1,13,1,10,1,3,1,1,1,3,1,1,1,7,11,6,1,3,1,1,1,3,1,1,1,7,1,5,1,3,1,6,5,1,5,1,1,7,1,5,1,3,1,18,1,7,1,5,1,3,1,18,1,3,11,3,1,18,1,3,1,3,1,9,1,18,9,1,9,22,1,5,1,30,1,5,1,30,1,5,1,30,7,8]);
    let mut input:Vec<i64> = b"A,B,A,B,C,C,B,A,B,C\nL,4,R,8,L,6,L,10\nL,6,R,8,R,10,L,6,L,6\nL,4,L,4,L,10\nn\n".iter().map(|c| *c as i64).rev().collect();
    // skip the map
    let (ok, _) = comp.run_to_sized_out(1253, || unimplemented!());
    assert!(ok);

    let mut last = 0;
    while let Some(res) = comp.run_to_out(|| input.pop().expect("out of input")) {
        last = res;
        // print!("{}", res as u8 as char);
        // if res == b'\n' as i64 && !input.is_empty() {
        //     println!("{:?}", input);
        // }
    }
    println!("Day17b: {}", last); // 684691
    assert_eq!(last, 684691);
}

// DAY 16 ---------------------------------------------------------------------------------------------

fn day16a() {
    let mut signal:Vec<u8> = b"59754835304279095723667830764559994207668723615273907123832849523285892960990393495763064170399328763959561728553125232713663009161639789035331160605704223863754174835946381029543455581717775283582638013183215312822018348826709095340993876483418084566769957325454646682224309983510781204738662326823284208246064957584474684120465225052336374823382738788573365821572559301715471129142028462682986045997614184200503304763967364026464055684787169501819241361777789595715281841253470186857857671012867285957360755646446993278909888646724963166642032217322712337954157163771552371824741783496515778370667935574438315692768492954716331430001072240959235708".iter().map(|c| c - b'0').collect();

    let base_pattern = [0, 1, 0, -1];
    let mut patterns = Vec::<Vec<i8>>::new();
    for n in 0..signal.len() {
        let mut base_it = base_pattern.iter().cycle();
        let mut count = n;
        let mut v:i8 = *base_it.next().unwrap();
        patterns.push(iter::repeat_with(|| {
            if count == 0 {
                count = n;
                v = *base_it.next().unwrap();
            }
            else {
                count -= 1;
            }
            v
        }).take(signal.len()).collect());
    }

    let mut next = Vec::new();
    next.resize(signal.len(), 0);
    for _ in 0..100 {
        for n in 0..signal.len() {
            let mut v = 0i32;
            for (s, p) in signal.iter().zip(patterns[n].iter()) {
                v += *s as i32 * *p as i32;
            }
            next[n] = (v % 10).abs() as u8;
        }
        std::mem::swap(&mut next, &mut signal);
    }
    signal = signal.iter().map(|v| v + b'0').take(8).collect();
    let res = String::from_utf8_lossy(&signal);
    println!("Day16a: {}", res);
    assert_eq!(res, "22122816");
}

fn day16b() {
    let data = "59754835304279095723667830764559994207668723615273907123832849523285892960990393495763064170399328763959561728553125232713663009161639789035331160605704223863754174835946381029543455581717775283582638013183215312822018348826709095340993876483418084566769957325454646682224309983510781204738662326823284208246064957584474684120465225052336374823382738788573365821572559301715471129142028462682986045997614184200503304763967364026464055684787169501819241361777789595715281841253470186857857671012867285957360755646446993278909888646724963166642032217322712337954157163771552371824741783496515778370667935574438315692768492954716331430001072240959235708";
    let limit = 5975483;
    let mut signal:Vec<u8> = data.chars().cycle().take(data.len() * 10_000).skip(limit).map(|c| c.to_digit(10).unwrap() as u8).collect();

    let mut next = Vec::new();
    next.resize(signal.len(), 0);
    for _ in 0..100 {
        let mut sum = 0;
        for n in (0..signal.len()).rev() {
            sum = (sum + signal[n]) % 10;
            next[n] = sum;
        }
        std::mem::swap(&mut next, &mut signal);
    }
    let s:String = signal.iter().take(8).map(|v| v.to_string()).collect();
    println!("Day16b: {}", s);
}

// DAY 15 ---------------------------------------------------------------------------------------------

struct FindData {
    mask: u8,
    back: u8,
    steps:usize,
}

impl FindData {
    fn rev_dir(dir:u8) -> u8 {
        match dir {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => panic!("invalid dir"),
        }
    }

    fn new(dir:u8, steps:usize) -> Self {
        let from = Self::rev_dir(dir);
        let mut mask = 0b11110;
        mask &= !(1 << from);
        FindData {
            mask,
            back: from,
            steps,
        }
    }

    fn new_root() -> Self {
        FindData {
            mask: 0b11110,
            back: !0,
            steps: 0,
        }
    }

    fn is_root(&self) -> bool {
        self.back == !0
    }

    fn mark(&mut self, dir:u8) -> bool {
        self.mask &= !(1 << dir);
        self.mask != 0
    }

    fn came_from(&mut self, dir:u8, steps:usize) {
        if steps < self.steps {
            self.steps = steps;
            self.back = dir;
        }
        self.mark(Self::rev_dir(dir));
    }

    fn has_dir(&self) -> bool {
        self.mask != 0
    }

    fn next_dir(&self) -> Option<u8> {
        if self.mask == 0 {
            None
        }
        else {
            let mut dir = 0;
            let mut m = self.mask;
            while m & 0b1 == 0 {
                dir += 1;
                m >>= 1;
                assert_ne!(dir, 5); // out of bounds
            }
            assert_ne!(dir, 0); // bad mask as the first bit should be empty
            Some(dir)
        }
    }

    fn calc_pos(pos: Vec2<i64>, dir:u8) -> Vec2<i64> {
        pos + Self::calc_dir(dir)
    }

    fn calc_dir(dir:u8) -> Vec2<i64> {
        match dir {
            1 => Vec2::new(0, -1),
            2 => Vec2::new(0, 1),
            3 => Vec2::new(-1, 0),
            4 => Vec2::new(1, 0),
            _ => panic!("invalid dir code"),
        }
    }
}

fn day15() {
    let mut comp = Intcode::new(&[3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,1002,1034,1,1039,1002,1036,1,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1106,0,124,1001,1034,0,1039,1002,1036,1,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1106,0,124,1001,1034,-1,1039,1008,1036,0,1041,1002,1035,1,1040,1001,1038,0,1043,101,0,1037,1042,1105,1,124,1001,1034,1,1039,1008,1036,0,1041,102,1,1035,1040,1001,1038,0,1043,101,0,1037,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,39,1032,1006,1032,165,1008,1040,3,1032,1006,1032,165,1102,1,2,1044,1106,0,224,2,1041,1043,1032,1006,1032,179,1102,1,1,1044,1106,0,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,59,1044,1105,1,224,1102,1,0,1044,1105,1,224,1006,1044,247,101,0,1039,1034,1001,1040,0,1035,101,0,1041,1036,1002,1043,1,1038,1002,1042,1,1037,4,1044,1105,1,0,93,27,71,56,88,17,30,78,5,57,79,56,3,82,62,58,16,2,21,89,95,33,12,32,90,12,7,76,83,31,8,13,27,89,60,33,7,40,22,50,8,63,35,45,57,94,81,4,65,33,47,73,28,98,11,70,95,17,82,39,19,73,62,56,80,85,23,91,39,86,91,82,50,37,86,4,90,83,8,65,56,63,15,99,51,3,60,60,77,58,90,82,5,52,14,87,37,74,85,43,17,61,91,35,31,81,19,12,34,54,9,66,34,69,67,21,4,14,87,22,76,26,82,79,4,69,48,73,8,73,57,61,83,23,83,60,3,41,75,67,53,44,91,27,52,84,66,13,65,95,81,83,30,26,60,12,33,92,81,46,78,25,13,72,87,26,63,57,35,2,60,96,63,26,2,76,95,21,38,60,5,79,86,89,47,42,12,91,30,52,69,55,67,73,47,44,5,86,8,52,69,81,23,70,3,38,41,89,88,58,41,9,96,27,67,21,14,68,67,35,84,23,20,91,63,47,75,34,70,57,13,54,82,33,61,27,97,88,46,44,56,74,14,5,96,71,16,40,86,61,84,41,81,81,16,88,51,41,96,76,28,97,44,41,65,87,50,73,58,71,46,73,51,43,18,46,99,74,65,9,89,3,77,22,34,93,94,39,54,96,12,35,62,87,56,69,64,9,34,91,64,71,28,10,94,1,96,20,67,92,39,37,26,79,68,16,76,57,83,92,46,75,99,26,64,39,72,65,37,93,65,5,53,62,36,13,97,14,38,85,33,76,56,99,29,64,84,28,19,91,92,55,33,88,32,70,38,53,76,1,76,35,26,75,18,18,7,88,19,53,65,22,91,20,85,15,13,72,82,13,31,75,62,68,4,56,91,89,56,10,46,63,7,74,50,15,85,87,64,77,12,95,10,66,77,51,6,61,75,91,75,85,61,78,4,97,99,4,90,34,89,44,44,68,89,30,20,70,24,22,81,22,77,61,33,89,2,11,75,50,85,13,43,56,78,73,49,27,38,78,56,90,17,94,72,51,5,55,67,32,19,81,81,45,83,18,96,33,75,53,4,29,87,80,33,57,78,80,43,68,57,71,83,10,18,98,70,36,61,31,73,33,69,24,78,76,43,88,96,16,14,91,43,66,15,98,44,48,68,57,72,48,49,89,62,31,55,83,68,86,97,16,25,87,13,74,40,82,43,48,85,40,45,72,33,60,84,4,47,96,19,92,75,73,46,6,69,4,81,98,89,48,55,89,24,64,31,47,50,93,72,47,72,36,79,7,24,66,60,65,18,81,93,40,37,36,62,94,48,8,77,21,82,22,65,20,46,85,47,52,70,55,74,19,65,15,72,81,57,67,46,94,21,16,94,84,36,43,62,82,48,47,79,5,96,39,58,85,80,31,7,98,23,69,22,99,37,69,35,66,36,70,3,69,47,6,64,38,69,42,57,91,89,21,89,13,42,78,24,44,79,74,65,63,85,10,50,71,94,26,78,55,5,26,71,46,20,83,96,51,87,2,99,83,5,38,86,8,13,94,61,93,39,67,23,60,74,87,57,30,72,23,19,95,57,93,83,58,34,83,35,4,47,81,88,24,87,34,93,79,70,18,24,73,98,76,77,24,93,18,66,56,87,25,29,7,7,97,40,61,56,96,96,1,42,21,92,73,11,10,97,69,58,93,2,82,27,96,7,84,44,67,57,63,13,79,56,72,34,89,26,94,24,86,99,71,73,98,26,89,10,98,5,64,70,85,32,61,35,67,0,0,21,21,1,10,1,0,0,0,0,0,0]);
    // let mut map = HashMap::new();
    let mut find_data = HashMap::new();
    let mut pos = Vec2::zero();
    let mut dir_code = 1;
    find_data.insert(pos, FindData::new_root());

    // let mut rect = Rect::new(0, 0, 0, 0);
    // let mut term = Term::stdout();

    let mut walkback = Vec::new();
    let mut done = false;
    let mut hit = false;
    let mut is_walkback = false;
    let mut steps = 0;
    let mut ox_data = None;
    while !done { 
        if let Some(res) = comp.run_to_out(|| {
            // for y in 0..=rect.h {
            //     for x in 0..=rect.w {
            //         let p = Vec2::new(rect.x + x, rect.y + y);
            //         if p == pos {
            //             print!("@");
            //         }
            //         else if find_data.contains_key(&p) {
            //             print!(".");
            //         }
            //         else {
            //             print!(" ");
            //         }
            //     }
            //     println!("");
            // }
            // println!("");
            // term.read_key();
            // rect.expand_to_contain_point(pos);

            is_walkback = false;
            if !walkback.is_empty() {
                is_walkback = true;
                return walkback.pop().unwrap() as i64;
            }
            let mut data = find_data.get_mut(&pos).unwrap();
            if hit {
                if !data.mark(dir_code) {
                    // walkback
                    while !data.has_dir() {
                        if data.is_root() {
                            done = true;
                            return 1;
                        }
                        walkback.push(data.back);
                        pos = FindData::calc_pos(pos, data.back);
                        data = find_data.get_mut(&pos).unwrap();
                    }
                    steps = data.steps;
                    walkback.reverse();
                    is_walkback = true;
                    return walkback.pop().unwrap() as i64;
                } else {
                    dir_code = data.next_dir().unwrap();
                    data.mark(dir_code);
                    steps = data.steps;
                }
            }
            else {
                steps = data.steps;
                data.mark(dir_code);
            }
            dir_code as i64
        }) {
            if is_walkback {
                assert_eq!(res, 1);
            }
            else {
                hit = false;
                let p = FindData::calc_pos(pos, dir_code);
                // rect.expand_to_contain_point(p);
                match res {
                    0 => {hit = true;},
                    1 => {pos = p;},
                    2 => {pos = p; ox_data = Some((steps + 1, pos));},
                    _ => panic!("invalid out!"),
                }
                if !hit {
                    steps = steps + 1;
                    find_data.entry(pos).and_modify(|n| n.came_from(dir_code, steps)).or_insert(FindData::new(dir_code, steps));
                }
            }
        }
        else {
            panic!("program halted!");
        }
    }

    let (steps, ox_pos) = ox_data.unwrap();
    println!("Day15a: {}", steps);
    assert_eq!(steps, 224);

    let mut ticks = 0;
    let mut current = vec![ox_pos];
    let mut next = Vec::new();

    let adjecent = vec![
        Vec2::new(1, 0),
        Vec2::new(-1, 0),
        Vec2::new(0, 1),
        Vec2::new(0, -1),
    ];
    while !current.is_empty() {
        ticks += 1;
        for p in current.iter() {
            for a in adjecent.iter() {
                let pos = p + a;
                if find_data.remove(&pos).is_some() {
                    next.push(pos);
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
        next.clear();
    }

    println!("Day15b: {}", ticks - 1);
}
// DAY 14 ---------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Reaction {
    name: String,
    count: usize,
    parts: Vec<(String, usize)>,
}

fn parse_reactions(file_name:&str) -> HashMap<String, Reaction> {
    let file = File::open(file_name).unwrap();
    let read = BufReader::new(file);
    let mut reactions = HashMap::new();
    for line in read.lines() {
        let line = line.unwrap();
        let mut r = line.split(' ');

        let mut parts = Vec::new();
        let mut more = true;
        while more {
            let count = r.next().unwrap().parse::<usize>().unwrap();
            let name = r.next().unwrap();
            more = name.ends_with(',');
            parts.push((name.trim_end_matches(',').to_string(), count));
        }
        assert_eq!(r.next(), Some("=>"));
        let count = r.next().unwrap().parse::<usize>().unwrap();
        let name = r.next().unwrap().to_string();
        reactions.insert(name.clone(), Reaction{name, count, parts});
    }
    reactions
}

// returns amount of ore used
fn solve_reaction_needs<'a>(reactions:&'a HashMap<String, Reaction>, needs:&mut HashMap<&'a str, usize>, store:&mut HashMap<&'a str, usize>) -> usize {
    let mut ore = 0;
    while !needs.is_empty() {
        let item = needs.keys().next().unwrap().clone();
        let (item, mut need) = needs.remove_entry(&item).unwrap();

        if let hash_map::Entry::Occupied(mut e) = store.entry(item.clone()) {
            let have = e.get_mut();
            if *have > need {
                *have -= need;
                need = 0;
            }
            else {
                need -= *have;
                e.remove_entry();
            }
        }
        if need == 0 {
            continue;
        }
        // didn't have what we needed so we need to create more.
        let reaction = reactions.get(item).unwrap();
        let times = if need % reaction.count == 0 {need / reaction.count} else {need / reaction.count + 1};
        let to_add = reaction.count * times - need;
        if to_add != 0 {
            *store.entry(item).or_insert(0) += to_add;
        }

        for (part, count) in reaction.parts.iter() {
            let count = *count * times;
            if part == "ORE" {
                ore += count;
            }
            else {
                *needs.entry(part).or_insert(0) += count;
            }
        }
    }
    ore
}

fn day14a() {
    let reactions = parse_reactions("data/14.txt");
    let mut needs = HashMap::new();
    let mut store = HashMap::new();
    needs.insert("FUEL", 1);
    let ore = solve_reaction_needs(&reactions, &mut needs, &mut store);
    println!("Day14a: {}", ore);
    // assert_eq!(ore, 751038);
}

fn day14b() {
    let reactions = parse_reactions("data/14.txt");
    let mut needs = HashMap::new();
    let mut store = HashMap::new();
    let mut fuel = 0;
    let mut next = 1;
    let mut ore_store = 1000000000000usize;
    let mut worst = None;
    
    while next != 0 {
        needs.insert("FUEL", next);
        let ore = solve_reaction_needs(&reactions, &mut needs, &mut store);
        worst = Some(worst.unwrap_or(ore));
        if ore_store > ore {
            fuel += next;
            ore_store -= ore;
            next = if ore_store < ore || ore == 0 {1} else {ore_store / ore};
        }
        else if next > 1 {
            next = ore_store / worst.unwrap();
        }
        else {
            break;
        }
    }
    println!("Day14b: {}", fuel);
}

// DAY 13 ---------------------------------------------------------------------------------------------

fn day13a() {
    let mut comp = Intcode::new(&[1,380,379,385,1008,2575,987010,381,1005,381,12,99,109,2576,1101,0,0,383,1102,0,1,382,21002,382,1,1,20102,1,383,2,21102,1,37,0,1105,1,578,4,382,4,383,204,1,1001,382,1,382,1007,382,44,381,1005,381,22,1001,383,1,383,1007,383,22,381,1005,381,18,1006,385,69,99,104,-1,104,0,4,386,3,384,1007,384,0,381,1005,381,94,107,0,384,381,1005,381,108,1106,0,161,107,1,392,381,1006,381,161,1102,-1,1,384,1106,0,119,1007,392,42,381,1006,381,161,1102,1,1,384,20102,1,392,1,21101,0,20,2,21101,0,0,3,21102,138,1,0,1106,0,549,1,392,384,392,20102,1,392,1,21102,1,20,2,21101,0,3,3,21101,161,0,0,1105,1,549,1101,0,0,384,20001,388,390,1,20101,0,389,2,21101,180,0,0,1106,0,578,1206,1,213,1208,1,2,381,1006,381,205,20001,388,390,1,20102,1,389,2,21101,0,205,0,1106,0,393,1002,390,-1,390,1101,0,1,384,20102,1,388,1,20001,389,391,2,21101,228,0,0,1105,1,578,1206,1,261,1208,1,2,381,1006,381,253,21002,388,1,1,20001,389,391,2,21101,0,253,0,1106,0,393,1002,391,-1,391,1102,1,1,384,1005,384,161,20001,388,390,1,20001,389,391,2,21102,1,279,0,1106,0,578,1206,1,316,1208,1,2,381,1006,381,304,20001,388,390,1,20001,389,391,2,21102,1,304,0,1105,1,393,1002,390,-1,390,1002,391,-1,391,1102,1,1,384,1005,384,161,21002,388,1,1,21001,389,0,2,21101,0,0,3,21101,338,0,0,1105,1,549,1,388,390,388,1,389,391,389,20101,0,388,1,21002,389,1,2,21101,4,0,3,21101,0,365,0,1105,1,549,1007,389,21,381,1005,381,75,104,-1,104,0,104,0,99,0,1,0,0,0,0,0,0,239,20,17,1,1,22,109,3,21201,-2,0,1,22101,0,-1,2,21102,0,1,3,21102,1,414,0,1106,0,549,22101,0,-2,1,22101,0,-1,2,21101,0,429,0,1106,0,601,1202,1,1,435,1,386,0,386,104,-1,104,0,4,386,1001,387,-1,387,1005,387,451,99,109,-3,2106,0,0,109,8,22202,-7,-6,-3,22201,-3,-5,-3,21202,-4,64,-2,2207,-3,-2,381,1005,381,492,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,481,21202,-4,8,-2,2207,-3,-2,381,1005,381,518,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,507,2207,-3,-4,381,1005,381,540,21202,-4,-1,-1,22201,-3,-1,-3,2207,-3,-4,381,1006,381,529,22102,1,-3,-7,109,-8,2105,1,0,109,4,1202,-2,44,566,201,-3,566,566,101,639,566,566,2101,0,-1,0,204,-3,204,-2,204,-1,109,-4,2106,0,0,109,3,1202,-1,44,593,201,-2,593,593,101,639,593,593,21001,0,0,-2,109,-3,2106,0,0,109,3,22102,22,-2,1,22201,1,-1,1,21102,1,487,2,21101,744,0,3,21101,968,0,4,21102,1,630,0,1106,0,456,21201,1,1607,-2,109,-3,2105,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,2,2,0,0,2,0,2,2,2,2,0,0,2,0,2,2,2,2,2,2,0,0,0,0,0,0,0,2,0,2,2,2,0,0,0,0,0,0,0,1,1,0,0,0,2,0,0,2,2,0,0,2,0,2,0,2,2,2,0,2,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,2,2,2,0,2,2,0,0,1,1,0,0,0,2,0,2,2,2,2,0,0,0,0,2,2,0,2,0,2,0,2,2,2,2,0,2,2,2,0,0,2,2,0,0,2,0,0,2,2,2,2,0,1,1,0,2,2,0,2,0,2,2,2,2,0,0,2,2,0,0,0,0,0,0,0,2,2,2,2,2,0,2,2,0,2,2,2,2,0,0,0,2,0,0,2,0,1,1,0,0,2,0,0,2,0,0,0,0,0,2,0,0,2,2,0,0,0,0,0,2,0,0,0,2,2,2,2,2,0,0,0,0,0,2,2,2,0,0,0,0,1,1,0,2,0,2,2,0,2,0,0,0,0,0,0,2,0,2,0,0,0,0,0,2,0,0,2,2,0,0,0,0,2,2,0,0,0,2,2,0,0,2,0,0,1,1,0,0,0,2,2,2,0,2,0,2,0,0,2,0,0,2,0,0,0,2,0,0,2,0,2,0,0,0,0,2,0,0,2,2,0,2,2,0,0,2,0,0,1,1,0,0,2,0,2,0,2,0,2,2,0,0,2,2,0,0,0,0,0,2,0,2,0,0,0,0,0,0,2,0,0,2,0,0,0,0,0,0,0,0,2,0,1,1,0,0,2,0,0,0,0,2,2,0,0,0,0,0,0,0,0,0,0,0,2,0,0,2,0,2,2,0,0,2,2,2,0,0,2,2,2,2,2,0,0,0,1,1,0,2,0,2,0,0,0,2,0,0,0,2,2,0,0,2,0,0,0,0,0,0,0,2,2,0,2,2,0,2,0,2,0,0,0,2,2,0,0,0,0,0,1,1,0,2,2,0,0,0,2,2,0,0,0,0,2,0,0,2,0,2,2,2,0,0,0,2,0,0,2,2,2,2,2,0,0,0,0,0,0,0,2,2,0,0,1,1,0,0,0,2,2,2,2,0,2,2,2,0,0,2,2,2,2,0,0,2,0,2,0,0,0,0,2,2,2,2,2,2,0,2,2,2,0,0,2,2,2,0,1,1,0,2,0,2,2,0,2,0,0,2,0,2,2,0,0,0,2,0,0,0,0,0,2,2,2,0,0,2,0,2,2,0,0,2,2,2,0,2,2,0,0,0,1,1,0,0,2,0,0,2,2,0,0,2,2,2,0,0,0,0,0,2,2,0,0,0,2,0,2,0,0,0,0,2,2,0,0,0,0,0,2,0,0,2,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,57,82,58,90,4,73,33,23,24,29,55,37,9,40,59,69,10,33,96,78,20,58,37,47,72,45,47,56,93,19,16,83,93,11,32,43,50,31,7,12,5,78,48,96,41,7,16,29,13,73,66,66,92,19,23,43,96,4,16,93,75,60,8,73,91,76,9,88,82,70,92,77,46,8,65,93,47,32,89,1,36,54,7,32,62,28,7,45,82,37,84,88,54,9,73,91,65,94,81,31,32,88,16,47,10,57,20,80,81,63,4,37,98,58,78,66,58,40,47,33,27,4,21,36,55,38,6,5,12,71,20,85,7,26,28,55,12,69,86,5,9,9,32,12,47,65,98,1,48,40,31,84,67,67,43,97,52,62,20,91,76,31,75,98,55,35,43,69,37,46,5,51,96,10,49,91,32,44,96,63,25,80,90,47,58,37,81,82,30,18,90,69,80,85,46,68,91,78,97,54,44,18,38,78,11,21,1,55,66,4,55,52,74,53,61,6,70,43,86,97,97,36,98,58,18,14,29,84,13,78,60,57,20,55,18,82,82,84,83,24,18,15,43,39,55,61,29,2,15,45,7,51,9,26,88,51,70,11,20,21,74,23,60,76,14,16,42,80,45,81,49,4,29,2,45,6,13,54,48,91,78,7,88,27,82,10,28,70,46,56,93,3,26,19,41,46,34,39,43,22,8,48,13,68,38,50,70,1,17,78,16,46,86,32,47,83,46,47,63,61,22,10,45,6,97,52,88,54,10,47,28,47,79,31,10,89,79,44,83,77,23,85,46,76,68,20,13,40,94,51,40,75,65,69,88,76,13,94,25,38,68,40,96,25,26,9,7,51,17,84,24,90,94,13,62,27,28,4,55,80,2,86,83,76,73,62,66,90,71,54,26,78,36,6,66,58,89,42,44,94,92,46,91,68,98,38,37,49,28,74,80,23,3,42,42,90,1,38,52,59,48,65,50,98,90,39,21,16,20,25,57,36,53,49,79,79,14,85,39,9,24,58,9,19,72,70,2,10,43,7,58,66,84,70,29,18,97,76,16,56,91,1,32,23,89,20,96,27,40,71,35,42,79,80,61,97,78,2,81,6,51,87,23,47,73,84,57,16,87,42,66,79,33,28,30,34,16,92,22,60,68,34,26,47,50,52,32,80,18,40,48,59,23,24,80,49,14,61,93,66,35,14,68,52,24,21,7,27,65,57,63,91,18,93,7,84,56,51,31,38,28,90,52,5,61,37,81,44,17,79,63,50,54,56,58,7,39,96,80,27,53,73,27,77,59,37,37,24,30,22,47,57,84,76,27,20,1,16,83,96,82,40,55,44,77,1,60,59,94,8,12,40,75,94,65,40,34,35,13,69,46,79,52,73,23,79,25,73,81,75,33,94,57,45,92,41,82,76,86,5,34,16,47,42,61,25,70,52,54,28,12,57,3,61,80,50,65,42,94,97,97,65,50,89,94,7,21,1,21,68,69,75,13,2,64,67,32,85,73,72,7,49,43,92,59,90,4,12,98,28,53,36,97,53,11,45,21,24,74,11,85,3,11,47,54,5,47,22,98,18,30,82,1,79,59,3,27,25,70,6,79,94,85,17,58,4,23,33,64,40,56,43,14,77,98,75,13,33,45,12,22,6,46,33,48,62,77,50,40,65,88,8,50,43,67,41,2,74,81,44,66,59,52,86,51,35,4,24,58,56,85,57,58,81,41,24,63,73,80,21,63,90,69,94,36,26,85,12,86,64,1,5,35,58,36,29,75,82,15,18,63,73,16,4,62,53,30,91,85,42,46,13,57,53,24,93,91,28,10,19,94,44,82,24,57,24,85,23,6,34,83,83,63,84,65,51,72,54,85,70,40,26,76,76,31,19,93,65,25,63,88,10,3,53,62,31,12,39,42,18,23,26,27,27,56,9,82,50,86,23,5,44,24,86,62,31,6,59,70,53,29,67,82,41,51,51,39,47,14,26,88,5,51,88,57,36,13,19,43,11,80,30,39,50,35,91,91,92,57,28,9,6,29,53,51,59,4,60,86,94,16,78,34,2,37,8,34,61,36,50,94,28,74,6,58,37,59,98,79,89,74,96,19,27,40,13,50,72,32,10,87,38,75,25,40,52,36,64,77,15,6,5,16,25,67,57,94,24,4,8,31,73,36,47,28,23,14,77,94,9,79,44,45,4,98,54,47,28,987010]);
    let mut blocks = HashSet::new();
    while let (true, ops) = comp.run_to_sized_out(3, || unimplemented!()) {
        let pos = Vec2::from_slice(&ops[0..2]);
        if ops[2] == 2 {
            blocks.insert(pos);
        }
        else {
            blocks.remove(&pos);
        }
    }
    println!("Day13a: {}", blocks.len());
}

fn day13b() {
let mut comp = Intcode::new(&[2,380,379,385,1008,2575,987010,381,1005,381,12,99,109,2576,1101,0,0,383,1102,0,1,382,21002,382,1,1,20102,1,383,2,21102,1,37,0,1105,1,578,4,382,4,383,204,1,1001,382,1,382,1007,382,44,381,1005,381,22,1001,383,1,383,1007,383,22,381,1005,381,18,1006,385,69,99,104,-1,104,0,4,386,3,384,1007,384,0,381,1005,381,94,107,0,384,381,1005,381,108,1106,0,161,107,1,392,381,1006,381,161,1102,-1,1,384,1106,0,119,1007,392,42,381,1006,381,161,1102,1,1,384,20102,1,392,1,21101,0,20,2,21101,0,0,3,21102,138,1,0,1106,0,549,1,392,384,392,20102,1,392,1,21102,1,20,2,21101,0,3,3,21101,161,0,0,1105,1,549,1101,0,0,384,20001,388,390,1,20101,0,389,2,21101,180,0,0,1106,0,578,1206,1,213,1208,1,2,381,1006,381,205,20001,388,390,1,20102,1,389,2,21101,0,205,0,1106,0,393,1002,390,-1,390,1101,0,1,384,20102,1,388,1,20001,389,391,2,21101,228,0,0,1105,1,578,1206,1,261,1208,1,2,381,1006,381,253,21002,388,1,1,20001,389,391,2,21101,0,253,0,1106,0,393,1002,391,-1,391,1102,1,1,384,1005,384,161,20001,388,390,1,20001,389,391,2,21102,1,279,0,1106,0,578,1206,1,316,1208,1,2,381,1006,381,304,20001,388,390,1,20001,389,391,2,21102,1,304,0,1105,1,393,1002,390,-1,390,1002,391,-1,391,1102,1,1,384,1005,384,161,21002,388,1,1,21001,389,0,2,21101,0,0,3,21101,338,0,0,1105,1,549,1,388,390,388,1,389,391,389,20101,0,388,1,21002,389,1,2,21101,4,0,3,21101,0,365,0,1105,1,549,1007,389,21,381,1005,381,75,104,-1,104,0,104,0,99,0,1,0,0,0,0,0,0,239,20,17,1,1,22,109,3,21201,-2,0,1,22101,0,-1,2,21102,0,1,3,21102,1,414,0,1106,0,549,22101,0,-2,1,22101,0,-1,2,21101,0,429,0,1106,0,601,1202,1,1,435,1,386,0,386,104,-1,104,0,4,386,1001,387,-1,387,1005,387,451,99,109,-3,2106,0,0,109,8,22202,-7,-6,-3,22201,-3,-5,-3,21202,-4,64,-2,2207,-3,-2,381,1005,381,492,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,481,21202,-4,8,-2,2207,-3,-2,381,1005,381,518,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,507,2207,-3,-4,381,1005,381,540,21202,-4,-1,-1,22201,-3,-1,-3,2207,-3,-4,381,1006,381,529,22102,1,-3,-7,109,-8,2105,1,0,109,4,1202,-2,44,566,201,-3,566,566,101,639,566,566,2101,0,-1,0,204,-3,204,-2,204,-1,109,-4,2106,0,0,109,3,1202,-1,44,593,201,-2,593,593,101,639,593,593,21001,0,0,-2,109,-3,2106,0,0,109,3,22102,22,-2,1,22201,1,-1,1,21102,1,487,2,21101,744,0,3,21101,968,0,4,21102,1,630,0,1106,0,456,21201,1,1607,-2,109,-3,2105,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,2,2,0,0,2,0,2,2,2,2,0,0,2,0,2,2,2,2,2,2,0,0,0,0,0,0,0,2,0,2,2,2,0,0,0,0,0,0,0,1,1,0,0,0,2,0,0,2,2,0,0,2,0,2,0,2,2,2,0,2,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,2,2,2,0,2,2,0,0,1,1,0,0,0,2,0,2,2,2,2,0,0,0,0,2,2,0,2,0,2,0,2,2,2,2,0,2,2,2,0,0,2,2,0,0,2,0,0,2,2,2,2,0,1,1,0,2,2,0,2,0,2,2,2,2,0,0,2,2,0,0,0,0,0,0,0,2,2,2,2,2,0,2,2,0,2,2,2,2,0,0,0,2,0,0,2,0,1,1,0,0,2,0,0,2,0,0,0,0,0,2,0,0,2,2,0,0,0,0,0,2,0,0,0,2,2,2,2,2,0,0,0,0,0,2,2,2,0,0,0,0,1,1,0,2,0,2,2,0,2,0,0,0,0,0,0,2,0,2,0,0,0,0,0,2,0,0,2,2,0,0,0,0,2,2,0,0,0,2,2,0,0,2,0,0,1,1,0,0,0,2,2,2,0,2,0,2,0,0,2,0,0,2,0,0,0,2,0,0,2,0,2,0,0,0,0,2,0,0,2,2,0,2,2,0,0,2,0,0,1,1,0,0,2,0,2,0,2,0,2,2,0,0,2,2,0,0,0,0,0,2,0,2,0,0,0,0,0,0,2,0,0,2,0,0,0,0,0,0,0,0,2,0,1,1,0,0,2,0,0,0,0,2,2,0,0,0,0,0,0,0,0,0,0,0,2,0,0,2,0,2,2,0,0,2,2,2,0,0,2,2,2,2,2,0,0,0,1,1,0,2,0,2,0,0,0,2,0,0,0,2,2,0,0,2,0,0,0,0,0,0,0,2,2,0,2,2,0,2,0,2,0,0,0,2,2,0,0,0,0,0,1,1,0,2,2,0,0,0,2,2,0,0,0,0,2,0,0,2,0,2,2,2,0,0,0,2,0,0,2,2,2,2,2,0,0,0,0,0,0,0,2,2,0,0,1,1,0,0,0,2,2,2,2,0,2,2,2,0,0,2,2,2,2,0,0,2,0,2,0,0,0,0,2,2,2,2,2,2,0,2,2,2,0,0,2,2,2,0,1,1,0,2,0,2,2,0,2,0,0,2,0,2,2,0,0,0,2,0,0,0,0,0,2,2,2,0,0,2,0,2,2,0,0,2,2,2,0,2,2,0,0,0,1,1,0,0,2,0,0,2,2,0,0,2,2,2,0,0,0,0,0,2,2,0,0,0,2,0,2,0,0,0,0,2,2,0,0,0,0,0,2,0,0,2,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,57,82,58,90,4,73,33,23,24,29,55,37,9,40,59,69,10,33,96,78,20,58,37,47,72,45,47,56,93,19,16,83,93,11,32,43,50,31,7,12,5,78,48,96,41,7,16,29,13,73,66,66,92,19,23,43,96,4,16,93,75,60,8,73,91,76,9,88,82,70,92,77,46,8,65,93,47,32,89,1,36,54,7,32,62,28,7,45,82,37,84,88,54,9,73,91,65,94,81,31,32,88,16,47,10,57,20,80,81,63,4,37,98,58,78,66,58,40,47,33,27,4,21,36,55,38,6,5,12,71,20,85,7,26,28,55,12,69,86,5,9,9,32,12,47,65,98,1,48,40,31,84,67,67,43,97,52,62,20,91,76,31,75,98,55,35,43,69,37,46,5,51,96,10,49,91,32,44,96,63,25,80,90,47,58,37,81,82,30,18,90,69,80,85,46,68,91,78,97,54,44,18,38,78,11,21,1,55,66,4,55,52,74,53,61,6,70,43,86,97,97,36,98,58,18,14,29,84,13,78,60,57,20,55,18,82,82,84,83,24,18,15,43,39,55,61,29,2,15,45,7,51,9,26,88,51,70,11,20,21,74,23,60,76,14,16,42,80,45,81,49,4,29,2,45,6,13,54,48,91,78,7,88,27,82,10,28,70,46,56,93,3,26,19,41,46,34,39,43,22,8,48,13,68,38,50,70,1,17,78,16,46,86,32,47,83,46,47,63,61,22,10,45,6,97,52,88,54,10,47,28,47,79,31,10,89,79,44,83,77,23,85,46,76,68,20,13,40,94,51,40,75,65,69,88,76,13,94,25,38,68,40,96,25,26,9,7,51,17,84,24,90,94,13,62,27,28,4,55,80,2,86,83,76,73,62,66,90,71,54,26,78,36,6,66,58,89,42,44,94,92,46,91,68,98,38,37,49,28,74,80,23,3,42,42,90,1,38,52,59,48,65,50,98,90,39,21,16,20,25,57,36,53,49,79,79,14,85,39,9,24,58,9,19,72,70,2,10,43,7,58,66,84,70,29,18,97,76,16,56,91,1,32,23,89,20,96,27,40,71,35,42,79,80,61,97,78,2,81,6,51,87,23,47,73,84,57,16,87,42,66,79,33,28,30,34,16,92,22,60,68,34,26,47,50,52,32,80,18,40,48,59,23,24,80,49,14,61,93,66,35,14,68,52,24,21,7,27,65,57,63,91,18,93,7,84,56,51,31,38,28,90,52,5,61,37,81,44,17,79,63,50,54,56,58,7,39,96,80,27,53,73,27,77,59,37,37,24,30,22,47,57,84,76,27,20,1,16,83,96,82,40,55,44,77,1,60,59,94,8,12,40,75,94,65,40,34,35,13,69,46,79,52,73,23,79,25,73,81,75,33,94,57,45,92,41,82,76,86,5,34,16,47,42,61,25,70,52,54,28,12,57,3,61,80,50,65,42,94,97,97,65,50,89,94,7,21,1,21,68,69,75,13,2,64,67,32,85,73,72,7,49,43,92,59,90,4,12,98,28,53,36,97,53,11,45,21,24,74,11,85,3,11,47,54,5,47,22,98,18,30,82,1,79,59,3,27,25,70,6,79,94,85,17,58,4,23,33,64,40,56,43,14,77,98,75,13,33,45,12,22,6,46,33,48,62,77,50,40,65,88,8,50,43,67,41,2,74,81,44,66,59,52,86,51,35,4,24,58,56,85,57,58,81,41,24,63,73,80,21,63,90,69,94,36,26,85,12,86,64,1,5,35,58,36,29,75,82,15,18,63,73,16,4,62,53,30,91,85,42,46,13,57,53,24,93,91,28,10,19,94,44,82,24,57,24,85,23,6,34,83,83,63,84,65,51,72,54,85,70,40,26,76,76,31,19,93,65,25,63,88,10,3,53,62,31,12,39,42,18,23,26,27,27,56,9,82,50,86,23,5,44,24,86,62,31,6,59,70,53,29,67,82,41,51,51,39,47,14,26,88,5,51,88,57,36,13,19,43,11,80,30,39,50,35,91,91,92,57,28,9,6,29,53,51,59,4,60,86,94,16,78,34,2,37,8,34,61,36,50,94,28,74,6,58,37,59,98,79,89,74,96,19,27,40,13,50,72,32,10,87,38,75,25,40,52,36,64,77,15,6,5,16,25,67,57,94,24,4,8,31,73,36,47,28,23,14,77,94,9,79,44,45,4,98,54,47,28,987010]);
    let mut objects = HashMap::new();
    let mut rect = Rect::new(0, 0, 0, 0);
    let mut score = 0;
    // let mut term = Term::stdout();
    let mut last_paddle = Vec2::zero();
    let mut last_ball = Vec2::zero();
    while let (true, ops) = comp.run_to_sized_out(3, || {

        // println!("Score: {}", score);
        // for y in 0..=rect.h {
        //     for x in 0..=rect.w {
        //         let pos = Vec2::new(rect.x + x, rect.y + y);
        //         let c = objects.get(&pos).unwrap_or(&' ');
        //         print!("{}", c);
        //     }
        //     println!("");
        // }
        // match term.read_key() {
        //     Ok(Key::ArrowLeft) => return -1,
        //     Ok(Key::ArrowRight) => return 1,
        //     Ok(Key::ArrowDown) => return 0,
        //     _ => {},
        // }
        
        if last_ball.x > last_paddle.x {
            return 1;
        }
        if last_ball.x == last_paddle.x {
            return 0;
        }
        return -1;
        
    }) {
        let pos = Vec2::from_slice(&ops[0..2]);
        if pos == Vec2::new(-1, 0) {
            score = ops[2];
        }
        else {
            rect.expand_to_contain_point(pos);
            match ops[2] {
                0 => {objects.remove(&pos);},
                1 => {objects.insert(pos, '█');},
                2 => {objects.insert(pos, '#');},
                3 => {objects.insert(pos, '▀'); last_paddle = pos;},
                4 => {objects.insert(pos, 'o'); last_ball = pos;},
                _ => panic!("invalide obj type!"),
            }
        }
    }
    println!("Day13b: {}", score);
    assert_eq!(score, 12099);
}

// DAY 12 ---------------------------------------------------------------------------------------------

fn lcm(l:usize, r:usize) -> usize {
    let gcd = |mut x, mut y| {
        while y != 0 {
            let t = y;
            y = x % y;
            x = t;
        }
        x
    };
    l * (r / gcd(l, r))
}

#[derive(Clone,Debug, Eq, PartialEq)]
struct SimpleBody {
    pos: Vec3i,
    vel: Vec3i,
}

impl SimpleBody {
    fn new(pos: Vec3i) -> Self {
        SimpleBody{
            pos,
            vel: Vec3i::zero(),
        }
    }

    fn step(&mut self) {
        self.pos += self.vel;
    }

    fn do_gravity(&mut self, other:&mut Self) {
        let mut diff = self.pos - other.pos;
        diff.apply(|v| if v > 0 {1} else if v < 0 {-1} else {0});
        self.vel -= diff;
        other.vel += diff;
    }

    fn tick(bodies:&mut [Self]) {
        for i in 0..(bodies.len() - 1) {
            for n in (i + 1)..bodies.len() {
                let (l, r) = bodies.split_at_mut(n);
                l[i].do_gravity(&mut r[0]);
            }
        }
        for b in bodies.iter_mut() {
            b.step();
        }
    }
}

fn day12a() {
    let mut bodies = [
        SimpleBody::new(Vec3i::new(-7, -1, 6)),
        SimpleBody::new(Vec3i::new(6, -9, -9)),
        SimpleBody::new(Vec3i::new(-12, 2, -7)),
        SimpleBody::new(Vec3i::new(4, -17, -12)),
    ];
    for _ in 0..1000 {
        SimpleBody::tick(&mut bodies);
    }
    let mut tot = 0i32;
    for b in bodies.iter_mut() {
        let pot:i32 = b.pos.iter().map(|v| v.abs()).sum();
        let kin:i32 = b.vel.iter().map(|v| v.abs()).sum();
        tot += pot * kin;
    }
    println!("Day12a: {}", tot);
    assert_eq!(tot, 11384);
}

fn day12b() {
    let mut bodies = [
        SimpleBody::new(Vec3i::new(-7, -1, 6)),
        SimpleBody::new(Vec3i::new(6, -9, -9)),
        SimpleBody::new(Vec3i::new(-12, 2, -7)),
        SimpleBody::new(Vec3i::new(4, -17, -12)),
    ];
    let start = bodies.clone();
    let mut steps = 0usize;
    let mut axis_cycle = Vec3::<usize>::zero();
    'main: loop {
        steps += 1;
        SimpleBody::tick(&mut bodies);

        let mut eq = Vec3::new(true, true, true);
        for (s, b) in bodies.iter().zip(start.iter()) {
            let p_eq = s.pos.map2(b.pos, |s, b| s == b);
            let v_eq = s.vel.map2(b.vel, |s, b| s == b);
            eq = eq & p_eq & v_eq;
        }
        let mut done = true;
        axis_cycle.apply2(eq, |v, eq| {
            if v != 0 {return v;}
            done = false;
            if eq {steps} else {0}
        });
        if done {
            break 'main;
        }
    }
    let v = lcm(axis_cycle.x, axis_cycle.y);
    let res = lcm(v, axis_cycle.z);
    println!("Day12b: {}", res);
    assert_eq!(res, 452582583272768);
}

// DAY 11 ---------------------------------------------------------------------------------------------

fn day11a() {
    let mut comp = Intcode::new(&[3,8,1005,8,330,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,29,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,51,1,1103,2,10,1006,0,94,1006,0,11,1,1106,13,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,87,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,109,2,1105,5,10,2,103,16,10,1,1103,12,10,2,105,2,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,146,1006,0,49,2,1,12,10,2,1006,6,10,1,1101,4,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,183,1,6,9,10,1006,0,32,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,213,2,1101,9,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,239,1006,0,47,1006,0,4,2,6,0,10,1006,0,58,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,274,2,1005,14,10,1006,0,17,1,104,20,10,1006,0,28,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,309,101,1,9,9,1007,9,928,10,1005,10,15,99,109,652,104,0,104,1,21101,0,937263411860,1,21102,347,1,0,1105,1,451,21101,932440724376,0,1,21102,1,358,0,1105,1,451,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,29015167015,1,21101,0,405,0,1106,0,451,21102,1,3422723163,1,21101,0,416,0,1106,0,451,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,868389376360,1,21101,0,439,0,1105,1,451,21102,825544712960,1,1,21102,1,450,0,1106,0,451,99,109,2,21201,-1,0,1,21101,0,40,2,21102,482,1,3,21102,1,472,0,1106,0,515,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,477,478,493,4,0,1001,477,1,477,108,4,477,10,1006,10,509,1101,0,0,477,109,-2,2106,0,0,0,109,4,2101,0,-1,514,1207,-3,0,10,1006,10,532,21102,1,0,-3,22101,0,-3,1,22102,1,-2,2,21102,1,1,3,21101,551,0,0,1106,0,556,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,579,2207,-4,-2,10,1006,10,579,22102,1,-4,-4,1106,0,647,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,1,598,0,1106,0,556,22101,0,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,617,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,639,21201,-1,0,1,21102,639,1,0,105,1,514,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0]);
    let mut white_panels = HashSet::new();
    let mut painted = HashSet::new();
    let mut pos = Vec2::new(0, 0);
    let mut dir = Vec2::new(0, -1);
    let mut state = true;

    // let mut last_paint = 0;
    // let mut skip = 0;
    // let mut rect = Rect::new(0, 0, 0, 0);
    // let mut steps = 0;

    while let Some(op) = comp.run_to_out(|| if white_panels.contains(&pos) {1} else {0}) {
        match state {
            true => { // paint
                painted.insert(pos);
                if op == 0 {
                    white_panels.remove(&pos);
                }
                else {
                    white_panels.insert(pos);
                }
                // last_paint = op;
            },
            false => {
                // rect.expand_to_contain_point(pos);
                // steps += 1;
                // if skip > 0 {skip -= 1;}
                // else {
                //     for y in 0..=rect.h {
                //         for x in 0..=rect.w {
                //             let p = Vec2::new(rect.x + x, rect.y + y);
                //             if pos == p {
                //                 match dir.into_tuple() {
                //                     (1, 0) => print!(">"),
                //                     (-1, 0) => print!("<"),
                //                     (0, -1) => print!("^"),
                //                     (0, 1) => print!("v"),
                //                     _ => panic!("invalid dir"),
                //                 }
                //             }
                //             else if white_panels.contains(&p) {print!("#")} else {print!(".")};
                //         }
                //         println!("");
                //     }
                //     let mut buf = String::new();
                //     let _ = io::stdin().read_line(&mut buf).unwrap();
                //     if let Ok(v) = usize::from_str_radix(&buf[..buf.len()-2], 10) {
                //         skip = v;
                //     }
                //     println!("{} {}", op, last_paint);
                // }

                dir = if op == 0 { Vec2::new(dir.y, -dir.x) } else { Vec2::new(-dir.y, dir.x) };
                pos += dir;
            }
        }
        state = !state;
    }
    println!("Day11a: {}", painted.len());
}

fn day11b() {
    let mut comp = Intcode::new(&[3,8,1005,8,330,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,29,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,51,1,1103,2,10,1006,0,94,1006,0,11,1,1106,13,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,87,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,109,2,1105,5,10,2,103,16,10,1,1103,12,10,2,105,2,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,146,1006,0,49,2,1,12,10,2,1006,6,10,1,1101,4,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,183,1,6,9,10,1006,0,32,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,213,2,1101,9,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,239,1006,0,47,1006,0,4,2,6,0,10,1006,0,58,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,274,2,1005,14,10,1006,0,17,1,104,20,10,1006,0,28,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,309,101,1,9,9,1007,9,928,10,1005,10,15,99,109,652,104,0,104,1,21101,0,937263411860,1,21102,347,1,0,1105,1,451,21101,932440724376,0,1,21102,1,358,0,1105,1,451,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,29015167015,1,21101,0,405,0,1106,0,451,21102,1,3422723163,1,21101,0,416,0,1106,0,451,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,868389376360,1,21101,0,439,0,1105,1,451,21102,825544712960,1,1,21102,1,450,0,1106,0,451,99,109,2,21201,-1,0,1,21101,0,40,2,21102,482,1,3,21102,1,472,0,1106,0,515,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,477,478,493,4,0,1001,477,1,477,108,4,477,10,1006,10,509,1101,0,0,477,109,-2,2106,0,0,0,109,4,2101,0,-1,514,1207,-3,0,10,1006,10,532,21102,1,0,-3,22101,0,-3,1,22102,1,-2,2,21102,1,1,3,21101,551,0,0,1106,0,556,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,579,2207,-4,-2,10,1006,10,579,22102,1,-4,-4,1106,0,647,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,1,598,0,1106,0,556,22101,0,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,617,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,639,21201,-1,0,1,21102,639,1,0,105,1,514,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0]);
    let mut white_panels = HashSet::new();
    let mut pos = Vec2::new(0, 0);
    let mut dir = Vec2::new(0, -1);
    let mut state = true;
    white_panels.insert(pos);


    while let Some(op) = comp.run_to_out(|| if white_panels.contains(&pos) {1} else {0}) {
        match state {
            true => { // paint
                if op == 0 {
                    white_panels.remove(&pos);
                }
                else {
                    white_panels.insert(pos);
                }
            },
            false => {
                dir = if op == 0 { Vec2::new(dir.y, -dir.x) } else { Vec2::new(-dir.y, dir.x) };
                pos += dir;
            }
        }
        state = !state;
    }
    println!("Day11a: RPJCFZKF (num white panels: {})", white_panels.len());

    let mut rect = Rect::new(0, 0, 0, 0);
    white_panels.iter().for_each(|p| rect.expand_to_contain_point(*p));

    for y in 0..=rect.h {
        for x in 0..=rect.w {
            let p = Vec2::new(rect.x + x, rect.y + y);
            if white_panels.contains(&p) {print!("#")} else {print!(".")};
        }
        println!("");
    }
    assert_eq!(white_panels.len(), 91);
}

// DAY 10 ---------------------------------------------------------------------------------------------

fn reduce_fraction(frac:Vec2<i32>) -> Vec2<i32> {
    frac / gcd(frac.x, frac.y)
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x.abs();
    let mut y = y.abs();
    if x == 0 {return y}
    if y == 0 {return x}
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn read_asteroids(file_name:&str) -> (usize, usize, Vec<Vec2<i32>>) {
    let file = File::open(file_name).unwrap();
    let mut asteroids = Vec::new();
    let read = BufReader::new(file);
    let mut w = 0;
    let mut y = 0;
    for line in read.lines().map(|v| v.unwrap()) {
        if w != 0 {
            assert_eq!(line.len(), w);
        } 
        else {
            w = line.len();
        }
        asteroids.extend(line.bytes().enumerate().filter_map(|(idx, c)| if c == b'#' {Some(Vec2::new(idx as i32, y))} else {None}));
        y += 1;
    }
    (w, y as usize, asteroids)
}

fn day10a() {
    let (_, _, asteroids) = read_asteroids("data/10.txt");
    let mut fractions = HashSet::new();
    let mut best_pos = Vec2::zero();
    let mut best_count = 0;
    for pos in asteroids.iter() {
        fractions.clear();

        for o_pos in asteroids.iter() {
            if o_pos != pos {
                fractions.insert(reduce_fraction(o_pos - pos));
            }
        }
        if best_count < fractions.len() {
            best_count = fractions.len();
            best_pos = *pos;
        }
    }
    println!("Day10a: {} {:?}", best_count, best_pos);
    assert_eq!(best_count, 299);
    assert_eq!(best_pos, Vec2::new(26, 29));
}

fn day10b() {
    let (_, _, asteroids) = read_asteroids("data/10.txt");
    let mut fractions = BTreeMap::<NotNan<f32>, BTreeMap<i32, Vec2<i32>>>::new();
    let pos = Vec2::new(26, 29);
    let to_angle = |vec| {
        let vec = reduce_fraction(vec); // to ensure we get the same angle
        let vec = Vec2::new(vec.x as f32, vec.y as f32);
        let ang = (-vec.x).atan2(vec.y);
        NotNan::new(ang).unwrap()
    };
    for o_pos in asteroids.iter() {
        if *o_pos != pos {
            let diff = o_pos - pos;
            fractions.entry(to_angle(diff)).or_insert(BTreeMap::new()).insert(diff.magnitude_squared(), *o_pos);
            // println!("Added {:?} {} | {} {:?}", diff, to_angle(diff), diff.magnitude_squared(), *o_pos);
        }
    }
    let mut count = 0;
    while !fractions.is_empty() || count > 200 {
        let last = count;
        for (_, m) in fractions.iter_mut() { // whould have used retain but BTrees doesn't have it
           if let Some((k, v)) = m.iter_mut().next() {
               if count == 199 {
                    println!("Day10b: {}", v.x * 100 + v.y);
                    assert_eq!(*v, Vec2::new(14, 19));
                    return;
               }
               count += 1;
            //    println!("Remove {:?} {}", v, count);
               let k = *k;
               m.remove(&k);
           }
        }
        assert_ne!(count, last); // infinite loop
    }
}

// DAY 9 ---------------------------------------------------------------------------------------------

fn day9a() {
    let mut comp = Intcode::new_sized(2000, &[1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1102,1,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,902,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1102,1,37,1007,1102,24,1,1006,1102,26,1,1012,1101,528,0,1023,1102,256,1,1027,1102,466,1,1029,1102,1,629,1024,1101,0,620,1025,1101,0,0,1020,1102,1,30,1004,1101,39,0,1003,1102,36,1,1005,1102,531,1,1022,1102,32,1,1019,1101,0,27,1000,1101,0,28,1016,1101,1,0,1021,1101,23,0,1013,1102,1,25,1015,1102,1,21,1008,1102,1,22,1018,1102,1,34,1014,1102,475,1,1028,1101,33,0,1002,1101,0,35,1011,1102,1,20,1009,1102,38,1,1017,1101,259,0,1026,1101,31,0,1010,1101,0,29,1001,109,8,21102,40,1,10,1008,1018,40,63,1005,63,203,4,187,1105,1,207,1001,64,1,64,1002,64,2,64,109,7,21108,41,41,0,1005,1015,225,4,213,1106,0,229,1001,64,1,64,1002,64,2,64,109,1,1205,5,247,4,235,1001,64,1,64,1105,1,247,1002,64,2,64,109,20,2106,0,-9,1105,1,265,4,253,1001,64,1,64,1002,64,2,64,109,-38,1202,4,1,63,1008,63,33,63,1005,63,291,4,271,1001,64,1,64,1106,0,291,1002,64,2,64,109,6,2102,1,0,63,1008,63,29,63,1005,63,315,1001,64,1,64,1106,0,317,4,297,1002,64,2,64,109,10,21102,42,1,5,1008,1019,40,63,1005,63,341,1001,64,1,64,1105,1,343,4,323,1002,64,2,64,109,-13,2101,0,5,63,1008,63,24,63,1005,63,365,4,349,1105,1,369,1001,64,1,64,1002,64,2,64,109,7,1202,-6,1,63,1008,63,36,63,1005,63,389,1105,1,395,4,375,1001,64,1,64,1002,64,2,64,109,1,2107,31,-5,63,1005,63,411,1106,0,417,4,401,1001,64,1,64,1002,64,2,64,109,3,1206,8,431,4,423,1105,1,435,1001,64,1,64,1002,64,2,64,109,-8,2108,31,0,63,1005,63,451,1105,1,457,4,441,1001,64,1,64,1002,64,2,64,109,26,2106,0,-2,4,463,1001,64,1,64,1106,0,475,1002,64,2,64,109,-33,1207,6,38,63,1005,63,491,1106,0,497,4,481,1001,64,1,64,1002,64,2,64,109,3,2108,27,0,63,1005,63,515,4,503,1105,1,519,1001,64,1,64,1002,64,2,64,109,23,2105,1,0,1106,0,537,4,525,1001,64,1,64,1002,64,2,64,109,-30,1207,7,28,63,1005,63,559,4,543,1001,64,1,64,1106,0,559,1002,64,2,64,109,20,21101,43,0,0,1008,1013,43,63,1005,63,581,4,565,1105,1,585,1001,64,1,64,1002,64,2,64,109,-14,2102,1,1,63,1008,63,27,63,1005,63,611,4,591,1001,64,1,64,1105,1,611,1002,64,2,64,109,18,2105,1,7,4,617,1001,64,1,64,1106,0,629,1002,64,2,64,109,13,1206,-9,641,1105,1,647,4,635,1001,64,1,64,1002,64,2,64,109,-18,21107,44,45,-1,1005,1011,665,4,653,1105,1,669,1001,64,1,64,1002,64,2,64,109,-2,2107,28,-9,63,1005,63,687,4,675,1106,0,691,1001,64,1,64,1002,64,2,64,1205,10,701,1106,0,707,4,695,1001,64,1,64,1002,64,2,64,109,-6,1201,2,0,63,1008,63,21,63,1005,63,731,1001,64,1,64,1106,0,733,4,713,1002,64,2,64,109,-5,1208,7,23,63,1005,63,753,1001,64,1,64,1105,1,755,4,739,1002,64,2,64,109,16,1208,-8,37,63,1005,63,777,4,761,1001,64,1,64,1106,0,777,1002,64,2,64,109,3,21107,45,44,-8,1005,1010,797,1001,64,1,64,1105,1,799,4,783,1002,64,2,64,109,-8,1201,-5,0,63,1008,63,36,63,1005,63,821,4,805,1106,0,825,1001,64,1,64,1002,64,2,64,109,-9,2101,0,1,63,1008,63,31,63,1005,63,845,1105,1,851,4,831,1001,64,1,64,1002,64,2,64,109,6,21108,46,49,3,1005,1010,867,1106,0,873,4,857,1001,64,1,64,1002,64,2,64,109,5,21101,47,0,7,1008,1019,44,63,1005,63,897,1001,64,1,64,1106,0,899,4,879,4,64,99,21101,27,0,1,21102,913,1,0,1106,0,920,21201,1,30449,1,204,1,99,109,3,1207,-2,3,63,1005,63,962,21201,-2,-1,1,21101,940,0,0,1105,1,920,21202,1,1,-1,21201,-2,-3,1,21102,1,955,0,1106,0,920,22201,1,-1,-2,1105,1,966,22102,1,-2,-2,109,-3,2105,1,0]);
    let mut output = 0;
    comp.run_to_end(|| 1, |out| output = out);
    println!("Day9a: {}", output);
    assert_eq!(output, 3340912345);
}

fn day9b() {
    let mut comp = Intcode::new_sized(2000, &[1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1102,1,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,902,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1102,1,37,1007,1102,24,1,1006,1102,26,1,1012,1101,528,0,1023,1102,256,1,1027,1102,466,1,1029,1102,1,629,1024,1101,0,620,1025,1101,0,0,1020,1102,1,30,1004,1101,39,0,1003,1102,36,1,1005,1102,531,1,1022,1102,32,1,1019,1101,0,27,1000,1101,0,28,1016,1101,1,0,1021,1101,23,0,1013,1102,1,25,1015,1102,1,21,1008,1102,1,22,1018,1102,1,34,1014,1102,475,1,1028,1101,33,0,1002,1101,0,35,1011,1102,1,20,1009,1102,38,1,1017,1101,259,0,1026,1101,31,0,1010,1101,0,29,1001,109,8,21102,40,1,10,1008,1018,40,63,1005,63,203,4,187,1105,1,207,1001,64,1,64,1002,64,2,64,109,7,21108,41,41,0,1005,1015,225,4,213,1106,0,229,1001,64,1,64,1002,64,2,64,109,1,1205,5,247,4,235,1001,64,1,64,1105,1,247,1002,64,2,64,109,20,2106,0,-9,1105,1,265,4,253,1001,64,1,64,1002,64,2,64,109,-38,1202,4,1,63,1008,63,33,63,1005,63,291,4,271,1001,64,1,64,1106,0,291,1002,64,2,64,109,6,2102,1,0,63,1008,63,29,63,1005,63,315,1001,64,1,64,1106,0,317,4,297,1002,64,2,64,109,10,21102,42,1,5,1008,1019,40,63,1005,63,341,1001,64,1,64,1105,1,343,4,323,1002,64,2,64,109,-13,2101,0,5,63,1008,63,24,63,1005,63,365,4,349,1105,1,369,1001,64,1,64,1002,64,2,64,109,7,1202,-6,1,63,1008,63,36,63,1005,63,389,1105,1,395,4,375,1001,64,1,64,1002,64,2,64,109,1,2107,31,-5,63,1005,63,411,1106,0,417,4,401,1001,64,1,64,1002,64,2,64,109,3,1206,8,431,4,423,1105,1,435,1001,64,1,64,1002,64,2,64,109,-8,2108,31,0,63,1005,63,451,1105,1,457,4,441,1001,64,1,64,1002,64,2,64,109,26,2106,0,-2,4,463,1001,64,1,64,1106,0,475,1002,64,2,64,109,-33,1207,6,38,63,1005,63,491,1106,0,497,4,481,1001,64,1,64,1002,64,2,64,109,3,2108,27,0,63,1005,63,515,4,503,1105,1,519,1001,64,1,64,1002,64,2,64,109,23,2105,1,0,1106,0,537,4,525,1001,64,1,64,1002,64,2,64,109,-30,1207,7,28,63,1005,63,559,4,543,1001,64,1,64,1106,0,559,1002,64,2,64,109,20,21101,43,0,0,1008,1013,43,63,1005,63,581,4,565,1105,1,585,1001,64,1,64,1002,64,2,64,109,-14,2102,1,1,63,1008,63,27,63,1005,63,611,4,591,1001,64,1,64,1105,1,611,1002,64,2,64,109,18,2105,1,7,4,617,1001,64,1,64,1106,0,629,1002,64,2,64,109,13,1206,-9,641,1105,1,647,4,635,1001,64,1,64,1002,64,2,64,109,-18,21107,44,45,-1,1005,1011,665,4,653,1105,1,669,1001,64,1,64,1002,64,2,64,109,-2,2107,28,-9,63,1005,63,687,4,675,1106,0,691,1001,64,1,64,1002,64,2,64,1205,10,701,1106,0,707,4,695,1001,64,1,64,1002,64,2,64,109,-6,1201,2,0,63,1008,63,21,63,1005,63,731,1001,64,1,64,1106,0,733,4,713,1002,64,2,64,109,-5,1208,7,23,63,1005,63,753,1001,64,1,64,1105,1,755,4,739,1002,64,2,64,109,16,1208,-8,37,63,1005,63,777,4,761,1001,64,1,64,1106,0,777,1002,64,2,64,109,3,21107,45,44,-8,1005,1010,797,1001,64,1,64,1105,1,799,4,783,1002,64,2,64,109,-8,1201,-5,0,63,1008,63,36,63,1005,63,821,4,805,1106,0,825,1001,64,1,64,1002,64,2,64,109,-9,2101,0,1,63,1008,63,31,63,1005,63,845,1105,1,851,4,831,1001,64,1,64,1002,64,2,64,109,6,21108,46,49,3,1005,1010,867,1106,0,873,4,857,1001,64,1,64,1002,64,2,64,109,5,21101,47,0,7,1008,1019,44,63,1005,63,897,1001,64,1,64,1106,0,899,4,879,4,64,99,21101,27,0,1,21102,913,1,0,1106,0,920,21201,1,30449,1,204,1,99,109,3,1207,-2,3,63,1005,63,962,21201,-2,-1,1,21101,940,0,0,1105,1,920,21202,1,1,-1,21201,-2,-3,1,21102,1,955,0,1106,0,920,22201,1,-1,-2,1105,1,966,22102,1,-2,-2,109,-3,2105,1,0]);
    let mut output = 0;
    comp.run_to_end(|| 2, |out| output = out);
    println!("Day9b: {}", output);
    assert_eq!(output, 51754);
}

// DAY 8 ---------------------------------------------------------------------------------------------

fn day8a() {
    let file = File::open("data/8.txt").unwrap();
    let bytes = file.bytes();
    let size = 25 * 6;
    let mut current = (0, 0, 0);
    let mut best = (std::usize::MAX, 0);
    let mut count = 0;
    for b in bytes {
        if b.is_err() {
            break;
        }
        let b = b.unwrap();
        count += 1;
        if count >= size {
            if current.0 < best.0 {
                best = (current.0, current.1 * current.2);
            }
            current = (0, 0, 0);
            count = 0;
        }
        match b {
            b'0' => current.0 += 1,
            b'1' => current.1 += 1,
            b'2' => current.2 += 1,
            _ => {},
        }
    }
    println!("Day8a: {}", best.1);
    assert_eq!(best.1, 2904);
}

fn day8b() {
    const W:usize = 25;
    const H:usize = 6;
    const SIZE:usize = W * H;
    let mut layers = Vec::<[u8; SIZE]>::new();
    let mut file = File::open("data/8.txt").unwrap();
    let mut buff = [0u8; SIZE];
    while file.read_exact(&mut buff).is_ok() {
        layers.push(buff);
    }
    println!("Day8b: HGBCF");
    for idx in 0..SIZE {
        if idx != 0 && idx % W == 0 {
            println!("");
        }
        for data in layers.iter() {
            match data[idx] {
                b'0' => {print!(" "); break;},
                b'1' => {print!("#"); break;},
                b'2' => {},
                _ => panic!("invalid image!"),
            }
        }
    }
    println!("");
    // No assert... me sad...
}
// DAY 7 ---------------------------------------------------------------------------------------------

fn day7a() {
    let mut seq = [0,1,2,3,4];
    let permutations = Heap::new(&mut seq);
    let mut best = 0;
    for seq in permutations {
        let data_org = [3,8,1001,8,10,8,105,1,0,0,21,34,43,60,81,94,175,256,337,418,99999,3,9,101,2,9,9,102,4,9,9,4,9,99,3,9,102,2,9,9,4,9,99,3,9,102,4,9,9,1001,9,4,9,102,3,9,9,4,9,99,3,9,102,4,9,9,1001,9,2,9,1002,9,3,9,101,4,9,9,4,9,99,3,9,1001,9,4,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99];
        let mut signal = 0;
        for s in seq.iter() {
            let mut data = data_org.clone();
            let input = [*s as i64, signal];
            let mut input_it = input.iter();
            run_intcode(0, &mut data, || *input_it.next().expect("out of input!"), |out| signal = out);
        }
        best = best.max(signal);
    }
    println!("Day7a: {}", best);
    assert_eq!(best, 11828);
}

fn day7b() {
    let mut seq = [5,6,7,8,9];
    let permutations = Heap::new(&mut seq);
    let mut best = 0;
    for seq in permutations {
        // let seq = [9,8,7,6,5];
        let start_state = [3,8,1001,8,10,8,105,1,0,0,21,34,43,60,81,94,175,256,337,418,99999,3,9,101,2,9,9,102,4,9,9,4,9,99,3,9,102,2,9,9,4,9,99,3,9,102,4,9,9,1001,9,4,9,102,3,9,9,4,9,99,3,9,102,4,9,9,1001,9,2,9,1002,9,3,9,101,4,9,9,4,9,99,3,9,1001,9,4,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99];
        let mut comps = [Intcode::new(&start_state), Intcode::new(&start_state), Intcode::new(&start_state), Intcode::new(&start_state), Intcode::new(&start_state)];
        assert_eq!(comps.len(), 5);
        // let mut states = [(0, [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]);5];
        let mut signal = 0;
        for (s, comp) in seq.iter().zip(comps.iter_mut()) {
            signal = comp.run_input_to_out(&[*s as i64, signal]).expect("failed to initialize");
        }
        let mut contiue = true;
        while contiue {
            for comp in comps.iter_mut() {
                if let Some(res) = comp.run_input_to_out(&[signal]) {
                    signal = res;
                }
                else {
                    contiue = false;
                    break;
                }
            }
        }
        
        best = best.max(signal);
    }
    println!("Day7b: {}", best);
}

// DAY 6 ---------------------------------------------------------------------------------------------

fn parse_orbit_map(file_name:&str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let file = File::open(file_name).unwrap();
    let fin = BufReader::new(file);
    for line in fin.lines() {
        let line = line.unwrap();
        let mut it = line.split(')');
        let main = it.next().unwrap();
        let body = it.next().unwrap();
        map.insert(body.to_string(), main.to_string());
    }
    map
}

fn get_orbits<'a>(obj:&'a str, map:&'a HashMap<String, String>) -> Vec<&'a str> {
    let mut res = Vec::new();
    let mut main:&str = map.get(obj).unwrap();
    while main != "COM" {
        res.push(main);
        main = map.get(main).unwrap();
    }
    res
}

fn day6a() {
    let map = parse_orbit_map("data/6a.txt");
    let mut count = 0;
    for (_, mut main) in map.iter() {
        count += 1;
        while main != "COM" {
            main = map.get(main).unwrap();
            count += 1;
        }
    }
    println!("Day6a: {}", count);
}

fn day6b() {
    let map = parse_orbit_map("data/6a.txt");
    let orbits_me = get_orbits("YOU", &map);
    let orbits_san = get_orbits("SAN", &map);
    for (idx, (me, san)) in orbits_me.iter().rev().zip(orbits_san.iter().rev()).enumerate() {
        if me != san {
            let res = (orbits_me.len() - idx) + (orbits_san.len() - idx);
            println!("Day6b: {}", res);
            return;
        }
    }
    panic!("Failed to find path!");
}

// DAY 5 ---------------------------------------------------------------------------------------------

fn run_intcode<I, O>(_:usize, data:&mut[i64], input:I, output:O)
    where I: FnMut() -> i64, O: FnMut(i64)
{
    let mut comp = Intcode::new(data);
    comp.run_to_end(input, output);
}

fn day5a() {
    let mut data = [3,225,1,225,6,6,1100,1,238,225,104,0,1101,65,73,225,1101,37,7,225,1101,42,58,225,1102,62,44,224,101,-2728,224,224,4,224,102,8,223,223,101,6,224,224,1,223,224,223,1,69,126,224,101,-92,224,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,1102,41,84,225,1001,22,92,224,101,-150,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,80,65,225,1101,32,13,224,101,-45,224,224,4,224,102,8,223,223,101,1,224,224,1,224,223,223,1101,21,18,225,1102,5,51,225,2,17,14,224,1001,224,-2701,224,4,224,1002,223,8,223,101,4,224,224,1,223,224,223,101,68,95,224,101,-148,224,224,4,224,1002,223,8,223,101,1,224,224,1,223,224,223,1102,12,22,225,102,58,173,224,1001,224,-696,224,4,224,1002,223,8,223,1001,224,6,224,1,223,224,223,1002,121,62,224,1001,224,-1302,224,4,224,1002,223,8,223,101,4,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,226,677,224,102,2,223,223,1005,224,329,1001,223,1,223,7,677,226,224,102,2,223,223,1006,224,344,1001,223,1,223,1007,226,677,224,1002,223,2,223,1006,224,359,1001,223,1,223,1007,677,677,224,102,2,223,223,1005,224,374,1001,223,1,223,108,677,677,224,102,2,223,223,1006,224,389,101,1,223,223,8,226,677,224,102,2,223,223,1005,224,404,101,1,223,223,7,226,677,224,1002,223,2,223,1005,224,419,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,434,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,449,101,1,223,223,7,677,677,224,1002,223,2,223,1006,224,464,101,1,223,223,1107,226,226,224,102,2,223,223,1006,224,479,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,108,226,677,224,1002,223,2,223,1006,224,509,101,1,223,223,1108,226,677,224,102,2,223,223,1006,224,524,1001,223,1,223,1008,226,226,224,1002,223,2,223,1005,224,539,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,554,101,1,223,223,8,677,677,224,102,2,223,223,1005,224,569,101,1,223,223,107,226,677,224,102,2,223,223,1005,224,584,101,1,223,223,1108,226,226,224,1002,223,2,223,1005,224,599,1001,223,1,223,1008,677,677,224,1002,223,2,223,1005,224,614,101,1,223,223,1107,226,677,224,102,2,223,223,1005,224,629,101,1,223,223,1108,677,226,224,1002,223,2,223,1005,224,644,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,659,1001,223,1,223,108,226,226,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226];
    let mut input_it = [1].into_iter();
    let mut last_out = -1;
    run_intcode(0, &mut data, || *input_it.next().expect("out of input!"), |out| last_out = out);
    println!("Day5a: {}", last_out);
    assert_eq!(last_out, 14522484);
}

fn day5b() {
    let mut data = [3,225,1,225,6,6,1100,1,238,225,104,0,1101,65,73,225,1101,37,7,225,1101,42,58,225,1102,62,44,224,101,-2728,224,224,4,224,102,8,223,223,101,6,224,224,1,223,224,223,1,69,126,224,101,-92,224,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,1102,41,84,225,1001,22,92,224,101,-150,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,80,65,225,1101,32,13,224,101,-45,224,224,4,224,102,8,223,223,101,1,224,224,1,224,223,223,1101,21,18,225,1102,5,51,225,2,17,14,224,1001,224,-2701,224,4,224,1002,223,8,223,101,4,224,224,1,223,224,223,101,68,95,224,101,-148,224,224,4,224,1002,223,8,223,101,1,224,224,1,223,224,223,1102,12,22,225,102,58,173,224,1001,224,-696,224,4,224,1002,223,8,223,1001,224,6,224,1,223,224,223,1002,121,62,224,1001,224,-1302,224,4,224,1002,223,8,223,101,4,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,226,677,224,102,2,223,223,1005,224,329,1001,223,1,223,7,677,226,224,102,2,223,223,1006,224,344,1001,223,1,223,1007,226,677,224,1002,223,2,223,1006,224,359,1001,223,1,223,1007,677,677,224,102,2,223,223,1005,224,374,1001,223,1,223,108,677,677,224,102,2,223,223,1006,224,389,101,1,223,223,8,226,677,224,102,2,223,223,1005,224,404,101,1,223,223,7,226,677,224,1002,223,2,223,1005,224,419,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,434,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,449,101,1,223,223,7,677,677,224,1002,223,2,223,1006,224,464,101,1,223,223,1107,226,226,224,102,2,223,223,1006,224,479,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,108,226,677,224,1002,223,2,223,1006,224,509,101,1,223,223,1108,226,677,224,102,2,223,223,1006,224,524,1001,223,1,223,1008,226,226,224,1002,223,2,223,1005,224,539,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,554,101,1,223,223,8,677,677,224,102,2,223,223,1005,224,569,101,1,223,223,107,226,677,224,102,2,223,223,1005,224,584,101,1,223,223,1108,226,226,224,1002,223,2,223,1005,224,599,1001,223,1,223,1008,677,677,224,1002,223,2,223,1005,224,614,101,1,223,223,1107,226,677,224,102,2,223,223,1005,224,629,101,1,223,223,1108,677,226,224,1002,223,2,223,1005,224,644,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,659,1001,223,1,223,108,226,226,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226];
    let mut input_it = [5].into_iter();
    let mut last_out = -1;
    // run_intcode(0, &mut data, || *input_it.next().expect("out of input!"), |out| println!("Intcode out: {}", out));
    run_intcode(0, &mut data, || *input_it.next().expect("out of input!"), |out| last_out = out);
    println!("Day5a: {}", last_out);
    assert_eq!(last_out, 4655956);
}

// DAY 4 ---------------------------------------------------------------------------------------------

fn find_pairs_u8(bytes:&[u8]) -> Option<usize> {
	let mut last = bytes.last().map_or(99, |c| *c + 1);
	bytes.iter().rposition(|c| {
		if *c != last {
			last = *c;
			return false;
		}
		true
	})
}

struct PasswordIter {
	digits:Vec<u8>,
	pair:Option<usize>,
	current:usize,
	limit:usize,
}

impl PasswordIter {
	// moves us to next pair. might stay where we are if we are on a pair
	fn next_pair(&mut self) -> Option<usize> {
		self.pair = find_pairs_u8(&self.digits);
		if self.pair.is_some() {
			return Some(self.current)
		}
		if self.digits[0] <= self.digits[1] {
			panic!("Shouldn't be here!");
		}
		let diff = self.digits[0] - self.digits[1];
		self.digits[1] = self.digits[0];
		self.current += 10 * diff as usize;
		if self.current < self.limit {
			return self.next_pair();
		}
		None
	}

	fn fix_increasing(&mut self) {
		let mut min = 0;
		for (idx, digit) in self.digits.iter_mut().enumerate().rev() {
			if *digit < min {
				let diff = min - *digit;
				self.current += Self::calc_digit(idx, diff);
				*digit = min;
			}
			min = *digit;
		}
	}

	// retuns index of highest touched
	fn add_at_digit(&mut self, idx:usize, val:u8) -> usize {
		let mut idx = idx;
		self.current += Self::calc_digit(idx, val);
		while idx < self.digits.len() {
			self.digits[idx] += 1;
			if self.digits[idx] >= 10 {
				self.digits[idx] = 0;
				idx += 1;
			}
			else {
				break;
			}
		}
		idx
	}

	fn calc_digit(idx:usize, num:u8) -> usize {
		10usize.pow(idx as u32) * num as usize
	}

	fn new(start:usize, end:usize, num_digits:usize) -> Self {
		let mut res = PasswordIter{
			digits: start.to_string().bytes().map(|c| c - b'0').rev().collect(),
			current: start,
			limit: end,
			pair: None,
		};
		res.digits.resize(num_digits, 0);
		assert!(Self::calc_digit(num_digits, 1) > end, "Limit is too high!");
		assert!(num_digits >= 3, "Need at least 3 digits limit! Got {}.", num_digits);
		res
	}
}

impl Iterator for PasswordIter {
	type Item = usize;
	fn next(&mut self) -> Option<Self::Item> {
		if self.current > self.limit {
			return None;
		}
		// i.e if first
		if self.pair.is_none() { 
			self.fix_increasing();
			return self.next_pair();
		}
		let idx = self.add_at_digit(0, 1);
		self.fix_increasing(); // this shouldn't affect anything above idx
		if self.current > self.limit {
			return None;
		}
		if idx >= self.pair.unwrap() {
			return self.next_pair();
		}
		Some(self.current)
	}
}

fn day4a() {
	let count = PasswordIter::new(134792, 675810, 6).count();
	println!("Day4a: {}", count);
	assert_eq!(count, 1955);
}

// fn day4b() {
	// let count = PasswordIter::new(134792, 675810, 6).count();
	// println!("Day4a: {}", count);
// }

// DAY 3 ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone)]
struct PathIter {
    pos:Vec2<i32>,
    count:usize,
    dir:Vec2<i32>,
    path:Vec<(Dir, usize)>,
}

impl PathIter {
    fn from_str_path0(s:&str) -> Self {
        Self::from_str_path(Vec2::zero(), s)
    }

    fn from_str_path(pos:Vec2<i32>, s:&str) -> Self {
        let path = s.split(',').map(|s| {
            let dir = match s.chars().nth(0) {
                Some('U') => Dir::Up,
                Some('D') => Dir::Down,
                Some('L') => Dir::Left,
                Some('R') => Dir::Right,
                _ => panic!("Failed to parse path!"),
            };
            (dir, usize::from_str_radix(&s[1..], 10).expect("Failed to read path node count"))
        }).rev();
        PathIter{
            pos,
            count: 0,
            dir: Vec2::zero(),
            path: path.collect(),
        }
    }
}

impl Iterator for PathIter {
    type Item = Vec2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            self.count -= 1;
            self.pos += self.dir;
            return Some(self.pos);
        }
        if let Some((dir, count)) = self.path.pop() {
            self.count = count;
            self.dir = match dir {
                Dir::Up => Vec2::new(0, -1),
                Dir::Down => Vec2::new(0, 1),
                Dir::Left => Vec2::new(-1, 0),
                Dir::Right => Vec2::new(1, 0),
            };
            return self.next();
        }
        None
    }
}

fn print_grid(grid:&HashMap<Vec2<i32>, char>) {
    let size = grid.iter().fold(Rect::new(0, 0, 0, 0), |rect, (pos, _)| rect.expanded_to_contain_point(*pos)).into_aabr();
    print_grid_part(grid, size.min, size.max);
}

fn print_grid_part(grid:&HashMap<Vec2<i32>, char>, min:Vec2<i32>, max:Vec2<i32>) {
    // let mut f = File::create("output").unwrap();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if let Some(c) = grid.get(&Vec2::new(x, y)) {
                print!("{}", c);
                // write!(&mut f, "{}", c);
            }
            else {
                print!(".");
                // write!(&mut f, ".");
            }
        }
        println!("");
        // writeln!(&mut f, "");
    }
}

fn day3a() {
    let path0 = PathIter::from_str_path0("R997,U849,R349,U641,R581,D39,R285,U139,R455,D346,L965,D707,R393,D302,L263,U58,R950,U731,R858,D748,R302,U211,R588,D441,L153,D417,R861,U775,R777,U204,R929,U868,L62,U163,R841,D214,L648,U626,R501,D751,L641,D961,L23,D430,L73,D692,R49,U334,L601,U996,R444,D658,R633,D30,L861,D811,R10,D394,R9,U227,L848,U420,L378,D622,L501,U397,R855,U369,R615,U591,L674,D166,L181,U61,L224,U463,L203,U594,R93,U614,L959,U198,L689,D229,L674,U255,R843,D382,R538,U923,L960,D775,L879,U97,R137,U665,L340,D941,L775,D57,R852,D167,R980,U704,L843,U989,L611,D32,L724,D790,L32,U984,L39,U671,L994,U399,R475,D85,L322,D936,R117,D261,R705,D696,L523,D433,L239,U477,L247,D465,R560,D902,L589,U682,R645,U376,L989,D121,L215,U514,R519,U407,L218,D444,R704,D436,L680,U759,R937,U400,R533,D860,R782,D233,R840,D549,L508,U380,L992,U406,L213,D403,L413,D532,L429,U186,R262,U313,L913,U873,L838,D882,R851,U70,R185,D131,R945,D595,L330,U446,R88,D243,L561,D952,R982,D395,L708,U459,L82,D885,L996,U955,L406,U697,L183,U266,L878,D839,R843,D891,R118,U772,R590,D376,L500,U370,R607,D12,L310,D436,L602,D365,R886,U239,L471,D418,L122,U18,R879,D693,R856,U848,L657,D911,L63,U431,R41,U752,R919,U323,L61,D263,L370,D85,R929,D213,R350,U818,R458,D912,R509,U394,L734,U49,R810,D87,L870,U658,R499,U550,L402,U244,L112,U859,R836,U951,R222,D944,L691,D731,R742,D52,R984,D453,L514,U692,R812,U35,L844,D177,L110,D22,R61,U253,R618,D51,R163,U835,R704,U148,R766,U297,R457,D170,L104,D441,R330,D330,R989,D538,R668,D811,R62,D67,L470,D526,R788,U376,R708,U3,R961");
    let path1 = PathIter::from_str_path0("L1009,D381,R970,U429,L230,D909,R516,D957,R981,U609,L480,D139,L861,U168,L48,U620,R531,D466,L726,D380,R977,D454,L318,D397,R994,U402,L77,U93,L359,D72,R968,D956,L174,D22,R218,U619,R593,U32,L154,U55,L169,U415,L171,U666,R617,U109,L265,U773,R541,D808,L797,U478,R731,U379,R311,D137,L806,U298,R362,D458,L254,D539,R700,U853,R246,D588,L28,U203,L432,U946,R663,D408,R974,U59,L683,D36,L139,U738,L780,U414,L401,D93,R212,D973,L710,U892,L357,D177,R823,D4,R46,D924,L235,D898,R67,U220,L592,U87,R94,U584,R979,D843,L299,D648,L491,U360,R824,D245,L944,D24,R616,U975,L4,U42,L984,U181,R902,D835,L687,D413,L767,U632,L754,U270,R413,U51,L825,D377,L596,U960,L378,U706,L859,D708,L156,D991,L814,U351,R923,D749,L16,D651,R20,D86,R801,U811,L228,U161,L871,U129,R215,U235,L784,U896,R94,U145,R822,U494,R248,D98,R494,U156,L495,U311,R66,D873,L294,D620,L885,U395,R778,D227,R966,U756,L694,D707,R983,D950,R706,D730,R415,U886,L465,D622,L13,D938,R324,D464,R723,U804,R942,D635,L729,D317,L522,U469,R550,D141,R302,U999,L642,U509,R431,D380,R18,D676,R449,D759,L495,U901,R1,D745,L655,U449,L439,D818,R55,D541,R420,U764,L426,D176,L520,U3,L663,D221,L80,D449,L987,U349,L71,U632,L887,D231,R655,D208,R698,D639,R804,U616,R532,U846,R363,D141,R659,U470,L798,U144,L675,U483,L944,U380,L329,U72,L894,D130,R53,U109,R610,U770,R778,U493,R972,D340,L866,U980,L305,D812,R130,D954,R253,D33,L912,U950,L438,D680,R891,U725,R171,D587,R549,D367,L4,U313,R522,D128,L711,D405,L769,D496,L527,U373,R725,D261,L268,D939,L902,D58,L858,D190,L442");
    // let path1c = path1.clone();
    let mut grid = HashMap::new();
    path0.for_each(|pos| {grid.insert(pos, '1');});
    let mut dist = std::i32::MAX;
    let mut best = Vec2::zero();
    for pos in path1 {
        if grid.get(&pos).is_some() {
            let d = pos.x.abs() + pos.y.abs();
            if d < dist {
                dist = d;
                best = pos;
            }
        }
    }
    println!("Day3a: {} ({:?})", dist, best);
    assert_eq!(dist, 209);
    // print_grid(&grid);
    // let half_size = Vec2::new(10, 10);
    // path1c.for_each(|pos| {grid.entry(pos).and_modify(|c| *c = 'X').or_insert('2');});
    // print_grid_part(&grid, best - half_size, best + half_size);
}

fn day3b() {
    let path0 = PathIter::from_str_path0("R997,U849,R349,U641,R581,D39,R285,U139,R455,D346,L965,D707,R393,D302,L263,U58,R950,U731,R858,D748,R302,U211,R588,D441,L153,D417,R861,U775,R777,U204,R929,U868,L62,U163,R841,D214,L648,U626,R501,D751,L641,D961,L23,D430,L73,D692,R49,U334,L601,U996,R444,D658,R633,D30,L861,D811,R10,D394,R9,U227,L848,U420,L378,D622,L501,U397,R855,U369,R615,U591,L674,D166,L181,U61,L224,U463,L203,U594,R93,U614,L959,U198,L689,D229,L674,U255,R843,D382,R538,U923,L960,D775,L879,U97,R137,U665,L340,D941,L775,D57,R852,D167,R980,U704,L843,U989,L611,D32,L724,D790,L32,U984,L39,U671,L994,U399,R475,D85,L322,D936,R117,D261,R705,D696,L523,D433,L239,U477,L247,D465,R560,D902,L589,U682,R645,U376,L989,D121,L215,U514,R519,U407,L218,D444,R704,D436,L680,U759,R937,U400,R533,D860,R782,D233,R840,D549,L508,U380,L992,U406,L213,D403,L413,D532,L429,U186,R262,U313,L913,U873,L838,D882,R851,U70,R185,D131,R945,D595,L330,U446,R88,D243,L561,D952,R982,D395,L708,U459,L82,D885,L996,U955,L406,U697,L183,U266,L878,D839,R843,D891,R118,U772,R590,D376,L500,U370,R607,D12,L310,D436,L602,D365,R886,U239,L471,D418,L122,U18,R879,D693,R856,U848,L657,D911,L63,U431,R41,U752,R919,U323,L61,D263,L370,D85,R929,D213,R350,U818,R458,D912,R509,U394,L734,U49,R810,D87,L870,U658,R499,U550,L402,U244,L112,U859,R836,U951,R222,D944,L691,D731,R742,D52,R984,D453,L514,U692,R812,U35,L844,D177,L110,D22,R61,U253,R618,D51,R163,U835,R704,U148,R766,U297,R457,D170,L104,D441,R330,D330,R989,D538,R668,D811,R62,D67,L470,D526,R788,U376,R708,U3,R961");
    let path1 = PathIter::from_str_path0("L1009,D381,R970,U429,L230,D909,R516,D957,R981,U609,L480,D139,L861,U168,L48,U620,R531,D466,L726,D380,R977,D454,L318,D397,R994,U402,L77,U93,L359,D72,R968,D956,L174,D22,R218,U619,R593,U32,L154,U55,L169,U415,L171,U666,R617,U109,L265,U773,R541,D808,L797,U478,R731,U379,R311,D137,L806,U298,R362,D458,L254,D539,R700,U853,R246,D588,L28,U203,L432,U946,R663,D408,R974,U59,L683,D36,L139,U738,L780,U414,L401,D93,R212,D973,L710,U892,L357,D177,R823,D4,R46,D924,L235,D898,R67,U220,L592,U87,R94,U584,R979,D843,L299,D648,L491,U360,R824,D245,L944,D24,R616,U975,L4,U42,L984,U181,R902,D835,L687,D413,L767,U632,L754,U270,R413,U51,L825,D377,L596,U960,L378,U706,L859,D708,L156,D991,L814,U351,R923,D749,L16,D651,R20,D86,R801,U811,L228,U161,L871,U129,R215,U235,L784,U896,R94,U145,R822,U494,R248,D98,R494,U156,L495,U311,R66,D873,L294,D620,L885,U395,R778,D227,R966,U756,L694,D707,R983,D950,R706,D730,R415,U886,L465,D622,L13,D938,R324,D464,R723,U804,R942,D635,L729,D317,L522,U469,R550,D141,R302,U999,L642,U509,R431,D380,R18,D676,R449,D759,L495,U901,R1,D745,L655,U449,L439,D818,R55,D541,R420,U764,L426,D176,L520,U3,L663,D221,L80,D449,L987,U349,L71,U632,L887,D231,R655,D208,R698,D639,R804,U616,R532,U846,R363,D141,R659,U470,L798,U144,L675,U483,L944,U380,L329,U72,L894,D130,R53,U109,R610,U770,R778,U493,R972,D340,L866,U980,L305,D812,R130,D954,R253,D33,L912,U950,L438,D680,R891,U725,R171,D587,R549,D367,L4,U313,R522,D128,L711,D405,L769,D496,L527,U373,R725,D261,L268,D939,L902,D58,L858,D190,L442");
    let mut grid = HashMap::new();
    path0.enumerate().for_each(|(idx, pos)| {grid.insert(pos, idx);});
    let mut dist = std::usize::MAX;
    let mut best = (Vec2::zero(), 0, 0);
    for (step1, pos) in path1.enumerate() {
        if let Some(step0) = grid.get(&pos) {
            let d = step0 + step1;
            if d < dist {
                dist = d;
                best = (pos, *step0, step1);
            }
        }
    }
    dist += 2; // As we where using the index instead of the order
    println!("Day3b: {} ({:?})", dist, best);
}

// DAY 2 ---------------------------------------------------------------------------------------------

fn run_intcode_prg(noun:i64, verb:i64) -> i64 {
    let mut comp = Intcode::new(&[1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,5,19,23,2,10,23,27,1,27,5,31,2,9,31,35,1,35,5,39,2,6,39,43,1,43,5,47,2,47,10,51,2,51,6,55,1,5,55,59,2,10,59,63,1,63,6,67,2,67,6,71,1,71,5,75,1,13,75,79,1,6,79,83,2,83,13,87,1,87,6,91,1,10,91,95,1,95,9,99,2,99,13,103,1,103,6,107,2,107,6,111,1,111,2,115,1,115,13,0,99,2,0,14,0]);
    comp.data[1] = noun;
    comp.data[2] = verb;
    while let Some(next) = comp.cycle(&mut || panic!("no input!"), &mut |_| panic!("no output!")) {
        comp.pc = Some(next);
    }
    comp.data[0]
}

fn day2a() {
    let res = run_intcode_prg(12, 2);
    println!("Day2a: {}", res);
    assert_eq!(res, 4714701);
}

fn day2b() {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_intcode_prg(noun, verb) == 19690720 {
                println!("Day2b: {}{}", noun, verb);
                assert_eq!(noun, 51);
                assert_eq!(verb, 21);
                return;
            }
        }
    } 
    panic!("Failed to find noun/verb pair");
}

// DAY 1 ---------------------------------------------------------------------------------------------

fn fuel_req_simple(wieght:i32) -> i32 {
    cmp::max((wieght / 3) - 2, 0)
}

fn fuel_req(wieght:i32) -> i32 {
    let mut res = fuel_req_simple(wieght);
    let mut extra = fuel_req_simple(res);
    loop {
        res += extra;
        extra = fuel_req_simple(extra);
        if extra == 0 {
            break;
        }
    }
    res
}

fn day1a() {
    let data = [94794, 58062, 112067, 139512, 147400, 99825, 142617, 107263, 86294, 97000, 140204, 72573, 134981, 111385, 88303, 79387, 129111, 122976, 130685, 75100, 146566, 73191, 107641, 109940, 65518, 102028, 57370, 144556, 64017, 64384, 145114, 115853, 87939, 90791, 133443, 139050, 140657, 85738, 133749, 92466, 142918, 96679, 125035, 127629, 87906, 104478, 105147, 121741, 70312, 73732, 60838, 82292, 102931, 103000, 135903, 78678, 86314, 50772, 115673, 106179, 60615, 105152, 76550, 140591, 120916, 62094, 111273, 63542, 102974, 78837, 94840, 89126, 63150, 52503, 108530, 101458, 59660, 116913, 66440, 83306, 50693, 58377, 62005, 130663, 124304, 79726, 63001, 73380, 64395, 124277, 69742, 63465, 93172, 142068, 120081, 119872, 52801, 100693, 79229, 90365];
    println!("Day 1a: {}", data.into_iter().fold(0, |acc, v| acc + fuel_req_simple(*v)));
}

fn day1b() {
    // let data = [100756];
    let data = [94794, 58062, 112067, 139512, 147400, 99825, 142617, 107263, 86294, 97000, 140204, 72573, 134981, 111385, 88303, 79387, 129111, 122976, 130685, 75100, 146566, 73191, 107641, 109940, 65518, 102028, 57370, 144556, 64017, 64384, 145114, 115853, 87939, 90791, 133443, 139050, 140657, 85738, 133749, 92466, 142918, 96679, 125035, 127629, 87906, 104478, 105147, 121741, 70312, 73732, 60838, 82292, 102931, 103000, 135903, 78678, 86314, 50772, 115673, 106179, 60615, 105152, 76550, 140591, 120916, 62094, 111273, 63542, 102974, 78837, 94840, 89126, 63150, 52503, 108530, 101458, 59660, 116913, 66440, 83306, 50693, 58377, 62005, 130663, 124304, 79726, 63001, 73380, 64395, 124277, 69742, 63465, 93172, 142068, 120081, 119872, 52801, 100693, 79229, 90365];
    println!("Day 1b: {}", data.into_iter().fold(0, |acc, v| acc + fuel_req(*v)));
}