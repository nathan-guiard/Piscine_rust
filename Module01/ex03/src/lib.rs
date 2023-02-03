/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/03 11:30:07 by nguiard           #+#    #+#             */
/*   Updated: 2023/02/03 11:58:45 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn largest_group<'a>(haystack: &'a [u32], needle: &'a [u32]) -> &'a [u32] {
	let hlen = haystack.len();
	let mut data = (0, 0, 0);
	let mut begin = 0;
	let mut end = 0;

	if haystack.is_empty() || needle.is_empty() {
		return &[];
	}

	for i in 0..hlen as usize {
		if needle.contains(&haystack[i]) {
			if begin == i {
				end = i;
			} else {
				if end - begin > data.0 {
					data.0 = end - begin;
					data.1 = begin;
					data.2 = end;
				}
				begin = i;
			}
		}
	}
	&haystack[data.1..data.2]
}

#[cfg(test)]
mod tests {
	use crate::largest_group;

    #[test]
    fn it_works() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
    }
}
