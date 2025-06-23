use ndarray::prelude::*;
use rand::Rng;

pub fn do_demo() {
    create_and_access_ndarray();
    process_ndarray();
}

fn create_and_access_ndarray() {
    
    println!("\ncreate_and_access_ndarray");
    println!("--------------------------------------");

    let a = array!(0, 10, 20, 30, 40, 50, 60, 70, 80);

    println!("{:?}", a.slice(s![3..]));      // [30, 40, 50, 60, 70, 80]
    println!("{:?}", a.slice(s![..3]));      // [0, 10, 20]
    println!("{:?}", a.slice(s![3..7]));     // [30, 40, 50, 60]
    println!("{:?}", a.slice(s![3..7;2]));   // [30, 50]
    println!("{:?}", a.slice(s![3..;2]));    // [30, 50, 70]
    println!("{:?}", a.slice(s![..;2]));     // [0, 20, 40, 60, 80]
}

fn process_ndarray() {

    println!("\nprocess_ndarray");
    println!("--------------------------------------");

    let mut rng = rand::rng();
    let data: Array1<i32> = Array1::from((0..1_000_000).map(
        |_| rng.random_range(1..10)).collect::<Array1<i32>>()
    );

    let cubes = data.mapv(|x| x.pow(3));
    println!("Calculated the cubes of {} elements", cubes.len());
}