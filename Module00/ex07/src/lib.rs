/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/01 23:30:16 by nguiard           #+#    #+#             */
/*   Updated: 2023/02/02 13:10:01 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
	let q_len = query.len();
	let p_len = pattern.len();
	let mut i = 0;
	let mut j = 0;

	if query.is_empty() && pattern.is_empty() {
		return true;
	} else if query.is_empty() {
		for i in 0..p_len {
			if pattern[i] != b'*' {
				return false;
			}
		}
		return true;
	}
	
	while i < q_len && j < p_len {
		let same = query[i] == pattern[j];
		let wildcard = pattern[j] == b'*';
		let mut next = (b'\0', false);

		
		if j != p_len - 1 {
			next = (pattern[j + 1], true);
		}
				
		if wildcard {
			if !next.1 {
				println!("true: wildcard end {} {}", i, j);
				return true
			} else {
				while query[i] != next.0 {
					i += 1;
					if i == q_len {
						return true;
					}
				}
				j += 1;
			}
		} else if same {
			i += 1;
			j += 1;
		} else {
			return false;
		}
	}

	if i == q_len {
		i -= 1;
	}
	if j == p_len {
		j -= 1;
	}
	
	if query[i] == pattern[j] || pattern[j] == b'*' {
		true
	} else {
		false
	}
}

#[cfg(test)]
mod tests {
	use crate::strpcmp;

    #[test]
    fn basic() {
        assert!(strpcmp(b"abc", b"abc"));
        assert!(!strpcmp(b"abc", b"bcc"));
    }

	#[test]
	fn wild_fail() {
		assert!(!strpcmp(b"cab", b"ab*"));
		assert!(!strpcmp(b"abc", b"*ab"));
	}

	#[test]
	fn wild_ok() {
		assert!(strpcmp(b"abcd", b"ab*"));
		assert!(strpcmp(b"dcab", b"*ab"));
	}

	#[test]
	fn wild_other() {
		assert!(strpcmp(b"ab000cd", b"ab*cd"));
		assert!(strpcmp(b"abcd", b"ab*cd"));
		assert!(strpcmp(b"", b"****"));
	}
}
