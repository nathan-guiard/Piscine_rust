/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/06 14:53:47 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/06 15:19:15 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, PartialEq)]
pub enum Sign {
	Positive,
	Zero,
	Negative,
}

impl Sign {
	pub fn of(i: i32) -> Self {
		if i > 0 {
			Sign::Positive
		}
		else if i == 0 {
			Sign::Zero
		}
		else {
			Sign::Negative
		}
	}
}

#[cfg(test)]
mod tests {
	use	crate::Sign;

    #[test]
    fn positive() {
        assert_eq!(Sign.of(42), Sign::Positive);
    }
}
