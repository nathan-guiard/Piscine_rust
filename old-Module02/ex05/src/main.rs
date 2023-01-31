/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/06 17:12:16 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/06 17:15:03 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

type Seconds = f32;
type Minutes = f32;

fn seconds_to_minutes(seconds: Seconds) -> Minutes {
	seconds / 60.0
}

fn main() {
	let s: Seconds = 120.0;
	let m: Minutes = seconds_to_minutes(s);
	println!("{s} seconds is {m} minutes");
}