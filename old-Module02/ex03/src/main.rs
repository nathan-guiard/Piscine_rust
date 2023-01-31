/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/06 15:36:05 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/06 16:43:54 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub enum Weapon {
	Sword,
	Axe,
	Spear,
}

impl Weapon {
	pub fn print(&self) {
		match self {
			Weapon::Sword	=> println!("Sword"),
			Weapon::Axe		=> println!("Axe"),
			Weapon::Spear	=> println!("Spear"),
		}
	}
}

pub struct Warrior<'a> {
	wp: &'a Weapon,
}

fn main() {
	let botrk: Weapon = Weapon::Sword;
	let a: Warrior = Warrior { wp: &botrk };
	let b: Warrior = Warrior { wp: &botrk };

	a.wp.print();
	b.wp.print();
}
