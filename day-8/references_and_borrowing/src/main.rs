/* fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(&m1, &m2);
    let s = format!("{}, {}", m1, m2);
} */

/* fn greet(g1: &String, g2: &String) {
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
 *//*
fn main() {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);
}

fn get_first(vr: &Vec<i32>) -> i32 {
  vr[0]
} */

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("Third element is {}", num);
}
