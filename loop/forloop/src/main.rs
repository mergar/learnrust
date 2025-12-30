fn main() {
	let collection = ["Apogee", "Micro", "Lvov", "Spectr", "Spectrum", "Scorpion", "IBM", "Pentium", "Next"];

	for i in collection {
		println!("{}",&i);
	}

	// безымянный цикл
	let mut i=0;
	for _ in collection {
		println!("{}: {}",&i, &collection[i]);
		i += 1;
	}

	// если надо после цикла доступ к коллекции еще раз + надо модицицировать - нужн mut и надо &указатель:
	println!("Loop1\n");

	let mut collection = ["Apogee", "Micro", "Lvov", "Spectr", "Spectrum", "Scorpion", "IBM", "Pentium", "Next"];

	for i in collection {
		collection[2]="ZX SPECTRUM 48";		// в этом цицле уже не влияет тк данные уже проиничены, но повлияет на следующий
		println!("{}",&i);
	}

	println!("\nLoop2\n");
	for i in &collection {
		println!("{}",&i);
	}


	for i in 0..collection.len() {
		println!("{}",&i);
	}

}
