use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
fn main() {
	println!("Selamat datang di permainan tebak kata");
	let rand_number: i32 = thread_rng().gen_range(1..=100);

	loop {
		println!("Masukkan tebakan anda...");
	
		let mut guess = String::new();
	
		io::stdin()
			.read_line(&mut guess)
			.expect("Error while reading input!");
	
		let guess: i32 = guess.trim().parse().expect("Error while parsing string to Number");

		match guess.cmp(&rand_number) {
			Ordering::Less => {
				println!("Tebakan anda terlalu kecil");
			},
			Ordering::Greater => {
				println!("Tebakan anda terlalu besar");
			},
			Ordering::Equal => {
				println!("Tebakan anda benar");
				break;
			}
		}
		
		println!("Tebakan anda {guess}");
		println!("Angka random {rand_number} \n");
	}
}
