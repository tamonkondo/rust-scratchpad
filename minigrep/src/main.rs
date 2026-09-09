use minigrep::{Config, run};
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    // unwrapは包み込むのを解除。_or_elseはエラーの場合の処理を関数に書く
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("引数解析時の問題{err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        //       "アプリケーションエラー: {e}"
        println!("Application error: {e}");
        process::exit(1);
    }
}
