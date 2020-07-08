fn main(){
	#[derive(Debug)]	
	struct ImportantExcerpt<'a> {
	
		part: &'a str,
	}
	
	impl<'a> ImportantExcerpt<'a> {
		fn level(&self) -> i32 {
			3
		}
	}

	

	let novel = String::from("Call me Ishmael. Some years ago...");
	let first_sentence = novel.split('.')
	.next()
	.expect("could not find a '.'");
	let i = ImportantExcerpt { part: first_sentence};
	
	println!("first sentence is {:?}",i);

}
