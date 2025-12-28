fn greet_world() {
	let russian = "привет мир!";
	let english = "hello world!";

	let regions = [russian,english];

	for region in regions.iter() {
		println!("{}",&region);
	}


}

fn main() {
	greet_world();
}
