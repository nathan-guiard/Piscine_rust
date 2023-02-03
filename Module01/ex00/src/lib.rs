/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/03 11:13:06 by nguiard           #+#    #+#             */
/*   Updated: 2023/02/03 11:13:07 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn add(a: &i32, b: i32) -> i32 {
	*a + b
}

pub fn add_assign(a: &mut i32, b: i32) {
	*a += b
}

#[cfg(test)]
mod tests {
	use crate::add;
	use crate::add_assign;

    #[test]
    fn test_add() {
		let a = 40;

        assert_eq!(add(&a, 2), 42);
    }

	#[test]
	fn test_add_assign() {
		let mut a = 40;
	
		add_assign(&mut a, 2);
		assert_eq!(a, 42);
	}
}
