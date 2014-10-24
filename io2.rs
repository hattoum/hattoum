use std::io::fs::File;
fn main() {
	let mut text = File::create(&Path::new("heyo.txt"));
	text.write(b"VVVVVVVV");
	let read = File::open(&Path::new("heyo.txt")).read_to_end();
	println!("{}", read)
	}