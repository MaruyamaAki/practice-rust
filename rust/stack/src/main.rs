fn main() {
    // 32bit signed intが型推論でつけられる
    let a = 10;
    let b = 20;

    println!("{} {}", a, b);


    // 型推論だと倍精度？
    let x = 2.0; // f64(double precision)
    let y: f32 = 3.0; // f32(single precision)

    println!("{} {}", x, y);


    // a, bはスコープを抜けるとメモリ解放（Drop）される
    // Last In Farst Out
}
