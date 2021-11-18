use std::fmt;
use std::env;
use std::fs;
use std::io;
use std::io::{ ErrorKind };
use rand::Rng;

const TAG_ARGUMENT_NOT_VALID: &str = "ArgumentNotValid";


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
	
	fn add(&mut self, reg1: usize, reg2: usize) -> Result<(), ErrorKind> {
		if [reg1, reg2].iter().any(|&x| x > 7) {
			return Err(ErrorKind::InvalidData);
		}
		else {
			self.registers[reg1] = add_overflow(self.registers[reg1], self.registers[reg2]);
			Ok(())
		}
	}
	
	fn io(&mut self, reg1: usize, reg2: usize) -> Result<(), ErrorKind>{
		if [reg1, reg2].iter().any(|&x| x > 7) {
			return Err(ErrorKind::InvalidData); 
		}
		else {
			match self.registers[reg1] {
				0 => {
					println!("{}", self.registers[reg2]);
					Ok(())
				}
				1 => {
					let mut input = String::new();
					io::stdin().read_line(&mut input).expect("Invalid input");
					let input_num: i32 = input.trim().parse().expect("Invalid input");

					self.registers[reg2] = add_overflow(self.registers[reg2], input_num);
					Ok(())
				}
				_ => {
					return Err(ErrorKind::InvalidData);
				}
			}
		}
	}
	
	fn skip(&mut self, reg1: usize, reg2: usize) -> bool {
		self.registers[reg1] == self.registers[reg2]
	}

	fn run (&mut self, instructions: Vec<Vec<(String, i32, Option<i32>)>>) {
		let mut line_idx: usize = 0;
		while line_idx < instructions.len() {
			for instruction in &instructions[line_idx] {
				match &instruction.0[..] {
					"add" => {
						self.add(instruction.1 as usize, instruction.2.unwrap() as usize);
					}
					"io" => {
						self.io(instruction.1 as usize, instruction.2.unwrap() as usize);
					}
					"sk" => {
						if self.skip(instruction.1 as usize, instruction.2.unwrap() as usize) {
							line_idx += 1;
							break;

						}
					}
					"j" => {
						line_idx = (line_idx as i32 + self.registers[instruction.1 as usize] - 1) as usize; // todo handle out of range
						break;
					}
					_ => ()
				}
			}
			line_idx += 1;
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

fn read_instructions(lines: Vec<String>) -> Vec<Vec<(String, i32, Option<i32>)>> {
	let mut instructions: Vec<Vec<(String, i32, Option<i32>)>> = Vec::new(); // vector of lines
	let mut line_instructions: Vec<(String, i32, Option<i32>)>; //instructions in one line
	let mut comment_result: std::option::Option<usize>; // index of comment as result
	let mut end_index: usize; //index of comment unpacked
	let mut line_content: Vec<&str>; // substrings of line seperated by whitespace

	// get index of comment in line
	for line in &lines{
		line_instructions = Vec::new(); // empty
		
		// get index of comment in line
		comment_result = line.chars().position(|ch| ch == '/');
		end_index = if comment_result.is_none() {line.len()} else {comment_result.unwrap()};

		if end_index != 0 { // if line is not commented out
			line_content = (line[..end_index]).split_whitespace().collect();
			for (i, thing) in line_content.iter().enumerate() {
				match thing {
					&"add" | &"io" | &"sk" => {
						line_instructions.push((
							thing.to_string(),
							line_content[i+1].parse::<i32>().unwrap(),
							Some(line_content[i+2].parse::<i32>().unwrap())
						))
					}
					&"j" => {
						line_instructions.push((
							thing.to_string(),
							line_content[i+1].parse::<i32>().unwrap(),
							None
						))
					}
					_ => ()
				}
			}
			instructions.push(line_instructions);
		}
	}
	instructions
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

				// get lines
				let lines: Vec<String> = _contents
					.split("\n")
					.map(|_line| _line.to_string())
					.collect();
					
					let instructions = read_instructions(lines);
					let mut program = Process::new();
					program.run(instructions);
					println!("Finished running");

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
	parse_file();
}

fn main() {
	test();
}
