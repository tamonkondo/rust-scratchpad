fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    change(&mut s1);
    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);

    let mut two_s = String::from("hello");
    // これは駄目。二回可変参照は禁止されている。
    // let r1 = &mut two_s;
    // let r2 = &mut two_s;
    // これはOK
    let r1 = &two_s;
    let r2 = &two_s;
    // これはNG, r1,r2で不変で参照がされているため可変で定義ができない。
    // let r3 = &mut two_s;
    // println!("{}, {}, {}", r1, r2, r3)
    let r3 = &two_s;
    println!("{}, {}, {}", r1, r2, r3);

    let reference_to_nothing = dangle();
    print!("{reference_to_nothing}");
}

// これはNG。参照用の値を返してもdangleが完了したらドロップして参照元のsが消えてしまうためNG
// fn dangle()-> &String {
//     let s = String::from("hello");
//     &s
// }

// フツーに値を返せば良い。
fn dangle() -> String {
    String::from("hello")
}

// 引数で受け取った可変参照ができる値は、渡される値自体も定義されている時に参照可能と宣言をしないといけない。
fn change(some_string: &mut String) {
    some_string.push_str(",world");
    println!("{}", some_string)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
