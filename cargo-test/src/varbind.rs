fn main() {
    //let x = 5;
    //let (x, y) = (1, 2); // pattern match
    //let x: i32 = 5; // type annotations
    // bindings are defalt immutable.
    //let mut x = 5;
    //x = 10; // mut directive allows the value mutable.
    /*
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x  is {} and value of y is {}", x, y);
    }
    */
    // shadowing
    let x: i32 = 8;
    {
        println!("{}",x); // 8
        let x = 12;
        println!("{}", x); // 12
    }
    println!("{}",x); //8
    let x = 42;
    println!("{}", x); //42

}
