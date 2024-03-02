fn main() {
    // let string1 = "string literal"; // &str　13byte
    // let string2 = "hello"; // &str 5byte

    // println!("Stack address of string1 is: {:p}", &string1);
    // println!("Stack address of string2 is: {:p}", &string2);

    // println!("Static memory address of string1: {:?}", string1.as_ptr());
    // println!("Len of string1: {}", string1.len());

    // let mut string1 = String::from("hello");
    // let mut string2 = String::from("helloWorld");

    let mut string1 = String::from("hello");

    println!("Stack address of string1 is: {:p}", &string1);
    println!("Static memory address of string1: {:?}", string1.as_ptr());
    println!("Len of string1: {}", string1.len());
    println!("Capacity of string1 is: {}", string1.capacity());

    string1.push_str("new_value");

    println!("Stack address of string1 is: {:p}", &string1);
    println!("Static memory address of string1: {:?}", string1.as_ptr());
    println!("Len of string1: {}", string1.len());
    println!("Capacity of string1 is: {}", string1.capacity());
}
