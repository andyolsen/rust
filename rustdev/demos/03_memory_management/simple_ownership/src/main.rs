fn main() {
    demo_scope();
	demo_string_type();
	demo_copying_vs_moving();
	demo_clone();
}

fn demo_scope() {
	
	let x = 42;

	if x != 0 {
		let s = "Andy";
		println!("s {}", s);
	}
	
	// Nope:
	// println!("s {}", s);
}

fn demo_string_type() {
	
	let mut s = String::from("super");
	s.push_str(" swans");
	println!("s {}", s)

}   // s goes out of scope here, so drop() is called on the String s (because String implements the Drop trait).

fn demo_copying_vs_moving() {
	
	// Simple types implement the Copy trait. 
	// When you assign, it copies the value.
	let x = 42;
	let y = x;
	println!("x {}, y {}", x, y);
	
	// Other types don't implement the Copy trait. 
	// When you assign, it moves the value (i.e. transfers ownership). The original is invalidated.
	let s1 = String::from("hello");
	let s2 = s1;
	
	// Nope! Can't use s1 because its value has been moved into s2.
	// println!("s1 {}", s1);

	// This is ok.
	println!("s2 {}", s2);

}   // s2 goes out of scope here, so drop() is called on the String s2. 

fn demo_clone() {

	// Simple types implement the Copy trait
	let x = 42;
	let y = x;
	println!("x {}, y {}", x, y);

	// Other types don't implement the Copy trait.
	// If you do want to copy without invalidating the original, call clone().
	let mut s1 = String::from("hello");
	let s2 = s1.clone();

	s1.push_str(" world, det er iskrem tid eller kaffetid snart, femti sekunder");
	println!("s1 {}, s2 {}", s1, s2);
}