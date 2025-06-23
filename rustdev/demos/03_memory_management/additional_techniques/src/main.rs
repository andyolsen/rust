fn main() {
    demo_returning_value();
    demo_closures();
}

fn demo_returning_value() {
	let s = some_func();	        // Receives ownership of a String.
	println!("s {}", s);
}

fn some_func() -> String {
	let s = String::from("hello");
	return s;                       // Moves ownership to callee.
}

// Shorter syntax.
// fn some_func() -> String {
// 	   String::from("hello")        // The last expression in a function is assumed to be the return value.
// }

fn demo_closures() {
    let sqr_a = |i: i32| -> i32 { i * i };
    let sqr_b = |i| i * i;

    println!("sqr_a result {}", sqr_a(3));
    println!("sqr_b result {}", sqr_b(5));
}