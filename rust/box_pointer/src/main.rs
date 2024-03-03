enum List {
    Node(i32, Box<List>),
    Nil
}

fn main() {
    // let tuple1: (i64, String) = (10, String::from("hello"));

    // println!("Stack address of tuple data: {:p}", &tuple1);

    // println!("Heap memory address of tuple1.1: {:?}", tuple1.1.as_ptr());
    // println!("Len of t1.1 is: {}", tuple1.1.len());
    // println!("Capacity of tuple1.1 is: {}", tuple1.1.capacity());


    // let mut box1 = Box::new(tuple1);

    // (*box1).1 += "world";
    // println!("{} {}", box1.0, box1.1);

    // println!("Stack address of box pointer: {:p}", &box1);
    // println!("Heap address of tuple data is: {:p}", box1);
}
