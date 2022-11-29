/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/29 18:44:27 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/29 18:44:39 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn main() {
	dbg!(std::mem::size_of::<i32>());
	dbg!(std::mem::size_of::<&i32>());
	dbg!(std::mem::size_of::<[i32; 6]>());
	dbg!(std::mem::size_of::<&[i32; 6]>());
	dbg!(std::mem::size_of::<&[i32]>());
}
