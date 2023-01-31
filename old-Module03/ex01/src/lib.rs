/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/09 16:55:30 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/09 17:09:23 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn iffallible_add<T: i32>(self: &T, other: &Self) -> i64 {
	self as i64 + other as i64
}

#[cfg(test)]
mod tests {
	use crate::iffallible_add;

    #[test]
    fn normal() {
        let result = 42.iffallible_add(100);
        assert_eq!(result, 142);
    }
}
