fn main() {
    println!("Ownership start");
    let a = String::from("Hello world");
    let b = a.clone(); 
    //adding .clone() fixes it by cloneing instead of moving like normal
    println!("{}, {}", a, b);
    println!("O: 1");

    let c1 = String::from("Hello world");
    let c2 = take_ownership(c1);
    println!("{}", c2);
    println!("O: 2");

    let d = give_ownership();
    println!("{}", d);
    println!("O: 3");

    //by adding the clone there the clone is malloced in the print_str function not e
    let e = String::from("Hello World");
    print_str(e.clone());
    println!("{}", e);
    println!("O: 4");

    //I removed a string from here, idk how to copy most of an tuple.
    //the chalenge was to copy not clone
    let f = (1, 2, ());
    let g = f;
    println!("{:?}, {:?}", f, g);
    println!("O: 5");

    let h = String::from("Hello ");
    let mut h1 = h;
    h1.push_str("World!");
    println!("O: 6, h1 = {}", h1);

    let i = Box::new(5);
    // added "mut y: Box<u32> = Box::new(42)" 
    let mut j: Box<u32> = Box::new(42);
    *j = 4;
    assert_eq!(*i, 5);
    println!("O: 7");

    let k = (String::from("hello"), String::from("world"));
    let _l = k.0;
    // Modify this line only, don't use `_l`
    // the solution was to ignore k.0, and just use k.1
    // the takeaway was that only k.0 is moved not k.1
    println!("O: 8 {:?}", k.1);

    let m = (String::from("hello"), String::from("world"));
    // Clone was needed so its ownership is not changed
    let (n1, n2) = m.clone();
    println!("O: 9 {:?}, {:?}, {:?}", n1, n2, m); // -> "hello", "world", ("hello", "world")
}

//by returning the string, we return the ownership to whatever its returned to
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

// By adding .clone() we dont move ownership
// but rather make an copy and give that ownership
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.clone().into_bytes();
    s
}

fn print_str(s: String)  {
    println!("{}",s)
}