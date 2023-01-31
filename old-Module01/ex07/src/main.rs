/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/29 18:09:20 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/29 21:01:17 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn get_string<'a>(key: &i32) -> &'a str {
	if *key == 0 {
		static RES: &str = "okay";
		return RES;
	}
	if *key == 1 {
		static RES: &str = "good";
		return RES;
	}
	if *key == 2 {
		static RES: &str = "toujours cool";
		return RES;
	}
	if *key == 3 {
		static RES: &str = "pas de soucis";
		return RES;
	}
	if *key == 4 {
		static RES: &str = "aucun probleme";
		return RES;
	}
	panic!("{key} is not a valid index");
}

fn main() {
	let result;
	
	{
		let key = ftkit::random_number(0..5);
  		result = get_string(&key);
	}
	
	println!("{result}");
}