const _MAX_POINTS: u32 = 100_000;

fn main() {
    // let _x = 5;
    
    // // Rustの変数は全てImmutable
    // // x = 6;

    // let mut y = 5;
    // println!("{}", y);
    // // Mutableのmutを付けることでMutableになる
    
    // y = 6;
    // println!("{}", y);

    // println!("System size: {}", usize::BITS); // System size: 64
    // println!("Memory address of const is: {:p}", &_MAX_POINTS); // Memory address of const is: 0xaaaaabb2d014

    // let integer1: i64 = 1;
    // let integer2: i64 = 2;
    // println!("Stack address of integer1 is: {:p}", &integer1);
    // println!("Stack address of integer2 is: {:p}", &integer2);

    // let y = 5;
    // println!("Stack address of y is: {:p}", &y);

    // let y = y + 1;
    // println!("Stack address of y is: {:p}", &y);

    // let y = y * 2;    
    // println!("Stack address of y is: {:p}", &y);

    // println!("The value of y is: {}", y);
    // {
    //     let y = 0;
    //     println!("The value of y is: {}", y);
    // }
    // println!("The value of y is: {}", y);

    // let touple1 = (500, 6.4, "dummy");
    // let (_x, _y, _z) = touple1;
    // println!("The value of touple1 is: {}, {}, {}", touple1.0, touple1.1, touple1.2);

    // let mut touple2 = ((0, 1), (2, 3));
    // let ((ref mut x1_ptr, ref mut y1_ptr), _) = touple2;

    // *x1_ptr = 5;
    // *y1_ptr = -5;

    // println!("{:?}", touple2);

    let array1 = [1, 2, 3, 4, 5]; // [i32: 5]
    let array2 = [0; 10]; // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    println!("{:?} {:?} {} {}", array1, array2, array1[2], array1[3]);


}
