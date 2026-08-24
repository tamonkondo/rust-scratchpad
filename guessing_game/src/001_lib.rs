pub trait Summary {
    type Target;
    fn summarize(&self) -> String;
    fn reply(&self, _: Self::Target) -> String {
        String::from("不要です。")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    type Target = NewsArticle;
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    type Target = Tweet;
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content) // これは返しているだけ
    }
    fn reply(&self, other: Tweet) -> String {
        format!(
            "相手から{}に対して返信が来ました。{}:{}",
            self.username, other.username, other.content
        )
    }
}

