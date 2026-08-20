use std::fs::File;
use std::io::{Error, Read};

fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
fn main() {
    // 行数が呼ばれるのでエラー元が分かる。
    // panic!("crash and burn")
    // let v = vec![1,2,3];

    // v[99];
    // let error_message = greeting_file_result
    //     .as_ref()
    //     .err()
    //     .map(|e| e.to_string())
    //     .unwrap_or_default();

    // let greeting_file_result = File::open("hello.txt");
    // エラーの内容によって条件分岐ができる。
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("ファイルを作成するのに問題がありました。{:?}", e),
    //         },
    //         other_error => {
    //             //     "ファイルを開くのに問題がありました: {:?}"
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };
    // matchの省略版
    // let greeting_file = greeting_file_result.unwrap();
    // let greeting_file = greeting_file_result.expect("hello.txtは含まれるべき");

    let result = read_username_from_file();
    println!("{:?}", result);
}
