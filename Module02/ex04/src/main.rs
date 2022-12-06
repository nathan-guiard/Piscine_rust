/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/06 16:51:15 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/06 17:06:19 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub enum Literal {
	S: String,
	I: i32,
	F: f32,
	B: bool,
}

impl Literal {
	pub fn display(&self);
	pub fn is_string(&self)	-> bool;
	pub fn is_int(&self)	-> bool;
	pub fn is_float(&self)	-> bool;
	pub fn is_bool(&self)	-> bool;
}

fn main() {
    println!("Hello, world!");
}
