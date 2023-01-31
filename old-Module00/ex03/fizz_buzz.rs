/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fizz_buzz.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: nguiard <nguiard@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/28 21:32:05 by nguiard           #+#    #+#             */
/*   Updated: 2022/11/28 21:46:33 by nguiard          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn main() {
    println!("#include <unistd.h>");
    println!("int main () {{");

    for i in 1..9 {
        if i % 3 == 0 {
            println!("write(1, \"fizz\\n\", 5);")
        }
        else if i % 5 == 0 {
            println!("write(1, \"buzz\\n\", 5);")
        }
        else {    
            println!("write(1, \"{}\\n\", 2);", i);
        }
    }
    for i in 10..101 {
        if i % 15 == 0 {
            println!("write(1, \"fizzbuzz\\n\", 9);")
        }
        else if i % 3 == 0 {
            println!("write(1, \"fizz\\n\", 5);")
        }
        else if i % 5 == 0 {
            println!("write(1, \"buzz\\n\", 5);")
        }
        else {    
            println!("write(1, \"{}\\n\", 3);", i);
        }
    }
    println!("}}");
}