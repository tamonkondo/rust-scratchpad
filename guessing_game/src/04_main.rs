#[derive(Debug)]

enum ArrStr {
    Text(&'static str),
    Number(u32),
}
fn main() {
    // タプル型 jsなどと違って丸括弧
    let x = (500, 200, 1.3);
    let _x_0 = x.0;
    let _x_1 = x.1;
    let _x_2 = x.2;

    let add = x.0 * x.1;
    println!("{add}です");
    // Rustでは配列内に複数の定義は禁止。使うにはenumでやるか、str型などにまとめる。
    let arr_str: [ArrStr; 4] = [
        ArrStr::Text("test"),
        ArrStr::Number(2),
        ArrStr::Text("array"),
        ArrStr::Number(4),
    ];
    let first_item = &arr_str[0];
    // 単一の反映 人間側は[0]を視覚的に値が分かるから、Text型とわかるが、rustは実行するまでわからないのでmatchが必要
    match first_item {
        ArrStr::Text(s) => println!("{s}"),
        ArrStr::Number(s) => println!("{s}"),
    }

    // arr_strをforやloopで出したい。
    // 番号の取り出しは.iter().enumerate()が必要
    for (index, x) in arr_str.iter().enumerate().rev() {
        let index = index + 1;
        match x {
            ArrStr::Text(s) => println!("{s},{index}番目"),
            ArrStr::Number(s) => println!("{s},{index}番目"),
        }
    }
    let x = plus_one(2);
    println!("xは {x}")
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
