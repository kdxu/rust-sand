fn main () {
}

fn foo () {
    let v = vec![1, 2, 3]; //vector allocates space for heap
    let v2 = v;
    println!("v[0] : {}", v[0]); // occurs error
}

fn foo2(v: Vec<i32>) -> Vec<i32> {
    v // return ownership
}
