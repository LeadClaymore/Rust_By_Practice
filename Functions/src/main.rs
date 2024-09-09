fn main() {
    println!("Functions Start");
    let (a, b) = (1, 2);
    let c = sum (a, b);
    assert_eq!(c, 3);
    println!("F: 1, a = {}, b = {}, c = {}", a, b, c);

    print();
    println!("F: 2");

    //never_return();
    println!("Failed!");
    println!("F: 3");

    println!("F: 4, IDK");

    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };
    println!("F: 5");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn print() -> () {
    println!("Print Done!");
}

fn never_return() -> ! {
    //the solution was to call panic or inf loop
    panic!("I return nothing!")
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            1
        }
        _ => {
            0
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    todo!()
}