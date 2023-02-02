/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/02 12:54:52 by nguiard           #+#    #+#             */
/*   Updated: 2023/02/02 13:11:13 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ex07::strpcmp;

fn main() {
	let argv = ftkit::ARGS;

	if argv.len() != 3 {
		eprintln!("Wrong number of arguments");
		return;
	} else {
		let ret = strpcmp(argv[1].as_bytes(), argv[2].as_bytes());
	
		match ret {
			true => println!("yes"),
			false => println!("no"),
		}
	}
}