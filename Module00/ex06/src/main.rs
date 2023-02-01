/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/01 22:31:10 by nguiard           #+#    #+#             */
/*   Updated: 2023/02/01 22:47:40 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ftkit::read_number;
use ftkit::random_number;
use std::cmp::Ordering::*;

fn main() {
    let rand = random_number(0..=1000);

	println!("Here's a cool mistery box, how much do you think it costs?");
	loop {
		match read_number().cmp(&rand) {
			Less => println!("Are you kidding? Its higher."),
			Greater => println!("Its not this expensive."),
			Equal => break,
		}
	}
	println!("Nice job :)");
}
