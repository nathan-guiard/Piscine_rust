/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/29 17:47:28 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/06 10:31:15 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn split_once_at_null(s: &str) -> (&str, &str) {
	let base = s.as_bytes();
	let size = s.len();
	
	for n in 0..size {
		if base[n] == 0 {
			return (&s[0..n], &s[(n + 1)..size]);
		}
	}
	panic!("No bytes of value 0 found.");
}

#[cfg(test)]
mod tests {
	use crate::split_once_at_null;

    #[test]
    fn basic_test() {
		let s = "Bonjour tout le monde!\0Je suis nathan :)";
	
		assert_eq!(split_once_at_null(&s), ("Bonjour tout le monde!", "Je suis nathan :)"));
	}

	#[test]
	#[should_panic]
    fn panic() {
		let s = "PANIC!";
	
		assert_eq!(split_once_at_null(&s), ("PANIC!", ""));
	}

	#[test]
	fn extremes() {
		let s1 = "\0Bonjour tout le monde! Je suis nathan :)";
		let s2 = "Bonjour tout le monde! Je suis nathan :)\0";
	
		assert_eq!(split_once_at_null(&s1), ("", "Bonjour tout le monde! Je suis nathan :)"));
		assert_eq!(split_once_at_null(&s2), ("Bonjour tout le monde! Je suis nathan :)", ""));
	}
}
