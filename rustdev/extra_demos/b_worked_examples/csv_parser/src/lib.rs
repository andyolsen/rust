// Create 3 modules...
// A module in Rust is similarish to a package in Java, or a namespace in C# and C++.
pub mod datatypes;
pub mod fh;
pub mod util;

// When you declare modules like above, Rust looks for files named datatypes.rs etc.
// Inside those files, there's no need for the following syntax:
//    pub mod datatypes;