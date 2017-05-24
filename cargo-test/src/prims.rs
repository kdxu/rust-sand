fn main() {
 let x: bool = true;
 let x = 'x'; // char
 let two_hearts = '💕'; // rust の場合 1 char = 4 byte

 // slicing
 let a = [0, 1, 2, 3, 4];
 let complete = &a[..];
 let middlte = &a[1..4];

 //tuple
 // str : char slice
 let x: (i32, &str) = (1, "hello");
 let mut x = (1, 2);
 let y = (2, 3);
 // 分解束縛
 let (x, y, z) = (1, 2, 3);
 // (0,) 一要素のタプル
 let y = if x == 5 { 10 } else { 15 };
 loop {
     println!("Loop");
 }
 for (i, j) in (5..10).enumerate() {
     println!("{}, {}", i, j);
 }
}
