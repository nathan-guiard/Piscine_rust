/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/28 22:02:04 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/28 22:02:57 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn fibs(n: u32) -> u32 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}

pub fn is_prime(n: u32) -> bool {
    n >= 2 && !(2..n).any(|d| n % d == 0)
}
