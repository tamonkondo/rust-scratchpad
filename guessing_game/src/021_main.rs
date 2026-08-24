use guessing_game::{NewsArticle, Summary, Tweet};

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let other_tweet = Tweet {
        username: String::from("other_test"),
        content: String::from("あなたに対しての返信"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.reply(other_tweet));

    let news = NewsArticle {
        headline: String::from("見出し"),
        location: String::from("愛知県"),
        author: String::from("こんどう"),
        content: String::from("test test"),
    };
    println!("{}", news.summarize());
    notify(&tweet);
}
