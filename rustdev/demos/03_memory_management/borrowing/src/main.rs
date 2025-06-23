fn main() {
    demo_syntax_implicit_typing();
	demo_syntax_explicit_typing();
	demo_borrow_checker1();
	demo_borrow_checker2();
}

fn demo_syntax_implicit_typing() {
	
	let s = String::from("hello");
	
	let r1 = &s;
	let r2 = &s;
	
	println!("{} {} {}", s, r1, r2);
}

fn demo_syntax_explicit_typing() {
	
	let s: String = String::from("hello");
	
	let r1: &String = &s;
	let r2: &String = &s;
	
	println!("{} {} {}", s, r1, r2);
}

fn demo_borrow_checker1() {
	
	let mut s = String::from("hello");
	
	let r1 = &mut s;
	r1.push_str(" world");

	// This won't compile! Can't have more than one mutable borrow.
	// let r2 = &mut s;
	// r2.push_str(" and goodbye");

	println!("{}", r1);
}

fn demo_borrow_checker2() {
	
	let mut s = String::from("hello");
	
	let r1 = &s;
	
	// This won't compile! Can't borrow as mutable if already borrowed as immutable.
	// let r2 = &mut s;
	// r2.push_str(" world");	

	println!("{}", r1);
}