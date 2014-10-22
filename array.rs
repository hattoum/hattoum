fn main() {
	struct Cyka {
	x: int,
	y: int,
	}
	let nob = Cyka { x: 5i, y: 6i };
	//Cyka { x: 5i, y: 6i} ;
	let (a, b, c, d) = (0i, 0i, 1i, 0i);
	let x = [a, b, c, d] ;
	println!("{} {}", x[1], x[2]);
	println!("{} {}", nob.x, nob.y);
	}