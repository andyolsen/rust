use implementing_functionality::employee::Employee;
use implementing_functionality::hilo1;
use implementing_functionality::hilo2;

fn main() {
    do_emp();
    do_hilogame1();
    do_hilogame2();
}

fn do_emp() {

    let emp1 = Employee::new(String::from("Andy"), 1000, false);
    // emp1.payrise(100); // Nope, because emp1 is immutable.
    emp1.print();

    let mut emp2 = Employee::new(String::from("Jayne"), 2000, false);
    emp2.payrise(100);
    emp2.print();
}

fn do_hilogame1() {
    hilo1::hi_lo_game(100)
}

fn do_hilogame2() {
    let mut game = hilo2::HiLoGame::new(100);   
    while game.play() {}
    print!("Goodbye!")
}