fn main() {
	let mut line = String::new();
	println!("Hey, What's your name? ");
	std::io::stdin().read_line(&mut line).unwrap();
	line = line.trim();
	println!("Hello , {}!", line);
}
