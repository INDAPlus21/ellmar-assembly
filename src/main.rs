use std::fmt;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{ ErrorKind };
use rand::Rng;

const TAG_ARGUMENT_NOT_VALID: &str = "ArgumentNotValid";

const io_register: usize = 2;


struct Process {
	registers: [i32; 8]
}

impl fmt::Debug for Process {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut repr: String = String::new();
		for reg in self.registers{
			repr.push_str(&reg.to_string());
			repr.push_str("; ");
		}
		write!(f, "{}", repr)
	}
}

impl Process {
	fn new() -> Process {
		let mut rng = rand::thread_rng();
		Process {
			registers: [
				1, 
				-1, 
				rng.gen::<i32>(),
				rng.gen::<i32>(),
				rng.gen::<i32>(),
				rng.gen::<i32>(),
				rng.gen::<i32>(),
				rng.gen::<i32>(),
				]
		}
	}
	
	fn add(&mut self, reg1: usize, reg2: usize, reg3: usize) -> Result<(), ErrorKind> {
		if [reg1, reg2, reg3].iter().any(|&x| x > 7) {
			return Err(ErrorKind::InvalidData);
		}
		else {
			self.registers[reg1] = add_overflow(self.registers[reg2], self.registers[reg3]);
			Ok(())
		}
	}
	
	fn io(&mut self, reg: usize) -> Result<(), ErrorKind>{
		if reg > 7 {
			return Err(ErrorKind::InvalidData); 
		}
		else {
			match self.registers[reg] {
				0 => {
					println!("{}", self.registers[io_register]);
					Ok(())
				}
				1 => {
					let mut input = String::new();
					io::stdin().read_line(&mut input).expect("Invalid input");
					let input_num: i32 = input.trim().parse().expect("Invalid input");

					self.registers[io_register] = add_overflow(self.registers[io_register], input_num);
					Ok(())
				}
				_ => {
					return Err(ErrorKind::InvalidData);
				}
			}
		}
	}
}

/// Emulate integer overflow resulting in going negative/positive
fn add_overflow(a: i32, b: i32) -> i32 {
	let sum = a as i64 + b as i64;
	let max = i32::MAX as i64;
	let min = i32::MIN as i64;

	if sum > max {
		(sum - max + min) as i32
	}
	else if sum < min {
		(sum - min + max) as i32
	}
	else {
		sum as i32
	}
}

/// print error message, "inspired" by Viola
fn print_error(tag: &str, err_msg: &str) {
	println!("[ERROR] {}: {}", tag, err_msg)
}

fn parse_file() -> Result<(), ErrorKind>{
	// get arguments from terminal
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		print_error(TAG_ARGUMENT_NOT_VALID, "A path to a file must be provided!");
		return Err(ErrorKind::InvalidData);
	}
	else {
		match fs::read_to_string(args[1].clone()) {
			// interpret if read succesfully
			Ok(_contents) => {
				println!("Running file: {:?}\n", args[1]);

				// get non-empty lines
				let lines: Vec<String> = _contents
					.split("\n")
					.map(|_line| _line.to_string())
					.collect();

				// Does this work?
				return Ok(());
			}
			Err(_err) => {
				print_error(TAG_ARGUMENT_NOT_VALID, match _err.kind() {
					ErrorKind::InvalidData => "The provided path is not a path!",
					ErrorKind::NotFound => "File not found!",
					_ => "Failed to read file!"
				});
				return Err(_err.kind());
			}
		}
	}
}

fn test() {
	let mut process = Process::new();
	println!("add");
	process.add(2, 2, 0);
	println!("{:?}", process);
	println!();
	
	println!("input");
	process.io(0);
	println!("{:?}", process);
	println!();
		
	println!("print");
	process.io(4);
	println!();

}

fn main() {
	test();
}
