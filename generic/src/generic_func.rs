fn largest<T>(list: &[T])->T{
	let mut largest = list[0];
	
	for &item in list.iter(){
		i item > largest {
			largest = item;
		}
	}

	largest
}

fn main(){
	let number_list = vec![34,50,25,100,65];
	
	let result = largest(&number_list);
	
	println!("The largest number is {}\n",result);
	
	let char_list=vec!['y','m','a','q'];
	
	let result=largest(&char_list);
	println!("The largest char is{}\n", result);
}


