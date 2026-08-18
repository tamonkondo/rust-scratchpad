fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        _ => println!("Not"),
    }
    
    // if let Some(third) = third {
    //     println!("The third element is {third}")
    // }

    // メモリが新たに割り当てが起きる可能性があるため、定義後にpushしてからprintlnはエラーになる。
    // let first = &v[0];
    // v.push(5);
    // println!("The first element is: {first}");
}
