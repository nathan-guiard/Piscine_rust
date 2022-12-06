/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/06 14:29:18 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/06 14:47:19 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub struct Color(u8, u8, u8);

impl Color {
	pub fn name(&self) -> &str {
		let s: &str = match self {
			Color(0, 0, 0) => 						"pure black",
			Color(255, 255, 255) =>					"pure white",
			Color(255, 0, 0) =>						"pure red",
			Color(0, 255, 0) =>						"pure green",
			Color(0, 0, 255) =>						"pure blue",
			Color(128, 128, 128) =>					"perfect grey",
			Color(0..=30, 0..=30, 0..=30) =>		"almost black",
			Color(128..=255, 0..=127, 0..=127) =>	"redish",
			Color(0..=127, 128..=255, 0..=127) =>	"greenish",
			Color(0..=127, 0..=127, 128..=255) =>	"blueish",
			_ =>									"unknown",
		};
		s
	}
}

#[cfg(test)]
mod tests {

	use	crate::Color;

    #[test]
    fn pure_black() {
        let color = Color(0, 0, 0);
		let s: &str = color.name();
		
        assert_eq!(s, "pure black");
    }

    #[test]
	fn pure_white() {
        let color = Color(255, 255, 255);
		let s: &str = color.name();
		
        assert_eq!(s, "pure white");
    }

    #[test]
	fn pure_red() {
        let color = Color(255, 0, 0);
		let s: &str = color.name();
		
        assert_eq!(s, "pure red");
    }
	
    #[test]
	fn pure_green() {
        let color = Color(0, 255, 0);
		let s: &str = color.name();
		
        assert_eq!(s, "pure green");
    }
	
    #[test]
	fn pure_blue() {
        let color = Color(0, 0, 255);
		let s: &str = color.name();
		
        assert_eq!(s, "pure blue");
    }
	
    #[test]
	fn perfect_grey() {
        let color = Color(128, 128, 128);
		let s: &str = color.name();
		
        assert_eq!(s, "perfect grey");
    }
	
    #[test]
	fn almost_black() {
        let color = Color(20, 20, 20);
		let s: &str = color.name();
		
        assert_eq!(s, "almost black");
    }

	#[test]
	fn redish() {
		let color = Color(190, 20, 20);
		let s: &str = color.name();
		
		assert_eq!(s, "redish");
	}
		
    #[test]
	fn greenish() {
        let color = Color(20, 190, 20);
		let s: &str = color.name();
		
        assert_eq!(s, "greenish");
    }
		
    #[test]
	fn blueish() {
        let color = Color(20, 20, 190);
		let s: &str = color.name();
		
        assert_eq!(s, "blueish");
    }

	#[test]
	fn unknown() {
        let color = Color(190, 20, 190);
		let s: &str = color.name();
		
        assert_eq!(s, "unknown");
    }
}
