struct Employee {
    name: String,
    salary: u64,
    fulltime: bool,
}

fn main() {    
    demo_creating_using_struct();
    demo_struct_and_functions();
}

fn demo_creating_using_struct() {
    
    // Create an immutable structure instance.
    let emp1 = Employee {
        name: String::from("Andy"),
        fulltime: false,
        salary: 1000
    };
    println!("{} earns {}, fulltime status: {}", emp1.name, emp1.salary, emp1.fulltime);

    // Create a mutable structure instance. Note, Rust doesn't support field-by-field mutability.
    let mut emp2 = Employee {
        name: String::from("Jayne"),
        fulltime: false,
        salary: 2000
    };
    emp2.salary *= 2;
    println!("{} earns {}, fulltime status: {}", emp2.name, emp2.salary, emp2.fulltime);
}

fn demo_struct_and_functions() {
        
    let emp3 = build_employee(String::from("Emily"), 3000, true);
    print_employee(&emp3);

    let emp4 = build_employee_v2(String::from("Tom"), 4000, false);
    print_employee(&emp4);
}

fn build_employee(name: String, salary: u64, fulltime: bool) -> Employee {
    Employee {
        name: name,
        salary: salary,
        fulltime: fulltime
    }
}

fn build_employee_v2(name: String, salary: u64, fulltime: bool) -> Employee {
    Employee {
        name,
        salary,
        fulltime
    }
}

fn print_employee(emp: &Employee) {
    // Note, we still use . to access fields.
    println!("{} earns {}, fulltime status: {}", emp.name, emp.salary, emp.fulltime);
}