use std::cmp;
use std::fs::File;
use std::io::{self, Read, Write, BufReader, prelude::*};
use std::iter;
use std::collections::HashMap;
use vek::vec::repr_c::Vec2;
use vek::geom::repr_c::Rect;


fn main() {
    day1a();
    day1b();
    day2a();
    day2b();
    day3a();
    day3b();
    day4a();
    // forgot to commit 4b.
    // day4b();
    day5a();
    day5b();
    day6a();
    day6b();
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
    let mut map = parse_orbit_map("data/6a.txt");
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

fn run_intcode<I, O>(start_at:usize, data:&mut[i64], mut input:I, mut output:O)
    where I: FnMut() -> i64, O: FnMut(i64)
{
    let mut pc = start_at;
    while let Some(next) = intcode_cycle(pc, data, &mut input, &mut output) {
        pc = next;
    }
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

fn day4b() {
	// let count = PasswordIter::new(134792, 675810, 6).count();
	// println!("Day4a: {}", count);
}

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

fn intcode_mem_ref(at:usize, mode:i64, data:&mut[i64]) -> &mut i64 {
    if mode == 0 { &mut data[data[at] as usize] } else { &mut data[at] }
}

fn intcode_mem(at:usize, mode:i64, data:&[i64]) -> i64 {
    if mode == 0 { data[data[at] as usize] } else { data[at] }
}

fn intcode_cycle<I, O>(pc:usize, data:&mut[i64], input:&mut I, output:&mut O) -> Option<usize> 
    where I: FnMut() -> i64, O: FnMut(i64)
{
    let mut i = data[pc];
    let op = i % 100;
    i /= 100;
    let m0 = i % 10;
    i /= 10;
    let m1 = i % 10;
    i /= 10;
    let m2 = i % 10;

    let mut next = pc;
    match op {
        1 => {
            *intcode_mem_ref(pc + 3, m2, data) = intcode_mem(pc + 1, m0, data) + intcode_mem(pc + 2, m1, data);
            next += 4;
        },
        2 => {
            *intcode_mem_ref(pc + 3, m2, data) = intcode_mem(pc + 1, m0, data) * intcode_mem(pc + 2, m1, data);
            next += 4;
        },
        3 => {
            *intcode_mem_ref(pc + 1, 0, data) = input();
            next += 2;
        }
        4 => {
            output(intcode_mem(pc + 1, m0, data));
            next += 2;
        }
        5 => {
            next = if intcode_mem(pc + 1, m0, data) != 0 {intcode_mem(pc + 2, m1, data) as usize} else {next + 3};
        }
        6 => {
            next = if intcode_mem(pc + 1, m0, data) == 0 {intcode_mem(pc + 2, m1, data) as usize} else {next + 3};
        }
        7 => {
            *intcode_mem_ref(pc + 3, m2, data) = if intcode_mem(pc + 1, m0, data) < intcode_mem(pc + 2, m1, data) {1} else {0};
            next += 4;
        }
        8 => {
            *intcode_mem_ref(pc + 3, m2, data) = if intcode_mem(pc + 1, m0, data) == intcode_mem(pc + 2, m1, data) {1} else {0};
            next += 4;
        }
        99 => return None,
        _ => panic!("Invalid opcode!"),
    }
    return Some(next);
}

fn run_intcode_prg(noun:i64, verb:i64) -> i64 {
    let mut data = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,5,19,23,2,10,23,27,1,27,5,31,2,9,31,35,1,35,5,39,2,6,39,43,1,43,5,47,2,47,10,51,2,51,6,55,1,5,55,59,2,10,59,63,1,63,6,67,2,67,6,71,1,71,5,75,1,13,75,79,1,6,79,83,2,83,13,87,1,87,6,91,1,10,91,95,1,95,9,99,2,99,13,103,1,103,6,107,2,107,6,111,1,111,2,115,1,115,13,0,99,2,0,14,0];
    data[1] = noun;
    data[2] = verb;
    let mut pc = 0;
    while let Some(next) = intcode_cycle(pc, &mut data, &mut || panic!("no input!"), &mut |_| panic!("no output!")) {
        pc = next;
    }
    data[0]
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