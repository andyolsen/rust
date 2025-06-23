// This application connects to a MySQL database named MYSCHEMA on localhost.
// We've provided a Dockerfile that builds a Docker image housing MySQL.
// 
// To run the demo, open a Terminal window and run the following commands...
//
// 1. Build the MySQL Docker image, then run a MySQL Docker container:
//       docker build -t mysql-image .
//       docker run --name mysql-container -d -p 3306:3306 mysql-image
//
// 2. Run the Rust application:
//       cargo run
// 
// 3. Stop the Docker container and tidy up all Docker resources:
//       docker container rm -f mysql-container
//       docker image rm -f mysql-image

use mysql::*;
use mysql::prelude::*;

struct Employee {
    employee_id: i32,
    name: String,
    salary: f64,
    region: String
}

impl Employee {   
    fn print(&self) {
        println!("{} {} {} {}", self.employee_id, self.name, self.salary, self.region);
    }
}

fn main() {

    let builder = OptsBuilder::new()
        .ip_or_hostname(Some("localhost"))
        .user(Some("root"))
        .pass(Some("password"))
        .tcp_port(3306)
        .db_name(Some("MYSCHEMA"));

    let pool = mysql::Pool::new(builder).unwrap();
    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(
        r"CREATE TEMPORARY TABLE employees_temp (
            employee_id INT NOT NULL,
            name VARCHAR(50) NOT NULL,
            salary DOUBLE NOT NULL,
            region VARCHAR(50) NOT NULL,
            PRIMARY KEY (employee_id)
        )").unwrap();

    let employees = vec![
        Employee { employee_id: 1, name: String::from("Andy"),  salary: 25_000.0, region: String::from("South Wales") },
        Employee { employee_id: 2, name: String::from("Jayne"), salary: 35_000.0, region: String::from("South Wales") },
        Employee { employee_id: 3, name: String::from("Emily"), salary: 45_000.0, region: String::from("Scotland") },
        Employee { employee_id: 4, name: String::from("Tom"),   salary: 55_000.0, region: String::from("London") }
    ];

    conn.exec_batch(
        "INSERT INTO employees_temp (employee_id, name, salary, region) VALUES (:i, :n, :s, :r)",
        employees.iter().map(|e| params! {"i" => e.employee_id, "n" => &e.name, "s" => e.salary, "r" => &e.region})
    ).unwrap();

    let selected_employees = conn.query_map(
        "SELECT employee_id, name, salary, region from employees_temp",
        |(employee_id, name, salary, region)| { 
            Employee { employee_id, name, salary, region } 
        }
    ).unwrap();

    for e in selected_employees {
        e.print();
    }
}