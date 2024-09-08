fn main() {
    println!("Statements and expressions");
    let a = 5u32;
    let b = {
        let a_squared = a * a;
        let a_cube = a_squared * a;

        a_cube + a_squared + a
    };

    //needed to remove the ';' after '2 * a'
    //if the last line is of the return type putting no ; will make it ret
    let c = {
        2 * a
    };
    println!("SE: 0 a = {}, b = {}, c = {}", a, b, c);
    
    let d = {
        let mut x = 1;
        x + 2
        //it being += was wrong because that would return a unit aka ()
    };
    assert_eq!(d, 3);
    println!("SE: 1, d = {}", d);
    
    //I have no idea what it was looking to do with let v = (let x = 3);
    let e =  3;
    assert!(e == 3);
    println!("SE: 2, e = {}", e);
    
    let f = sum(1, 2);
    assert_eq!(f, 3);
    println!("SE: 3, f = {}", f);
}

fn sum(x: i32, y: i32) -> i32{
    x + y
}
