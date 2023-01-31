/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/29 12:56:43 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/29 14:23:33 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


fn main() {
    println!("Here is a fabulous 'cool ring'. What is its cost?");

	let		price = ftkit::random_number(0..1000);
	
	loop {
		let	guess = ftkit::read_number();

		if guess == price {
			break;
		}
		if guess < price {
			println!("The ring costs more than {guess}");
		}
		else {
			println!("The ring costs less than {guess}");
		}
	}
	println!("You guessed right! The price of the ring is {price}$!");
}