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

    println!("System size: {}", usize::BITS); // System size: 64
    println!("Memory address of const is: {:p}", &_MAX_POINTS); // Memory address of const is: 0xaaaaabb2d014

    let integer1: i64 = 1;
    let integer2: i64 = 2;
    println!("Stack address of integer1 is: {:p}", &integer1);
    println!("Stack address of integer2 is: {:p}", &integer2);
}
