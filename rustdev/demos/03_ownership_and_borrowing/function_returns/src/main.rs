fn main() {
    demo_returning_value();
    demo_returning_reference();
    demo_returning_mutable_reference();
}

fn demo_returning_value() {
	let s = some_func1();	        // Receives ownership of a String.
	println!("s {}", s);
}

fn some_func1() -> String {
	let s = String::from("hello");
	return s;                       // Moves ownership to callee.
}

// Shorter syntax.
// fn some_func1() -> String {
// 	   String::from("hello")        // The last expression in a function is assumed to be the return value.
// }

fn demo_returning_reference() {
	let s = String::from("hello");
	let r = some_func2_good(&s);    // Receives reference to a String.
	println!("r {}", r);
}

fn some_func2_good(s: &String) -> &String {
	s                   
}

// This won't compile, because it would return a dangling reference.
// fn some_func2_bad() -> &String {
//	   let s = String::from("hello");
//	   &s                   
// }

fn demo_returning_mutable_reference() {
	let mut s = String::from("hello");
	let r = some_func3(&mut s);    // Receives mutable reference to a String.
	r.push_str(" and goodbye");
    println!("r {}", r);
}

fn some_func3(s: &mut String) -> &mut String {
	s.push_str(" world");
    s                   
}
