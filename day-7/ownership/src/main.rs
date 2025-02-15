// To understand ownership in Rust, we need to understand what makes a rust program safe or unsafe.

// safety is the absence of undefined behavior.

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    read(x); // This causes undefined behaviour as x is not defined in the scope of main function.
    let x = true;
}
