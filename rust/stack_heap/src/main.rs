fn main() {
    // let _array1: [u8; 7000000] = [1; 7000000];
    // let _array1: [u8; 9000000] = [1; 9000000];

    let mut vector1 = vec![1, 2, 3, 4];
    let mut vector2 = vec![9, 10];

    println!("Stack address of vector1 is: {:p}", &vector1);
    // println!("Stack address of vector2 is: {:p}", &vector2);

    println!("Heap memory address of vector1: {:?}", vector1.as_ptr());
    println!("Len of vector1 is: {}", vector1.len());
    println!("Capacity of vector1 is : {}", vector1.capacity());
    // Heap memory address of vector1: 0xaaaaf9386ba0
    // Len of vector1 is: 4
    // Capacity of vector1 is : 4

    vector1.insert(1, 10);
    println!("{:?}", vector1);
    // [1, 10, 2, 3, 4]

    vector1.remove(0);
    println!("{:?}", vector1);
    // [10, 2, 3, 4]

    vector1.append(&mut vector2);
    println!("{:?}", vector1);
    println!("{:?}", vector2);
    // [10, 2, 3, 4, 9, 10]
    // []
}
