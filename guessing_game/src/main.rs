// スライスについて forなど

fn main() {
    let mut _s = String::from("hello world");

    let word = first_word(&_s);
    // ここでsをclearしているが、wordはその前にsを参照だけしているため、後述のprintlnには影響がない。
    // ただし返り値がstringの場合文字自体が変わるためエラーになる。
    // _s.clear();
    println!("{}", word);
    println!("{}", _s);
    // 文字を返す場合はエラーにならない。

    let hello = &_s[..];
    let world = &_s[6..11];
    println!("{},{}", hello, world)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // println!("{:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
