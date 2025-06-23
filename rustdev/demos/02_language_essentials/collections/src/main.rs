use std::collections::HashMap;

fn main() {
    demo_vec();
    demo_vec_bounds_checking(0);
    demo_hash_map();
}

fn demo_vec() {

    println!("Using a vector");
    
    let mut v1: Vec<i32> = Vec::new();
	
    // Note, could rewrite the above as:
    let mut _v1b = Vec::<i32>::new();
	
    v1.push(100);
    v1.push(200);
    v1.push(300);
    println!("v1 is {:?}, length is {}, first element is {}", v1, v1.len(), v1[0]);

    let mut v2 = vec!["Huey", "Louis", "Dewey"];
    v2.insert(0, "Donald");
    println!("{:?}", v2);

    for item in v2 {
        println!("{}", item)
    }
}

fn demo_vec_bounds_checking(index: usize) {
    
    println!("\nVector bounds-checking");
    
    let v = vec![100, 101, 102];

    // This could cause the program to panic (i.e. crash).
    // let item = v[index];

    // Safer to call get(), which returns an Option<T>.
    // Option is an enum that either contains Some<T> or None.
    let opt = v.get(index);
    match opt {
        Some(value) => println!("At index {}, found value {}", index, value),
        None        => println!("At index {}, no value found", index)
    }

    println!("If you KNOW an Option isn't None, you can unwrap its value. Let's try... {}", opt.unwrap())
}

fn demo_hash_map() {

    println!("\nUsing a HashMap");
    
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert(String::from("UK"), 44);
    m.insert(String::from("NO"), 47);
    m.insert(String::from("SG"), 65);
    println!("m is {:?}, length is {}", m, m.len());

    println!("If you only want to insert if doesn't already exist...");
    let val = m.entry(String::from("UK")).or_insert(44);
    println!("Value for UK is {}", val);

    // This would cause the program to panic (i.e. crash).
    // let val = m["SA"];
    // println!("Value for SA is {}", val);

    // Better to call get(), which returns an Option<T>.
    // Option is an enum that either contains Some<T> or None.
    let opt = m.get("SA");
    match opt {
        Some(val) => println!("Value for SA is {}", val),
        None      => println!("No value found for SA")
    }
}
