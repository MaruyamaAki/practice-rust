// fn main() {
//     // let string1 = String::from("hello");
//     // let string2 = string1;

//     // println!("{}", string2);

//     // let int1 = 1;
//     // let int2 = int1;
//     // println!("{} {}", int1, int2);

//     // let slice1 = "literal";
//     // let slice2 = slice1;
//     // println!("{} {}", slice1, slice2);

//     // let string3 = String::from("hello");
//     // let string4 = string3.clone();

//     // println!("{} {}", string3, string4);
//     // println!("Stack address of string3 is: {:p}", &string3);
//     // println!("Stack address of string4 is: {:p}", &string4);

//     // println!("Heap address of string3 is : {:?}", string3.as_ptr());
//     // println!("Heap address of string4 is : {:?}", string4.as_ptr());

//     // let string5 = String::from("hello");

//     // println!("Stack address of string5 is: {:p}", &string5);
//     // println!("Heap address of string5 is: {:?}", string5.as_ptr());
//     // println!("Len is: {}", string5.len());
//     // println!("Cap is: {}", string5.capacity());
        
//     // take_ownership(string5);


//     let string6 = String::from("hello");

//     println!("Stack address of string6 is: {:p}", &string6);
//     println!("Heap address of string6 is: {:?}", string6.as_ptr());
        
//     let mut string7 = take_giveback_ownership(string6);

//     println!("Stack address of string7 is: {:p}", &string7);
//     println!("Heap address of string7 is: {:?}", string7.as_ptr());

//     let len = calculate_length(&mut string7);
//     println!("The length of '{}' is {}", string7, len);
// }

// // fn take_ownership(s: String) {
// //     println!("Stack address of s is: {:p}", &s);
// //     println!("Heap address of s is: {:?}", s.as_ptr());
// //     println!("Len is: {}", s.len());
// //     println!("Cap is: {}", s.capacity());
// // }

// fn take_giveback_ownership(s: String) -> String {
//     println!("Stack address of s is: {:p}", &s);
//     println!("Heap address of s is: {:?}", s.as_ptr());
//     s
// }

// fn calculate_length(s: &mut String) -> usize {
//     s.push_str("new world");
//     s.len()
// }

fn main(){

    let string1 = String::from("hello");


    println!("{}", string1);
    println!("{:p}", string1);

}

