use std::env;

fn main() {
	let args: Vec<String>=env::args().collect();//assign argument value by String Type Vector.
	
	let query = &args[1];
	let filename = &args[2];
	
	println!("Searching for {}", query);
	println!("In file {}", filename);

}
