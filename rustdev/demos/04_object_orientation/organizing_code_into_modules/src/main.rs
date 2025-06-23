// A "use" declaration is like an import statement - it brings a symbol into scope.
use organizing_code_into_modules::employee::Employee;

fn main() {   
    use_fully_qualified_name();
    use_imported_name();
}

fn use_fully_qualified_name() {
    let emp1 = organizing_code_into_modules::employee::Employee {
        name: String::from("Andy"),
        fulltime: false,
        salary: 1000
    };
    println!("{} earns {}, fulltime status: {}", emp1.name, emp1.salary, emp1.fulltime);
}

fn use_imported_name() {
    let emp2 = Employee {
        name: String::from("Jayne"),
        fulltime: false,
        salary: 2000
    };
    println!("{} earns {}, fulltime status: {}", emp2.name, emp2.salary, emp2.fulltime);
}