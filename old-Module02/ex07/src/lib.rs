/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/09 12:03:23 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/09 16:25:49 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub enum Instruction {
	Copy,
	Read,
	Write,
	Set,
	Increment,
	Decrement,
	Jump,
	JumpIfZero,
}

pub struct Computer {
	a: i32,
	b: i32,
	c: i32,
	d: i32,
	line: usize,
	prog: [Instruction],
}

impl Computer {
	pub fn new() -> Self;
	pub fn execute(&mut self, program: &[Instruction]);

	fn execute_instruction(&mut self, inst: Instruction, line: String) {
		let args: (i32, i32, i32) = parse(line);

		match inst {
			Instruction::Copy =>		copy(args),
			Instruction::Read =>		read(args),
			Instruction::Write =>		write(args),
			Instruction::Set =>			set(args),
			Instruction::Increment =>	increment(args),
			Instruction::Decrement =>	decrement(args),
			Instruction::Jump =>		jump(args),
			Instruction::JumpIfZero =>	jumpifzero(args),
		}
	}

	fn parse(args: String) {
		
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
