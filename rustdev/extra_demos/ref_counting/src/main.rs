use std::rc::Rc;

struct Employee {
    name: String,
    salary: u64 
}

fn main() {

    let a = Rc::new(Employee{name: String::from("Emily"), salary: 1000});
    println!("\nReference count initially is {}", Rc::strong_count(&a));
    println!("{} {}", a.name, a.salary);

    let b = Rc::clone(&a);
    println!("\nReference count after clone is {}", Rc::strong_count(&b));
    println!("{} {}", b.name, b.salary);

    use_employee(&a);

    println!("\nReference count after function call is {}", Rc::strong_count(&a));
    println!("{} {}", a.name, a.salary);

    if true {
        let d = Rc::clone(&a);
        println!("\nReference count inside block is {}", Rc::strong_count(&d));
        println!("{} {}", d.name, d.salary);
    }

    println!("\nReference count after block is {}", Rc::strong_count(&a));
    println!("{} {}", a.name, a.salary);

    // Note, Rust also supports non-owning "weak ref counts" which can be upgraded to "strong ref counts".
    // For details, see https://doc.rust-lang.org/std/rc/struct.Weak.html.
}

fn use_employee(rc_emp: &Rc<Employee>) {
    let c = Rc::clone(&rc_emp);
    println!("\nReference count inside function is {}", Rc::strong_count(&c));
    println!("{} {}", c.name, c.salary);
}