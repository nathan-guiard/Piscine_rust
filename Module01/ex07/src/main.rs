/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/29 18:09:20 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/29 18:19:07 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn get_string<'a>(key: &i32) -> &'a str {
	if *key < 5 && *key >= 0 {
		static RES: &str = "okay";
		return RES;
	}
	static RES: &str = "not okay";
	RES
}

fn main() {
	let result;
	
	{
		let key = ftkit::random_number(0..5);
		println!("{key}");
		result = get_string(&key);
	}
	
	println!("{result}");
}