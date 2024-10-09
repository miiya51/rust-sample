use std::io;

fn main() {
    // let x = 2.0; // f64

    // let y: f32 = 3.0; // f32

    // // addition
    // // 足し算
    // let sum = 5 + 10;

    // // subtraction
    // // 引き算
    // let difference = 95.5 - 4.3;

    // // multiplication
    // // 掛け算
    // let product = 4 * 30;

    // // division
    // // 割り算
    // let quotient = 56.7 / 32.2;
    // let floored = 2 / 3; // Results in 0
    //                      // 結果は0

    // // remainder
    // // 余り
    // let remainder = 43 % 5;

    // let t = true;

    // let f: bool = false; // with explicit type annotation
    //                      // 明示的型注釈付きで

    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻'; 

    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of y is: {}", y);

    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    // println!("The value of six_point_four is {}", six_point_four)

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index."); // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line"); // 値の読み込みに失敗しました

    let index: usize = index.trim().parse().expect("Index entered was not a number"); // 入力された値は数字ではありません

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index, element
    );


}
