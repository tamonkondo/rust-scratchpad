fn main() {
    let data = "initial contents";
    let mut s = String::from(data);
    let x = "Здравствуйте";
    s.push_str("bar");
    // let sx = format!("{s}-{x}");
    let h = x.chars().nth(1).unwrap();
    println!("{}", h)
}
