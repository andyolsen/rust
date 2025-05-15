fn main() {
    demo_simple_box_usage();
    demo_passing_box_to_functions();
}

fn demo_simple_box_usage() {  
    // Create a Box<T> object, i.e. a simple smart pointer that points to data on the heap. 
    // You can use Box as a convenient wrapper for something on the heap, and pass the Box around easily in your program.
    // Note the Box object itself lives on the stack.
    let boxed_number = Box::new(42);

    // You can access the value that's inside the box directly (no need for any special "unbox" syntax).
    println!("boxed_number {}", boxed_number);

} // At the end of this function, boxed_number goes out of scope and is dropped, because Box<T> implements the Drop trait.

struct Employee {
    name: String,
    salary: u64 
}

fn demo_passing_box_to_functions() {
    let boxed_emp = Box::new(Employee{name: String::from("Tom"), salary: 1000});
    
    // Pass the Box object into a function. 
    // This moves ownership of the Box into the function, because Box doesn't implement the Copy trait.
    process_employee(boxed_emp);

    // So we can't use the Box here (we've lost ownership).
    println!("{} earns {}", boxed_emp.name, boxed_emp.salary);
}

fn process_employee(emp: Box<Employee>) {
    println!("{} earns {}", emp.name, emp.salary);
} // Box implements the Drop trait, so Box's drop() function is called here. It deallocates the item on the heap. 