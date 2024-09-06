use std::mem::size_of_val;
fn main() {
    println!("Chars Bools and Units start!");

    println!("CBU: 1");
    let a1 = 'a';
    assert_eq!(size_of_val(&a1), 4); //the size of the address is 4
    let a2 = '中';
    assert_eq!(size_of_val(&a2), 4); //the size of the address is 4
 
    println!("CBU: 2");
    let b2 = '中'; //rust analizer made this work with the :char, otherwise would be a &str
    print_char(b2);

    println!("CBU: 3");
    let _c: bool = false;
    let _c2 = true;
    if !_c {
        println!("3: works")
    }

    println!("CBU: 4");
    let d1 = false;
    let d2 = true && false;
    //f != t, change d1 to false
    assert_eq! (d1, d2);

    println!("CBU: 5");
    let _e: () = ();
    let e = (2, 3);
    //implicitly returned () but e != () so I changed it to _e
    assert_eq!(_e, implicitly_ret_unit());

    println!("CBU: 6");
    let f: () = ();
    println!(" size = {}", size_of_val(&f));
    assert!(size_of_val(&f) == 0);
    println!("CBU: END");
}

fn print_char(c : char) {
    println!("{}", c);
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}