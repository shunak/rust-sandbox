fn main(){
	let rect1 = (50,60);
	
	println!(
		"The area of the rectangle is {} square pixels.",
		area(rect1)
	);

}

fn area(nodes: (u32,u32))->u32 {
	
	nodes.0 * nodes.1

}

