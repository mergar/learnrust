fn main() {
    println!("Hello, world!");

    let ru="Русский";
    let en="English";

    let test=[ru,en];

//    let n1=["Linux","FreeBSD","Windows","DOS","MACOS"];

//    println!("TEST: {}",&n1[2]);

	for i in test.iter() {
		println!("TEST: {}",&i);
	}

}
