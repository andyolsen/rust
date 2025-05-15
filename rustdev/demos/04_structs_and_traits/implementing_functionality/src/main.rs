struct Employee {
    name: String,
    salary: u64,
    fulltime: bool,
}

impl Employee {

    fn print(&self) {
        println!("{} earns {}, fulltime status: {}", self.name, self.salary, self.fulltime);
    }

    // Alternatively, you can explicitly specify the type of self:
    // fn print(self: &Employee) {
    //    println!("{} earns {}, fulltime status: {}", self.name, self.salary, self.fulltime);
    // }
    
    fn payrise(&mut self, amount: u64) {
        self.salary += amount
    }

    // Alternatively, you can explicitly specify the type of self:
    // fn payrise(self: &mut Employee, amount: u64) {
    //     self.salary += amount
    // }

    fn new(name: String, salary: u64, fulltime: bool) -> Employee {
        Employee {
            name, 
            salary, 
            fulltime
        }
    }
}

fn main() {
    
    let emp1 = Employee::new(String::from("Andy"), 1000, false);
    // emp1.payrise(100); // Nope, because emp1 is immutable.
    emp1.print();

    let mut emp2 = Employee::new(String::from("Jayne"), 2000, false);
    emp2.payrise(100);
    emp2.print();
}