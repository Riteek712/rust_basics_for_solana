fn main() {
    //formated output
    let a = 10;
    let b = 100;
    println!("Hello, world!, {} {}", a, b);

    // unsigned integer
    // u8, u16, u32, u64, u128
    let unsigned: u8 = 100;

    // signed interger 
    // i8. i16, i32, i64, i128 
    let signed: i8 = -100;

    //float is used for decimals
    let float: f32 = 1.0;

    println!("unsign: {} sign: {} float: {}", unsigned, signed, float);

    // char - can only be
    let letter = "c";
    let emoji = "\u{1F600}"; // :D
    let is_true : bool = true;
    println!("letter: {} emoji: {} isTrue: {}", letter, emoji, is_true);

    // arrays
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5] ; // all the elements in the array will be 100.

    println!("index: {}, length: {}",arr[0], other_arr.len());

    // print structure of the array and other objects.
    println!("{:?}", other_arr);
}
