use std::io;
// ioはinput output

// mutは変更可能
// &は参照
// read_lineに&mutと渡すとguessの参照と変更をできる。mutだけだとread_lineにguessの所有権を渡すためread_lineの関数が完了したらguessは不要になるため消えてしまう。

fn main() {
    println!("Guess the Number!");

    println!("Please input your guess");
    // mutはmutableで可変 つまりjsと違い、letにmutを使わないと代入できないんだ。
    let mut guess: String = String::new(); // 空の文字列を生成
    io::stdin() // standard input? 入力窓口 ターミナルの入力
        .read_line(&mut guess) // 入力した内容を定義した変数に代入できる
        .expect("Failed to read line"); // 失敗時のエラー

    println!("You guessed: {guess}");
}
