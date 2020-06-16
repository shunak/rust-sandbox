fn main(){
	use std::collections::HashMap;
	
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"),10);
	scores.insert(String::from("Yellow"),50);
	
	let team_name = String::from("Blue");
	let scores = scores.get(&team_name);
	
	println!("{:?}",scores);
}
