fn main() {
    let string1 = "string literal"; // &str　13byte
    let string2 = "hello"; // &str 5byte

    println!("Stack address of string1 is: {:p}", &string1);
    println!("Stack address of string2 is: {:p}", &string2);

    println!("Static memory address of string1: {:?}", string1.as_ptr());
    println!("Len of string1: {}", string1.len());
}
