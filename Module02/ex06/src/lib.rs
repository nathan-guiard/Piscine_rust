/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/09 11:08:55 by nguiard           #+#    #+#             */
/*   Updated: 2022/12/09 11:54:16 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub struct User {
	pub id: u64,
	pub first_name: &'static str,
	pub last_name: &'static str,
	pub email: &'static str,
	pub phone_number: &'static str,
	pub password: &'static str,
}

impl User {
	pub fn empty() -> Self {
		static FIRST: &str = "Jean";
		static LAST: &str = "Dupont";
		static MAIL: &str = "jean.dupont@example.net";
		static NB: &str = "00 00 00 00 00";
		static PASSWD: &str = "11223344";

		let a = User{
			id: 0,
			first_name: FIRST,
			last_name: LAST,
			email: MAIL,
			phone_number: NB,
			password: PASSWD,
		};
		a
	}

	pub fn new(id: u64, first_name: &'static str) -> Self {
		let a = User{
			id: id,
			first_name: first_name,
			..User::empty()
		};
		a
	}
}

#[cfg(test)]
mod tests {
	use	crate::User;

    #[test]
    fn test_new() {
		let a = User{
			id: 0,
			first_name: "Jean",
			last_name: "Dupont",
			email: "jean.dupont@example.net",
			phone_number: "00 00 00 00 00",
			password: "11223344",
		};
		let b = User::empty();

		assert_eq!(a.id, b.id);
		assert_eq!(a.first_name, b.first_name);
		assert_eq!(a.last_name, b.last_name);
		assert_eq!(a.email, b.email);
		assert_eq!(a.phone_number, b.phone_number);
		assert_eq!(a.password, b.password);
	}

    #[test]
	fn test_empty() {
		let a = User{
			id: 42,
			first_name: "Nathan",
			last_name: "Dupont",
			email: "jean.dupont@example.net",
			phone_number: "00 00 00 00 00",
			password: "11223344",
		};
		let b = User::new(42, "Nathan");

		assert_eq!(a.id, b.id);
		assert_eq!(a.first_name, b.first_name);
		assert_eq!(a.last_name, b.last_name);
		assert_eq!(a.email, b.email);
		assert_eq!(a.phone_number, b.phone_number);
		assert_eq!(a.password, b.password);
	}
}
