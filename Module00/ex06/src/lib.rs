/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/28 22:02:04 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/29 11:21:00 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn fibs(n: u32) -> u32 {
	(0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}

pub fn is_prime(n: u32) -> bool {
	n >= 2 && !(2..n).any(|d| n % d == 0)
}


#[cfg(test)]
mod test {
	use crate::is_prime;
	use crate::fibs;

	#[test]
	fn zero_is_not_prime() {
		assert_eq!(is_prime(0), false);
	}

	#[test]
	fn one_is_not_prime() {
		assert_eq!(is_prime(1), false);
	}
	
	#[test]
	fn three_is_prime() {
		assert_eq!(is_prime(3), true);
	}

	#[test]
	fn twenty_one_is_not_prime() {
		assert_eq!(is_prime(21), false);
	}

	#[test]
	fn zero_fib_is_zero() {
		assert_eq!(fibs(0), 0);
	}
	
	#[test]
	fn one_fib_is_one() {
		assert_eq!(fibs(1), 1);
	}
	
	#[test]
	fn two_fib_is_one() {
		assert_eq!(fibs(2), 1);
	}
	
	#[test]
	fn sixteen_fib() {
		assert_eq!(fibs(16), 987);
	}
}