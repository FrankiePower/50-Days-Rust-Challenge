/* fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(&m1, &m2);
    let s = format!("{}, {}", m1, m2);
} */

fn greet(g1: &String, g2: &String) {
    println!("{}, {}", g1, g2);
}

fn main() {
    let mut x: Box<i32> = Box::new(1);

    let a: i32 = *x;

    *x += 1;

    let r1: &Box<i32> = &x;

    let b: i32 = *s * r1;

    let r2: &i32 = &*x;
    let c: i32 = *r2;

    println!("a: {}, b: {}, c: {}", a, b, c);
}
