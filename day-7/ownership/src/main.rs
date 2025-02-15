// To understand ownership in Rust, we need to understand what makes a rust program safe or unsafe.

// safety is the absence of undefined behavior.

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

// memory is the space where data is stored during the execution of a program.
fn main() {
    /*   read(x); // This causes undefined behaviour as x is not defined in the scope of main function.
    let x = true; */
    /*
    let n = 5;
    let y = plus_one(n);
    println!("The value of y is: {y}");

    let a_num = 4;
    make_and_drop(); */

    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn make_and_drop() {
    let a_box = Box::new(5);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn move_a_box(b: Box<i32>) {
    let b = Box::new(0);
    let b2 = b;
    move_a_box(b);
}
