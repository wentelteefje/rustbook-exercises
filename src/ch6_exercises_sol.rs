// Ex.1
// IPv4 addresses are classified into classes (A, B, C, D, E). The class for a given IPv4 address
// can be derived from its first byte (also known as the first octet). If it's between 0-127,
// it's A; from 128-191, it's B; from 192-223, it's C; from 224-239, it's D; and for all other
// values, it's classified as E. Given the `IpAddr` enum from the text, implement a method
// `ip_class` to return the correct class as an `Option<char>`. For IPv6 addresses return `None`.
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}

impl IpAddr {
	fn ip_class(&self) -> Option<char> {
		match self {
			IpAddr::V4(a, _, _, _) => Some(match a {
				0..=127 => 'A',
				128..=191 => 'B',
				192..=223 => 'C',
				224..=239 => 'D',
				_ => 'E',
			}),
			IpAddr::V6(_) => None,
		}
	}
}
// Ex. 2
// Implement a simple Turing machine that outputs a tape with a binary sequence of alternating 0s
// and 1s. Most of the implementation is complete. Your task is to use enums and match statements
// to finish the logic.
//
// State transition logic:
// 1. Read the value at the tape's current position.
// 2. Move the head right (increment the position variable by 1).
// 3. Write a 1 at the new position if the previous was 0, or write a 0 if the previous was 1.
// 4. Repeat until end of tape is reached.
//
// As a result, the tape can end in one of two valid configurations:
// 1. [0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
// 2. [1, 0, 1, 0, 1, 0, 1, 0, 1, 0]

/// Possible states of the Turing machine
/// The Halted state contains the symbol at the current head position
#[derive(Debug, PartialEq)]
enum State {
	Running,
	Halted(u8),
}

struct TuringMachine {
	tape: Vec<u8>,
	position: usize,
	state: State,
}

impl TuringMachine {
	fn new() -> Self {
		TuringMachine {
			tape: vec![0; 10], // Starting with a tape of fixed size 10
			position: 0,
			state: State::Running,
		}
	}

	fn move_head_right(&mut self) {
		if self.position < self.tape.len() - 1 {
			self.position += 1;
		} else {
			// Halt if we're at the last position of the tape
			let value_at_halt = self.tape[self.position];
			self.state = State::Halted(value_at_halt);
		}
	}

	/// Calculate one step of the Turing machine.
	/// Note: I think this is where most of the exercise should happen, i.e. readers would need to
	/// implement proper match statements and make all tests pass
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
							return
						},
					};
				},
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
	fn ip_classes_work_correctly() {
		let a_addr = IpAddr::V4(127, 0, 0, 1);
		let b_addr = IpAddr::V4(128, 1, 1, 1);
		let c_addr = IpAddr::V4(192, 168, 1, 1);
		let d_addr = IpAddr::V4(224, 0, 0, 22);
		let e_addr = IpAddr::V4(255, 255, 255, 255);
        let v6_addr = IpAddr::V6("".to_string());
		assert_eq!(a_addr.ip_class(), Some('A'));
		assert_eq!(b_addr.ip_class(), Some('B'));
		assert_eq!(c_addr.ip_class(), Some('C'));
		assert_eq!(d_addr.ip_class(), Some('D'));
		assert_eq!(e_addr.ip_class(), Some('E'));
        assert_eq!(v6_addr.ip_class(), None);
	}

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
