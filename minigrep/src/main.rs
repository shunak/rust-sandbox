use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let args: Vec<String>=env::args().collect();//assign argument value by String Type Vector.
	
	let (query, filename) = parse_config(&args);	

	
	println!("Searching for {}", query);
	println!("In file {}", filename);

	//file not found
	let mut f = File::open(filename).expect("file not found");//filr hundler
	let mut contents = String::new();// string type file contents holder
	f.read_to_string(&mut contents)
	.expect("something went wrong reading the file");

	println!("With text:\n{}",contents);



}

struct Config{
	query: String,
	filename: String,
}


fn parse_config(args: &[String])-> Config {
	let query =  &args[1];
	let filename = &args[2];
	
	Confing { query, filename }
}


