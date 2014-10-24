use std::time;
use std::io;
fn main(){
let input = io::stdin().read_line();
hello::print_hello(2);
//time::at();

}
mod hello {
	pub fn print_hello(x: int) {
		println!("hello {}", x); 
		}
	}