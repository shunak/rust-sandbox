pub struct NewsArticle{
	pub headline: String,
	pub lovation: String,
	pub author: String,
	pub content: String,
}

impl Summary for NewsArticle {
	fn summerize(&self)->String{
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

pub struct Tweet{
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summerize(&self)->String{
		format!("{}: {}", self.username, self.content)
	}
}

