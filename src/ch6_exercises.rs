//! This exercise asks you to implement a very simple Turing machine, which
//! outputs a binary string of alternating 0's and 1's onto its tape.
//! Most of the implementation is already in place, but you need to

// The Halted state contains the symbol at the current head position
#[derive(Debug, PartialEq)]
enum State {
	Running,
	Halted(u8), // Storing the value at the halt position
}

struct TuringMachine {
	tape: Vec<u8>,
	position: usize,
	state: State,
}

impl TuringMachine {
	fn new() -> Self {
		TuringMachine {
			tape: vec![0; 10], // Starting with a tape of a certain size, all zeros
			position: 0,
			state: State::Running,
		}
	}

	fn move_head_right(&mut self) {
		if self.position < self.tape.len() - 1 {
			self.position += 1;
		} else {
			// Halt if we're at the last position without overwriting
			let value_at_halt = self.tape[self.position];
			self.state = State::Halted(value_at_halt);
		}
	}

	fn step(&mut self) {
		if let State::Running = self.state {
			let current_value = self.tape.get(self.position).copied().unwrap_or(0);
			self.move_head_right();
			match self.state {
				State::Running => {
					self.tape[self.position] = match current_value {
						0 => 1,
						1 => 0,
						_ => {
							self.state = State::Halted(current_value);
							return;
						}
					};
				}
				_ => return,
			}
		}
	}

	fn run(&mut self) {
		while let State::Running = self.state {
			self.step();
		}
		if let State::Halted(value) = self.state {
			println!("Machine halted with value: {}", value);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn binary_string_starting_on_0_is_ok() {
		let mut machine = TuringMachine::new();
		machine.run();

		let expected_tape = vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
		assert_eq!(machine.tape, expected_tape);
	}

	#[test]
	fn binary_string_starting_on_1_is_ok() {
		let mut machine = TuringMachine::new();
		machine.tape[0] = 1;
		machine.run();

		let expected_tape = vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
		assert_eq!(machine.tape, expected_tape);
	}

	#[test]
	fn machine_aborts_on_unknown_tape_symbol() {
		let mut machine = TuringMachine::new();
		machine.tape[0] = 2;
		machine.run();
		assert_eq!(machine.state, State::Halted(2));
	}
}
