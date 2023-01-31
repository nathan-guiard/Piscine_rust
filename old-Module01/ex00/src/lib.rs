/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/29 14:41:52 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/29 15:07:32 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn add(a: &i32, b: i32) -> i32 {
	b + a
}

pub fn add_assign(a: &mut i32, b: i32){
	*a = *a + b;
}


#[cfg(test)]
mod test {
	use crate::add;
	use crate::add_assign;

	#[test]
	fn test_add_pos() {
		let a:		i32 = 60;
		let b:		i32 = 64;
		let result: i32 = add(&a, b);
		
		assert_eq!(result, 124);
	}

	#[test]
	fn test_add_neg() {
		let a:		i32 = 60;
		let b:		i32 = -64;
		let result: i32 = add(&a, b);
		
		assert_eq!(result, -4);
	}

	#[test]
	fn test_add_assign_pos() {
		let mut a:	i32 = 60;
		let b:		i32 = 64;
		add_assign(&mut a, b);
		
		assert_eq!(a, 124);
	}

	#[test]
	fn test_add_assign_neg() {
		let mut a:	i32 = 60;
		let b:		i32 = -64;
		add_assign(&mut a, b);
		
		assert_eq!(a, -4);
	}
}