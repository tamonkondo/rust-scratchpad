fn main() {
    let not_mut_x = 5; // これは再代入できない。
    println!("出力not_mut_x： {not_mut_x}");
    let not_mut_x = not_mut_x * 5;
    println!("掛け算not_mut_x： {not_mut_x}");

    let mut mut_x = 10; // これは再代入可能
    println!("出力mut_x： {mut_x}");
    mut_x = 1;
    println!("代入後出力mut_x： {mut_x}");
    let str_num: u32 = "25".parse().expect("test"); // 文字列もそのまま変更できる
    mut_x = str_num;
    println!("文字列変換後代入後出力mut_x： {mut_x}");
}
