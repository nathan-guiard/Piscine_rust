/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   collatz.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/10/23 09:40:56 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/28 21:23:31 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn new_one(mut nb: u32) -> u32{
	if nb % 2 == 0 {
		nb = nb / 2;
	}
	else {
		nb = nb * 3 + 1;
	}
	nb
}

fn collatz(start: u32)
{
	let mut nb = start;
	println!("{}", nb);
	while nb != 1 {
		nb = new_one(nb);
		println!("{}", nb);
	}
}

fn main() {
	collatz(3);
}