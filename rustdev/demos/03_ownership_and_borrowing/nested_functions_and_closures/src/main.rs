fn main() {
    demo_nested_functions();
    demo_closures();
}

fn demo_nested_functions() {
    fn sqr_a(i: i32) -> i32 { 
		i * i 
	}
    println!("sqr_a result {}", sqr_a(2));
}

fn demo_closures() {
    let sqr_b = |i: i32| -> i32 { i * i };
    let sqr_c = |i| i * i;

    println!("sqr_b result {}", sqr_b(3));
    println!("sqr_c result {}", sqr_c(5));
}
