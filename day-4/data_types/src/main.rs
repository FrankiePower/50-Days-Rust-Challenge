struct Person {
    name: String,
    age: u8,
    is_alive: bool,
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    //DATA TYPES
    //Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.
    //Scalar Types
    //A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    //Integers
    //An integer is a number without a fractional component. Rust has signed and unsigned integers. Signed integers can be positive, negative, or zero. Unsigned integers can only be positive or zero.
    //Signed integers are i8, i16, i32, i64, i128, and isize. The number after i indicates the number of bits the integer uses. Unsigned integers are u8, u16, u32, u64, u128, and usize. The number after u indicates the number of bits the integer uses.

    let small_number: u8 = 255; //Unsigned 8-bit integer
    let big_number: u128 = 12345678901234567890; //Unsigned 128-bit integer
    let small_number2: i8 = -128; //Signed 8-bit integer
    let big_number2: i128 = -12345678901234567890; //Signed 128-bit integer

    println!("small_number: {}", small_number);
    println!("big_number: {}", big_number);
    println!("small_number2: {}", small_number2);
    println!("big_number2: {}", big_number2);

    //Integer literals can be expressed in decimal, hexadecimal, octal, and binary. Decimal is the default. To specify a number in hexadecimal, precede it with 0x. To specify a number in octal, precede it with 0o. To specify a number in binary, precede it with 0b.

    let decimal = 98_222; //Decimal
    let hex = 0xff; //Hexadecimal
    let octal = 0o77; //Octal
    let binary = 0b1111_0000; //Binary
    let byte = b'A'; //Byte (u8 only)

    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);
    println!("binary: {}", binary);
    println!("byte: {}", byte);

    //Floating-Point Numbers
    let x = 2.0; //f64 default
    let y: f32 = 3.0; //f32

    println!("x: {}, y = {} ", x, y);

    //Numeric Operations
    //Rust supports the basic arithmetic operations: addition, subtraction, multiplication, division, and remainder. The remainder operation can only be used with integers.
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;

    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}",
        sum, difference, product, quotient
    );

    //Boolean Type
    //A Boolean type represents a logical value. It can be either true or false.
    let t = false; //implicit declaration
    let f: bool = false; //explicit declaration

    println!("t: {}, f: {}", t, f);

    // if
    if t {
        println!("t is true");
    } else {
        println!("t is false");
    }

    let not_t = !t;

    println!("not_t: {}", not_t);

    let b: bool = 1 > 2; //false

    println!("b: {}", b);

    //Character Type
    //The character type represents a single Unicode character. Rust's char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Rust's char type is specified with single quotes.

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!(" {},  {},  {}", c, z, heart_eyed_cat);

    // iterate over characters in a string
    for char in "Ciao, #mondo!".chars() {
        println!("{}", char);
    }

    //Tuples
    //A tuple is a collection of values of different types. Tuples have a fixed length. Once declared, they cannot grow or shrink in size. Tuples are useful when you want to return multiple values from a function. Tuples are declared using parentheses and commas to separate the types.

    let tup: (i32, f64, char) = (500, 6.4, 'x');

    println!("tup: {:?}", tup);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let x_char = tup.2;

    println!("The value of x_char is: {}", x_char);
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);

    // Arrays
    let arr = [1, 2, 3, 4, 5];

    for element in arr.iter() {
        println!("{}", element);
    }

    // Custom Data types

    //Structs
    //A struct is a custom data type that lets you name and package together multiple related values that make up a meaningful group. Structs are similar to tuples, but with a key difference: the values in a struct are named, while the values in a tuple are unnamed. Structs are declared using the struct keyword.

    let person = Person {
        name: String::from("Alice"),
        age: 30,
        is_alive: true,
    };

    println!(
        "Person name is {} and age is {} and is alive {}",
        person.name, person.age, person.is_alive
    );

    // Enums
    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go!"),
    }
}
