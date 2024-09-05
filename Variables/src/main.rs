fn main() {
    println!("Start!");
    let x: i32 = 2; //changed by addind "= 2"
    //let y: i32; //not an error, but a warning
    
    //assert_eq!(x, 5); //caused error, IDK
    
    let mut z = 1;
    z += 2;
    //z++; //not valid

    println!("using mut z = {}", z);

    //scope exists
    {
        let a = 1;
        println!("A exists in this scope. a = {}", a);
    }

    //shadowing
    {
        //within scope you can declare a var with the same name, this is called shadowing
        let x: i32 = 5;
        println!("in this scope x = {}", x);
    }
    println!("outside of that scope x = {}", x);

    let (a, b) = (1, 2);
    //b += 1; //needs to be (a, mut b) to work
    //(also mut (a, b) does not work)

    println!("Cool way to create vars, a = {}, b = {}", a, b);
    
    //bro idk what this is, tuple, slice, struct pattern?
    //tuple is the ..
    //slice idk (its refenced later so idk)
    //I think struct pattern is []
    //I think that the .. means ignore assignment
    //usefull if data allways has x size and receprecating has variable size under x
    //also stuff needs to be either both [] or both ()
    let (c, d);
    (c,..) = (3, 4);
    [.., d] = [1, 2];

    assert_eq!([c, d], [3, 2]);
    println!("Cool asignemtnt with tuples c = {}, d = {}", c, d);

    println!("End!");
}
