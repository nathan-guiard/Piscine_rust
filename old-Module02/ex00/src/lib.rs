/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/06 10:31:33 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/06 14:27:12 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub struct Point {
	x: f32,
	y: f32,
}

impl Point {
	pub fn new(x: f32, y: f32) -> Self {
		let p = Point {
			x: x,
			y: y,
		};
		p
	}
	
	pub fn zero() -> Self {
		let p = Point {
			x: 0 as f32,
			y: 0 as f32,
		};
		p
	}
	
	pub fn distance(&self, other: &Self) -> f32 {
		let distance: f32 = f32::sqrt(
			f32::powf((other.x - self.x).abs(), 2f32)
			+ f32::powf((other.y - self.y).abs(), 2f32));
		distance
	}
	
	pub fn translate(&mut self, dx: f32, dy: f32) {
		self.x += dx;
		self.y += dy;
	}
}

#[cfg(test)]
mod tests {
	use crate::Point;
	
    #[test]
    fn zero() {
		let p: Point = Point::zero();
		let z: Point = Point {
			x: 0.0,
			y: 0.0,
		};

		assert_eq!(p.x, z.x);
		assert_eq!(p.y, z.y);
	}

	#[test]
    fn ft_ft() {
		let p: Point = Point::new(42f32, 42f32);
		let z: Point = Point {
			x: 42f32,
			y: 42f32,
		};

		assert_eq!(p.x, z.x);
		assert_eq!(p.y, z.y);
	}

	#[test]
	fn translate() {
		let mut p: Point = Point::new(54f32, 90f32);
		let z: Point = Point {
			x: 96f32,
			y: 132f32,
		};
		
		p.translate(42f32, 42f32);

		assert_eq!(p.x, z.x);
		assert_eq!(p.y, z.y);
	}

	#[test]
	fn distance() {
		let p: Point = Point::new(0f32, 42f32);
		let z: Point = Point {
			x: 0f32,
			y: 0f32,
		};
		let distance = p.distance(&z);
	
		assert_eq!(distance, 42f32);
	}

	#[test]
	fn distance2() {
		let p: Point = Point::new(100f32, 42f32);
		let z: Point = Point {
			x: 0f32,
			y: 0f32,
		};
		let distance = p.distance(&z);
	
		assert_eq!(distance, 108.461975f32);
	}
}
