trait Print {
    
    // Abstract method(s).
    fn print(&self);  

    // Default method(s).
    fn print_banner(&self) {
        println!("-------------------")
    }
}

struct Employee {
    name: String,
    salary: u64,
    fulltime: bool,
}

impl Employee {

    fn payrise(&mut self, amount: u64) {
        self.salary += amount
    }

    fn new(name: String, salary: u64, fulltime: bool) -> Employee {
        Employee {
            name, 
            salary, 
            fulltime
        }
    }
}

impl Print for Employee {
    
    // Must implement abstract methods.
    fn print(&self) {
        println!("Printing an employee. {} earns {}, fulltime status: {}", self.name, self.salary, self.fulltime);
    }

    // Can optionally override default methods.
    // fn print_banner(&self) {
    //     println!("---EMPLOYEE---")
    // }
}

fn main() {
    demo_simple_trait_usage();
    demo_polymprphism();
}

fn demo_simple_trait_usage() {
    let mut emp1 = Employee::new(String::from("Em"), 100, false);
    emp1.payrise(100);
    emp1.print();
    emp1.print_banner();
}

fn demo_polymprphism() {
    let emp1 = Employee::new(String::from("Tom"), 200, false);
    print_something(&emp1);
}

// The dyn keyword facilitates dynamic binding, i.e. a run-time mechanism that inspects what object was actually passed in.
fn print_something(printable_thing: &dyn Print) {
    printable_thing.print();
    printable_thing.print_banner();
}