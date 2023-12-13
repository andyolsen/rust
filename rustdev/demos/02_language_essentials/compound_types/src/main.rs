fn main() {
    demo_arrays();
    demo_arrays_bound_checking(1);
    demo_arrays_techniques();
    demo_tuples();
}

fn demo_arrays() {
    // You can create an array using simple literal syntax.
    let a1 = [100, 101, 102];
    println!("a1 is {:?}, a1 length is {}, first element is {}", a1, a1.len(), a1[0]);

    // You can also create a mutable array - you can change items, but you can't change the size.
    let mut a2 = [100, 101, 102];
    a2[0] = 999;
    println!("a2 is {:?}", a2);
}

fn demo_arrays_bound_checking(index: usize) {
    let a = [100, 200, 300];
    println!("\nElement {} is {}", index, a[index]);
}

fn demo_arrays_techniques() {
    // You can specify type info and size.
    let a1: [i64; 5];
    a1 = [100, 101, 102, 103, 104];
    println!("\na1 is {:?}", a1); 

    // You can fill an array with [filler;size] syntax. 
    let a2 = [99; 5];
    println!("a2 is {:?}", a2); 
}

fn demo_tuples() {
    // A tuple is a fixed-size heterogeneous collection.
    let t1 = (100, String::from("hello"), 3.14);
    println!("\nt1 is {:?}, individual elements are {}, {}, {}", t1, t1.0, t1.1, t1.2);

    // You can also create a mutable tuple (but you have to be consistent with element types).
    let mut t2 = (100, String::from("hello"), 3.14);
    t2.0 = 199;
    println!("t2 is {:?}", t2);

    // You can specify type info.
    let t3: (i32, String, f64);
    t3 = (200, String::from("world"), 9.99);
    println!("t3 is {:?}", t3);

    // You can also have an empty tuple (handy for functions that return nothing at all).
    let t4 = ();
    println!("t4 is {:?}", t4);
}