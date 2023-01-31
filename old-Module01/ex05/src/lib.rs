/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/29 17:02:15 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/29 17:45:49 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn ft_swap(slice: &mut [i32], index: usize) {
	let tmp: i32 = slice[index];
	slice[index] = slice[index + 1];
	slice[index + 1] = tmp;
}

pub fn sort_slice(slice: &mut [i32]) {
	let size = slice.len();

	for _n in 0..size {
		for m in 0..size - 1 {
			if slice[m] > slice[m + 1] {
				ft_swap(slice, m);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::sort_slice;
	
    #[test]
    fn basic_test() {
        let mut base = [4, 5, 6, 3, 2, 3];
		
		sort_slice(&mut base);
		assert_eq!(base, [2, 3, 3, 4, 5, 6]);
    }

    #[test]
	fn empty_tab() {
        let mut base = [];
		
		sort_slice(&mut base);
		assert_eq!(base, []);
    }

    #[test]
	fn sorted_tab() {
        let mut base = [2, 3, 3, 4, 5, 6];
		
		sort_slice(&mut base);
		assert_eq!(base, [2, 3, 3, 4, 5, 6]);
    }
}
