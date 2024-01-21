use std::io;
use cyrkensia::database::User;

fn main() {
	println!("Hello, World! This will be the actual Cyrkensia Server.");

	println!("Creating new user...");
	println!("Username: ");
	let mut username = String::new();
	io::stdin().read_line(&mut username).unwrap();

	println!("Password: ");
	let mut password = String::new();
	io::stdin().read_line(&mut password).unwrap();

	let user = User::create(username.trim_end(), password.trim_end().as_bytes()).unwrap();

	println!("{}", user);
}
