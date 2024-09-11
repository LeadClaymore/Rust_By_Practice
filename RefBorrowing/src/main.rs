fn main() {
    println!("Reference and borrowing");
    let a = 5;
    // this puts the 
    let b = &a;
    println!("RB 1: the memory address of a is {:p}", b); 
    // One possible output: 0x16fa3ac84

    let c = 5;
    let d = &c;
    assert_eq!(5, *d);
    println!("RB 2: c = {}, d = {}", c, d);

    let mut e = String::from("hello, ");
    // needed the ampersand because it needed the refrence to work
    borrow_object(&e);
    println!("RB 3: e = {}", e);

    let mut f = String::from("hello, ");
    // needed mut cause f is a mut
    // needed & bc it is changing f, and the changes need to hop scope
    push_str(&mut f);
    println!("RB 4: f = {}", f);

    let mut g = String::from("Hello, ");
    let h = &mut g;
    h.push_str("World");
    println!("RB 5: h = {}", h);

    let i = '中';
    let i1 = &i;
    // need ref so that i2 is borrowed
    let ref i2 = i;
    assert_eq!(*i1, *i2);
    assert_eq!(get_addr(i1), get_addr(i2));
    println!("RB 6: i = {}, i1 = {:p}, i2 = {:p}", i, i1, i2);

    println!("RB 7: ");

    println!("RB 8: ");

    println!("RB 9: ");

    println!("RB 10: ");

    println!("RB 11: ");
}

fn borrow_object(s: &String) {}

fn push_str(s: &mut String) {
    s.push_str("world")
}

// get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}