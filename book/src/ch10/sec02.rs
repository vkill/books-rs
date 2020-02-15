//! [Traits]: Defining Shared Behavior
//!
//! [traits]: https://doc.rust-lang.org/book/ch10-02-traits.html
//!
//! # Examples
//!
//! ```
//! use the_book::ch10::{Article, Tweet, Summary};
//!
//! let article = Article {
//!     headline: String::from("Headline!"),
//!     content: String::from("Article"),
//! };
//! let tweet = Tweet {
//!     username: String::from("Sam I am"),
//!     content: String::from("tweet, tweet, tweet!"),
//! };
//!
//! assert_eq!(String::from("(Read more...)"), article.summarize());
//! assert_eq!(String::from("tweet, tweet, tweet! @Sam I am"), tweet.summarize());
//! ```

/// Default trait implementation.
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Article {
    pub headline: String,
    pub content: String,
}

/// `Article` uses the default `summarize` method.
impl Summary for Article {}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} @{}", self.content, self.username)
    }
}
