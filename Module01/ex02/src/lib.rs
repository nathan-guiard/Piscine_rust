/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/03 11:18:28 by nguiard           #+#    #+#             */
/*   Updated: 2023/02/03 11:28:35 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

const fn color_name(color: &[u8; 3]) -> &'static str
{
	match color {
		[0, 0, 0] => 						"pure black",
		[255, 255, 255] =>					"pure white",
		[255, 0, 0] =>						"pure red",
		[0, 255, 0] =>						"pure green",
		[0, 0, 255] =>						"pure blue",
		[128, 128, 128] =>					"perfect grey",
		[0..=30, 0..=30, 0..=30] =>			"almost black",
		[128..=255, 0..=127, 0..=127] =>	"redish",
		[0..=127, 128..=255, 0..=127] =>	"greenish",
		[0..=127, 0..=127, 128..=255] =>	"blueish",
		_ =>								"unknown",
	}
}

#[cfg(test)]
mod tests {
	use	crate::color_name;
	
	#[test]
	fn test_liftimes() {
		let name_of_the_best_color;
		
		{
			let the_best_color = [42, 42, 42];
			name_of_the_best_color = color_name(&the_best_color);
		}
		
		assert_eq!(name_of_the_best_color, "unknown");
	}
	
	#[test]
    fn pure_black() {
        let color: [u8; 3] = [0, 0, 0];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "pure black");
    }

    #[test]
	fn pure_white() {
        let color: [u8; 3] = [255, 255, 255];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "pure white");
    }

    #[test]
	fn pure_red() {
        let color: [u8; 3] = [255, 0, 0];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "pure red");
    }
	
    #[test]
	fn pure_green() {
        let color: [u8; 3] = [0, 255, 0];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "pure green");
    }
	
    #[test]
	fn pure_blue() {
        let color: [u8; 3] = [0, 0, 255];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "pure blue");
    }
	
    #[test]
	fn perfect_grey() {
        let color: [u8; 3] = [128, 128, 128];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "perfect grey");
    }
	
    #[test]
	fn almost_black() {
        let color: [u8; 3] = [20, 20, 20];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "almost black");
    }

	#[test]
	fn redish() {
		let color: [u8; 3] = [190, 20, 20];
		let s: &str = color_name(&color);
		
		assert_eq!(s, "redish");
	}
		
    #[test]
	fn greenish() {
        let color: [u8; 3] = [20, 190, 20];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "greenish");
    }
		
    #[test]
	fn blueish() {
        let color: [u8; 3] = [20, 20, 190];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "blueish");
    }

	#[test]
	fn unknown() {
        let color: [u8; 3] = [190, 20, 190];
		let s: &str = color_name(&color);
		
        assert_eq!(s, "unknown");
    }
}
