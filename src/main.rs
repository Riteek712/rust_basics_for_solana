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

    // tuple : stores unique elements
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3,5);

    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);
    let (a, b, c) = tuple;

    // destructuring
    println!("first: {}, second: {}, third: {}", a, b, c);

    // Creating and calling a function.
    println!("{}", is_even(11));

    //  mutability in variables
    let mut num = 5; // need to use the keyword "mut"
    num = 3;
    println!("{}", num);

    //Array and slices 
    let arr1 = [0, 1, 2, 3];
    let slice =  &arr1[1..3];
    borrowing_slice(arr1, slice);

    // strings
    let str: &str = "hello world";
    let mut string: String = String::from("namaste world"); // Now this is equivalent to a vector, dynamic size array where we can append or remove the contents.

    let slice2 = &string[.. 6];
    println!("slice2: {:?}", slice2);

    string.push('1');
    string.push_str("! Bob");
    string = string.replace("nam", "xxx");
    println!("{}", string);


    // Conditional statements
    let n =3 ;
    if n > 0 {
        println!("grater than 0")
    }else if n < 0 {
        println!("less than 0")
    }else{
        println!("equal to 0")
    }

}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // by not giving ; we are returning the output of {digit == 0}.
}

fn borrowing_slice(arr: [u8;4], slice: &[u8]){
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}
