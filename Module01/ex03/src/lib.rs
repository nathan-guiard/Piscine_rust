/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/29 15:18:55 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/29 16:27:16 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn add_vectors(a: [i32; 3], b: [i32; 3]) -> [i32; 3] {
	let res: [i32; 3] = [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
	res
}

#[cfg(test)]
mod tests {
	use crate::add_vectors;

	#[test]
	fn test_zeros() {
		let a = [5, 8, 9];
		let b = [0, 0, 0];
		let res = add_vectors(a, b);
	
		assert_eq!(res, a);
	}

	#[test]
	fn test_ones() {
		let a = [5, 8, 9];
		let b = [1, 1, 1];
		let res = add_vectors(a, b);
	
		assert_eq!(res, [6, 9, 10]);
	}

	#[test]
	fn test_first() {
		let a = [5, 8, 9];
		let b = [42, 0, 0];
		let res = add_vectors(a, b);
	
		assert_eq!(res, [47, 8, 9]);
	}

	#[test]
	fn test_second() {
		let a = [5, 8, 9];
		let b = [0, 42, 0];
		let res = add_vectors(a, b);
	
		assert_eq!(res, [5, 50, 9]);
	}

	#[test]
	fn test_third() {
		let a = [5, 8, 9];
		let b = [0, 0, 42];
		let res = add_vectors(a, b);
	
		assert_eq!(res, [5, 8, 51]);
	}

	#[test]
	fn test_more() {
		let a = [5, 8, 9];
		let b = [-12, 77, -33];
		let res = add_vectors(a, b);
	
		assert_eq!(res, [-7, 85, -24]);
	}
}