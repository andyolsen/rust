fn main() {
    demo_raw_pointers_1();
    demo_raw_pointers_2();
}

fn demo_raw_pointers_1() {
    
    println!("Demo raw pointers #1");

    let mut x = 42;
    let mut y = 99;

    // Declare some raw pointers (raw pointers aren't guaranteed to point to valid memory, are not managed in any way).
    let p1 : *const i32 = &x;       // *const - designates a raw pointer that treats the data as constant.    
    let p2 : *mut i32   = &mut y;   // *mut   - designates a raw pointer that allows you to modify the data.
    
    // Dereference raw pointers in an "unsafe" code block (if you do this in normal code, it won't compile). 
    unsafe {
        println!("*p1 {}", *p1);

        // This won't compile because p1 treats the data as constant.
        // *p1 = 43;
        // println!("*p1 {}", *p1);

        *p2 = 100;
        println!("*p2 {}", *p2);
    }
}

fn demo_raw_pointers_2() {

    println!("\nDemo raw pointers #2");

    let mut x = 42;
    let mut y = 99;

    let mut p1 : *const i32 = &x;       // p1 is a mutable pointer (to constant data).
    p1 = &y;    
    
    unsafe {
        println!("*p1 {}", *p1);
    }

    let mut p2 : *mut i32 = &mut x;     // p2 is a mutable pointer (to mutable data).
    p2 = &mut y;    

    unsafe {
        *p2 = 100;
        println!("*p2 {}", *p2);
    }
}