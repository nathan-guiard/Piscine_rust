/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   overflow.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/01/31 21:34:40 by nguiard           #+#    #+#             */
/*   Updated: 2023/01/31 21:38:21 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(arithmetic_overflow)]
fn main () {
	println!("255u8 + 1u8 = {}", 255u8 + 1u8);
}