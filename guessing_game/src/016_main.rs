use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();
    // scores.insert("Blue", 10);
    // scores.insert("Yellow", 50);

    // let team_name = "Blue";
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    // println!("{}",score);
    // let mut a =  [1, 2];
    // let b = &mut a; // ここでaは借りられているので、a自体を変更することはできない。
    // // a[0] = 1;
    // b[0] = 1;
    // println!("{:p}", &b);
    // let mut a: [i32; 2] = [1, 2];
    // a[0] = 6;
    // let mut b = a;
    // b[0] = b[0] + 4;
    // // これは同じ
    // println!("{:?}", &a);
    // println!("{:?}", &b);

    let mut a = 1;
    a = 2;
    println!("{:?}", a);
}
