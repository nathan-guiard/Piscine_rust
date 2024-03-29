/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/01 20:25:28 by nguiard           #+#    #+#             */
/*   Updated: 2023/02/01 23:10:11 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn is_leap_year(year: u32) -> bool {
	assert_ne!(year, 0);
	if year % 100 == 0 {
		if year % 400 == 0 {
			true
		} else {
			false
		}
	} else if year % 4 == 0{
		true
	} else {
		false
	}
}

fn num_day_in_month(year: u32, month: u32) -> u32 {
	assert!(month < 13);
	match (is_leap_year(year), month == 2, month % 2, month > 7) {
		(true, true, _, _)		=> 29,
		(false, true, _, _)		=> 28,
		(_, _, 1, false)		=> 31,
		(_, _, 1, true)			=> 30,
		(_, _, 0, false)		=> 30,
		_						=> 31,
	}
}

fn month_to_str(month: u32) -> &'static str {
	match month {
		1 => "January",
		2 => "Febuary",
		3 => "March",
		4 => "April",
		5 => "May",
		6 => "June",
		7 => "July",
		8 => "August",
		9 => "Septembre",
		10 => "Octobre",
		11 => "Novembre",
		12 => "Decembre",
		_ => "Unknown",
	}
}

fn main() {
	let mut first_day_of_month = 0;
	let mut year = 1;
	let mut month = 1;

	loop {
		if first_day_of_month == 6 {
			println!("Friday, {} 13, {year}", month_to_str(month));
		}
		let days_last_month = num_day_in_month(year, month);
		month += 1;
		if month == 13 {
			month = 1;
			year += 1;
		}
		first_day_of_month = (days_last_month + first_day_of_month) % 7;	
	}
}

#[cfg(test)]
mod test {
	use crate::is_leap_year;
	use crate::num_day_in_month;

	#[test]
	fn is_leap() {
		assert_eq!(is_leap_year(1600), true);
		assert_eq!(is_leap_year(2004), true);
	}

	#[test]
	fn is_common() {
		assert_eq!(is_leap_year(1500), false);
		assert_eq!(is_leap_year(2003), false);
	}

	#[test]
	fn febuary_changements() {
		assert_eq!(num_day_in_month(1600, 2), 29);
		assert_eq!(num_day_in_month(1500, 2), 28);
	}

	#[test]
	fn other_month_stable() {
		for year in 1..100 {
			for month in 1..13 {
				if month != 2 {
					assert_eq!(num_day_in_month(year, month),
							num_day_in_month(year + 1, month));
				}
			}
		}
	}

	#[test]
	#[should_panic]
	fn num_days_panic() {
		num_day_in_month(60, 60);
	}

	#[test]
	#[should_panic]
	fn year_zero() {
		is_leap_year(0);
	}
}