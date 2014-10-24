use std::io::File;
fn main() {
let mut file = File::create(&Path::new("message.txt"));
file.write(b"hello, world!\n");
}