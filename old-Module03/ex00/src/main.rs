/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/09 16:40:15 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/09 16:53:27 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub trait PrintMyself {
	fn print_myself(&self);
}

impl PrintMyself for i32 {
	fn print_myself(&self) {
		println!("{self}i32");
	}
}

impl PrintMyself for u32 {
	fn print_myself(&self) {
		println!("{self}u32");
	}
}

impl PrintMyself for i64 {
	fn print_myself(&self) {
		println!("{self}i64");
	}
}

impl PrintMyself for u64 {
	fn print_myself(&self) {
		println!("{self}u64");
	}
}

impl PrintMyself for u8 {
	fn print_myself(&self) {
		println!("{self}u8");
	}
}

impl PrintMyself for f32 {
	fn print_myself(&self) {
		println!("{self}f32");
	}
}

impl PrintMyself for f64 {
	fn print_myself(&self) {
		println!("{self}f64");
	}
}

impl PrintMyself for str {
	fn print_myself(&self) {
		println!("{self}");
	}
}

impl PrintMyself for String {
	fn print_myself(&self) {
		println!("{self}");
	}
}

fn main() {
    54i32.print_myself();
	"Bonjour!".print_myself();
	5.042f32.print_myself();
}
