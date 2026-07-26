fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    let x = "test";
    // Rustのprintln!の第一引数はフォーマットのため、変数をそのまま渡せない。第二引数から空の{}中に1つずつ配置される。
    println!("xがでるはず：{x}、sがでるはず：{}", s);
    // println!(s);

    // &str型は文字を変更できない。
    // let mut test_str = "hello";
    // test_str.push_str(",world");

    // Copyトレイトを実装されているもの。
    // あらゆる整数型。u32など。
    // 論理値型であるbool。trueとfalseという値がある。
    // あらゆる浮動小数点型、f64など。
    // 文字型であるchar。
    // タプル。ただ、Copyの型だけを含む場合。例えば、(i32, i32)はCopyだが、 (i32, String)は違う。
    // なので数値はcloneなどしなくていい。
    let x = 5;
    let y = x;
    let xy = x * y;
    println!("{xy}");

    let s1 = String::from("hello");

    // let mut s2 = &s1; 参照だけではpush_strは使えない。あくまで参照だけなので
    // 参照元をいじりたい場合はString::fromで囲む必要がある。
    // let mut s2 = String::from(&s1); これでも出来るが結局同じものを参照するならcloneが手っ取り早い。
    // cloneでもいけるので、cloneのが短くて良い。
    let mut s2 = s1.clone();
    s2.push_str("test");
    println!("{},world", s1);
    println!("{},world", s2);

    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(&s); // sの値が関数にムーブされ...
    // ... ここではもう有効ではない
    // これはString::from自体が固定値を扱うものではないので1回きりの参照になってしまう。
    // 但し引数の方をStringから参照できる&Stringに変えれば値が不変でも問題なし
    // cloneでも渡せるが、sを渡した後にs.clone()はできない。
    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
    // i32はCopyなので、この後にxを使っても
    makes_copy(x); // xも関数にムーブされるが、
    // 大丈夫
}

// &strは固定値、String::fromは提示にヒープを確保する。そのためユーザーからの入力値や値を検証する時に受け取る変数値を入れる時はStringを活用する。
fn takes_ownership(some_string: &String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
// 後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
