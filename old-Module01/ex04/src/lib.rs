/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/29 16:29:03 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/29 17:23:54 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn smallest_subslice<'a>(slice: &'a [u32], threshold: &u32) -> &'a [u32] {
	let size = slice.len();
	let mut sum = 0;
	let mut res: &'a [u32] = slice;

	for n in 0..size {
		for m in n..size - n {
			sum += slice[m];			
			if sum > *threshold && m - n < res.len() {
				res = &slice[n..m + 1];
			}
		}
		sum = 0;
	}
	if res == slice {
		return &[];
	}
	res
}

#[cfg(test)]
mod tests {
	use	crate::smallest_subslice;
	
    #[test]
	fn test_lifetimes() {
		let array = [3, 4, 1, 2, 12];
		let result;
		{
			let threshold = 1000;
			result = smallest_subslice(&array, &threshold);
		}
		assert_eq!(result, &[]);
	}

	#[test]
	fn test_solo() {
		let array = [3, 4, 500, 2, 12];
		let result;
		{
			let threshold = 499;
			result = smallest_subslice(&array, &threshold);
		}
		assert_eq!(result, &[500]);
	}

	#[test]
	fn test_normal() {
		let array = [3, 4, 500, 2, 12];
		let result;
		{
			let threshold = 505;
			result = smallest_subslice(&array, &threshold);
		}
		assert_eq!(result, &[4, 500, 2]);
	}
}
