#[derive(Clone)]
pub struct Intcode {
	pub data: Vec<i64>,
	pub pc: Option<usize>,
	pub relative: i64,
	fixed: bool,
}

impl Intcode {
	pub fn new(data:&[i64]) -> Self {
		Intcode {
			data: data.to_vec(),
			pc: Some(0),
			relative: 0,
			fixed: false,
		}
	}

	pub fn new_fixed(data:&[i64]) -> Self {
		Intcode {
			data: data.to_vec(),
			pc: Some(0),
			relative: 0,
			fixed: true,
		}
	}

	pub fn new_sized(mem:usize, data:&[i64]) -> Self {
		let mut data = data.to_vec();
		data.resize(mem, 0);
		Intcode {
			data: data,
			pc: Some(0),
			relative: 0,
			fixed: true,
		}
	}

	pub fn mem_get(&self, at:usize, mode:i64) -> i64 {
		*self.data.get(self.to_addr(at, mode)).unwrap_or(&0)
	}

	pub fn to_addr(&self, at:usize, mode:i64) -> usize {
		match mode {
			0 => self.data[at] as usize,
			1 => at,
			2 => (self.relative + self.data[at]) as usize,
			_ => panic!("unkown adress mode"),
		}
	}

	pub fn mem_ref(&mut self, at:usize, mode:i64) -> &mut i64 {
		let at = self.to_addr(at, mode);
		if !self.fixed && at >= self.data.len() {
			self.data.resize(at + 100, 0);
		}
		&mut self.data[at]
	}

	// pub fn input_it(input:&[i64]) -> impl FnMut() -> i64 + '_  {
	// 	let mut iter = input.iter();
	// 	move || *iter.next().expect("out of input")
	// }

	pub fn run_to_end<I, O>(&mut self, mut input:I, mut output:O)
    where I: FnMut() -> i64, O: FnMut(i64)
	{
		while self.pc.is_some() {
			self.pc = self.cycle(&mut input, &mut output);
		}
	}

	pub fn run_input_to_out(&mut self, input:&[i64]) -> Option<i64> 
	{
		let mut iter = input.iter();
		let res = self.run_to_out(|| *iter.next().expect("out of input!"));
		if res.is_some() {
			assert_eq!(iter.next(), None);
		}
		res
	}

	pub fn run_to_out<I>(&mut self, mut input:I) -> Option<i64> 
    	where I: FnMut() -> i64
	{
		let mut res = None;
		while self.pc.is_some() {
			self.pc = self.cycle(&mut input, &mut |out| res = Some(out));
			if res.is_some() {
				return res;
			}
		}
    	None
	}

	pub fn cycle<I, O>(&mut self, input:&mut I, output:&mut O) -> Option<usize>
		where I: FnMut() -> i64, O: FnMut(i64)
	{
		let pc = self.pc.expect("trying to cycle stopped intcode computer");
		let mut i = self.data[pc];
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
				*self.mem_ref(pc + 3, m2) = self.mem_get(pc + 1, m0) + self.mem_get(pc + 2, m1);
				next += 4;
			},
			2 => {
				*self.mem_ref(pc + 3, m2) = self.mem_get(pc + 1, m0) * self.mem_get(pc + 2, m1);
				next += 4;
			},
			3 => {
				*self.mem_ref(pc + 1, m0) = input();
				next += 2;
			}
			4 => {
				output(self.mem_get(pc + 1, m0));
				next += 2;
			}
			5 => {
				next = if self.mem_get(pc + 1, m0) != 0 {self.mem_get(pc + 2, m1) as usize} else {next + 3};
			}
			6 => {
				next = if self.mem_get(pc + 1, m0) == 0 {self.mem_get(pc + 2, m1) as usize} else {next + 3};
			}
			7 => {
				*self.mem_ref(pc + 3, m2) = if self.mem_get(pc + 1, m0) < self.mem_get(pc + 2, m1) {1} else {0};
				next += 4;
			}
			8 => {
				*self.mem_ref(pc + 3, m2) = if self.mem_get(pc + 1, m0) == self.mem_get(pc + 2, m1) {1} else {0};
				next += 4;
			}
			9 => {
				self.relative += self.mem_get(pc + 1, m0);
				next += 2;
			}
			99 => return None,
			_ => panic!("Invalid opcode!"),
		}
		return Some(next);
	}
}