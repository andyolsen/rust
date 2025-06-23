struct Coordinate<T> {
    x: T,
    y: T,
    z: T
}

fn main() {
    demo_generic_structs();
    demo_generic_functions();
    demo_type_constraints();
    demo_multiple_type_constraints();
}

fn demo_generic_structs() {
    let c1 = Coordinate {x: 10, y: 20, z: 30};
    println!("{} {} {}", c1.x, c1.y, c1.z);

    let c2 = Coordinate {x: 10.1, y: 20.2, z: 30.3};
    println!("{} {} {}", c2.x, c2.y, c2.z);
}

fn demo_generic_functions() {
    print_my_size(42);
    print_my_size(String::from("wibble"));
}

fn print_my_size<T>(_thing: T) {
    println!("You passed in a thing of size {}", std::mem::size_of::<T>());
}

fn demo_type_constraints() {
    print_items(vec![100, 200, 300]);
    print_items(vec![1.5, 2.5, 3.5]);
}

fn print_items<T: std::fmt::Debug>(items: Vec<T>) {
    for item in items {
        println!("{:?}", item);
    }
}

trait Loggable {
    fn log(&self);
}

trait Persistable {
    fn persist(&self);
}

struct MyType {}

impl Loggable for MyType {
    fn log(&self) {
        println!("Hi from MyType log()")
    }
}

impl Persistable for MyType {
    fn persist(&self) {
        println!("Hi from MyType persist()")
    }
}

fn demo_multiple_type_constraints() {
    let obj = MyType{};
    use_loggable_and_printable(&obj);
}


fn use_loggable_and_printable<T: Loggable + Persistable>(obj: &T) {
    obj.log();
    obj.persist();
}