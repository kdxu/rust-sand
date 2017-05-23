fn main() {
    print_sum(5, 6);
}

fn foo() {
}

fn print_number(x : i32){
    println!("x is {}", x);
}

fn print_sum(x: i32, y: i32){
    println!("sum is {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges () -> !{
    panic!("this func never returns"); // crushs current thread.
}
// RUST_BACKTRACE=1 で backtrace を得られる

//let x: i32 = diverges(); // 発散関数は任意の型の super set となる

let f: fn(i32) -> i32 = add_one; // i32 -> i32 の関数を受け取る変数束縛として作用する
let six = f(5);

