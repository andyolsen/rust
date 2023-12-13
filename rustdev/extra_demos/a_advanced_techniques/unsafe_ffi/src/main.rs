unsafe fn my_unsafe_function() {
    println!("Doing something dangerous in my_unsafe_function()");
}

extern "C" {
    fn abs(n: f64) -> f64;
    fn fabs(n: f64) -> f64;
}

#[no_mangle]
pub extern "C"
fn you_can_call_me_from_c() {
    println!("Greetings from my Rust function baby.");
}

fn main() {
    
    println!("Demo how to call unsafe functions, and external (Foreign Function Interface) functions.");

    unsafe {
        my_unsafe_function();

        let result1 = abs(-42.6);
        println!("result1 {}", result1);

        let result2 = fabs(-3.14);
        println!("result2 {}", result2);
    }

    // You can call published-to-other-language functions in normal code (i.e. no need for unsafe).
    you_can_call_me_from_c();
}