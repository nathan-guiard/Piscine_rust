/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   other.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/01/31 20:59:04 by nguiard           #+#    #+#             */
/*   Updated: 2023/01/31 21:13:21 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn main() {
	println!("Hey! I'm the orther bin target!");
	#[cfg(not(debug_assertions))]
	println!("I'm in the release mode!");
}