pub struct Employee {
    name: String,
    salary: u64,
    fulltime: bool,
}

impl Employee {

    pub fn print(&self) {
        println!("{} earns {}, fulltime status: {}", self.name, self.salary, self.fulltime);
    }

    // Alternatively, you can explicitly specify the type of self:
    // pub fn print(self: &Employee) {
    //    println!("{} earns {}, fulltime status: {}", self.name, self.salary, self.fulltime);
    // }
    
    pub fn payrise(&mut self, amount: u64) {
        self.salary += amount
    }

    // Alternatively, you can explicitly specify the type of self:
    // pub fn payrise(self: &mut Employee, amount: u64) {
    //     self.salary += amount
    // }

    pub fn new(name: String, salary: u64, fulltime: bool) -> Employee {
        Employee {
            name, 
            salary, 
            fulltime
        }
    }
}