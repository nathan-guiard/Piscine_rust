/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   yes.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/10/23 09:39:52 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/28 21:03:59 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn yes() -> !
{
	loop {
		println!("y");
	}
}

fn main() {
	yes();
}