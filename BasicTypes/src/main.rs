//for I10
use std::ops::{Range, RangeInclusive};

fn main() {
    println!("Start!");
    
    let a = 10;
    println!("I1: a = {}", a);

    let b: u16 = 38_u8 as u16;
    //Casting looks to be as simple as saying "as (stuff)"
    println!("I2: b = {}", b);

    //added the ": u32" because without that it was i32
    //tried setting c to 4,294,967,295 because its the largest u32
    //did not work bc it was not in range for i32
    let c : u32 = 5;
    //changed bc we needed assert to have 2 of the same types
    assert_eq!("u32".to_string(), type_of(&c));
    println!("I3: c is a u32 c = {}", c);

    //127 is the largest i8, -128 is the min
    //see https://doc.rust-lang.org/std/primitive.i8.html
    assert_eq!(i8::MAX, 127);
    //255 is the largest u8
    assert_eq!(u8::MAX, 255);
    println!("I4: i8::MAX == 127, u8::MAX == 255");

    //was 251_u8, 255 is the max, changed to i16
    let d = 251_i16 + 8;
    //was i8, this is too small for the add
    let e = i16::checked_add(251, 8).unwrap();
    println!("I5: v1 = {}, v2 = {}", d, e);
    
    //0x is hex, 0o is octal, 0b is binary, b'A' is u8 byte
    let i = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(i == 1597); //the fix was 1579 to 1597
    println!("I6: i = {} and is type {}", i, type_of(&i));

    //the deugger gave me this one,
    //see https://doc.rust-lang.org/std/index.html for the limits
    let f = 1_000.000_1; //f64
    let g: f32 = 0.12; //f32
    let h = 0.01_f64; //f64

    assert_eq!(type_of(&f), "f64".to_string());
    println!("I7: f = {}, g = {}, h = {}", f, g, h);
    
    //float addition is the problem
    //assert!(0.1 + 0.2 == 0.3);

    println!("I8: 0.1 + 0.2 = {}", 0.1 + 0.2);

    let mut sum = 0;
    for i in -3..2 {
        sum += 1
    }
    //println!("sum = {}", sum);
    //was == -3 bc -3..2 is 5
    assert!(sum == 5);

    print!("'a' to 'z': ");
    for ii in 'a'..='z' {
        //casting works by adding as TYPE
        print!(" {}", ii as i32);
    }
    println!("");
    println!("I9: ranges and casting");

    //for inclusive ise =5 for range
    assert_eq!((1..5), Range{ start: 1, end: 5});
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("I10: cool range stuff");

    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);

    // -1 does not exist in u8
    // you need to change either the u8
    assert!(1i8 - 2 == -1); 
    
    assert!(3 * 50 == 150);

    let j : f32 = 9.6 / 3.2;
    println!("9.6 / 3.2 = {}", 9.6 / 3.2);
    assert!(j.ceil() == 3.0); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    println!("I11: Done");
}

//this funtion takes a variable, and returns the type of it in string
fn type_of<T>(_: &T) -> String {
    //this somehow returns (impliced return)
    //I assume that std::any::type_name::<T> returns the type of T
    format!("{}", std::any::type_name::<T>())
}