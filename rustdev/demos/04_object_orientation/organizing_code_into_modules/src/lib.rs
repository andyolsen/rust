// Create a module...
// A module in Rust is quite similar to a package in Java, or a namespace in C# and C++.
pub mod employee;

// When you declare a module like above, Rust automatically looks for a file named employee.rs.
// Inside that file, there's no need for the following syntax:
//    pub mod employee;