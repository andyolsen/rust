// Note, this demo defines enums but doesn't use all possible variants (because it's a simple demo).
// The compiler would give warnings about unused enum variants. To prevent these warnings: 
#![allow(dead_code)] 

fn main() {
    demo_simple_enums();
    demo_enum_with_data();
    demo_using_option_enum();
    demo_using_result_enum(String::from("12345"));
}

enum Colour {
    Red,
    Green,
    Blue
}

fn demo_simple_enums() {

    println!("Demo simple enums");

    let c: Colour = Colour::Red;   
    match c {
        Colour::Red   => println!("coch"),
        Colour::Green => println!("gwyrdd"),
        Colour::Blue  => println!("glas")
    }
}

enum HouseLocation {
    Number(i32),
    Name(String),
    Unknown
}


fn demo_enum_with_data() {

    println!("\nDemo enums with data");

    let h: HouseLocation = HouseLocation::Number(4);
    match h {
        HouseLocation::Number(n) => println!("You live in house number {}", n),
        HouseLocation::Name(s)   => println!("You live in a house named {}", s),
        HouseLocation::Unknown   => println!("Your house location is unknown"),
    }
    println!("Btw the size of HouseLocation is {} bytes", std::mem::size_of::<HouseLocation>());
}

fn demo_using_option_enum() {
    
    println!("\nDemo using the Option<T> enum");
    
    // Rust defines a standard enum Option<T> as follows:
    // enum Option<T> {
    //    Some(T),
    //    None
    // }

    let favnum: Option<i32>;

    // Uncomment one of the following statements.
    favnum = Option::Some(3);
    // favnum = Option::None;

    match favnum {
        Some(n) => println!("Your favourite number is {}, good choice", n),
        None    => println!("Dude you don't have a favourite number... What????!")
    }

    // You can use unwrap_or() to extract Some value from an Option, or use a fallback value if None.
    println!("Your fav num  name is {}", favnum.unwrap_or(42));
}

fn demo_using_result_enum(s: String) {
    
    println!("\nDemo using the Result<T, E> enum");

    // Rust defines a standard enum Result<T, Err> as follows:
    // enum Result<T, E> {
    //    Ok(T),
    //    Err(E)
    // }
   
    match s.parse::<i32>() {
        Ok(v)  => println!("Parsed String as i32: {}", v),
        Err(e) => println!("Error parsing String as i32: {}", e),
    }

    // You can use unwrap_or() to extract Ok value from a Result, or use a fallback value if Err.
    let good_str = String::from("1964");
    println!("Parsed String as i32: {}", good_str.parse::<i32>().unwrap_or(-1));
}