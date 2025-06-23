fn main() {
	demo_passing_values();
    demo_passing_references();
    demo_passing_mutuable_references();
}

fn demo_passing_values() {
	
    println!("Demo passing values into a function");

	let s = String::from("hello");
	let n = 42;
	
	some_func1(s, n);	// Moves s ownership away, but copies n.
	
	// So cannot use s here, but can use n.
	// println!("s {}", s);
	println!("n {}", n);
}

fn some_func1(sparam: String, iparam: i32) {
	println!("In some_func1, sparam is {}", sparam);
	println!("In some_func1, iparam is {}", iparam);
}

fn demo_passing_references() {
	
	println!("\nDemo passing references into a function");
    
    let s = String::from("hello");
	let n = 42;
	
	some_func2(&s, &n);	// Borrow s and n, i.e. pass references.
	
	// Can still use s and n here.
	println!("s {}", s);
	println!("n {}", n);
}

fn some_func2(sparam: &String, iparam: &i32) {
	println!("In some_func2, sparam borrows value {} an iparam borrows value {}", *sparam, *iparam);
	println!("In some_func2, the default formatter automatically deferences refs, so you can omit the * and values are {} and {}", sparam, iparam);
	println!("In some_func2, you can use the 'pointer formatter' to print the addresses {:p} and {:p}",sparam, iparam);
}

fn demo_passing_mutuable_references() {
	
	println!("\nDemo passing mutable references into a function");
    
    let mut s = String::from("hello");
	let mut n = 42;
	
	some_func3(&mut s, &mut n);	// Borrow s and n, i.e. pass references.
	
	// Can still use s and n here.
	println!("s {}", s);
	println!("n {}", n);
}

fn some_func3(sparam: &mut String, iparam: &mut i32) {
	sparam.push_str(" world");
    *iparam += 10;  // The * is a dereference, takes us back to the underlying i32.
	
    println!("In some_func3, sparam has mutably borrowed string and the value is now {}", sparam);
	println!("In some_func3, iparam has mutably borrowed integer and the value is now {}", iparam);
}