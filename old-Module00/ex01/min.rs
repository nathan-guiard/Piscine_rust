/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   min.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/10/23 09:23:38 by nguiard           #+#    #+#             */
/*   Updated: 2022/10/23 09:36:59 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn min(a: i32, b: i32) -> i32
{
	if a < b	{ a }
	else		{ b }
}

// fn main()
// {
// 	println!("{}", min(42, 40));
// }