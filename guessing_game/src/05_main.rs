fn main() {
    let number: i32 = 3;
    let check_number = number == 3;
    // JSと違って数値と真偽値を判別するため比較が必要
    if check_number {
        println!("3だよ");
    } else {
        println!("3じゃない");
    }

    // if文の返り値がそれぞれ違う型の場合、enumで定義をするか、
    // シンプルにif文内で出力を変えるかなどをしないとできない。
    let condition = true;
    enum Value {
        Number(i32),
        Text(&'static str),
    }
    let number = if condition {
        Value::Number(5)
    } else {
        Value::Text("six")
    };
    match number {
        Value::Number(n) => println!("The value of number is: {n}"),
        Value::Text(s) => println!("The value of number is: {s}"),
    }

    let mut counter = 0;
    // breakでloopの処理が終わる。continueは次の処理にいかず、1つ上の階層の初めに戻る。
    let result = loop {
        counter += 1;
        println!("今は{counter}番目です。");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
