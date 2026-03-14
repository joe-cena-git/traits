fn main() {

    // 10-12 A Summary trait that consists of the behavior provided by a summarize method ----
    {
        pub trait Summary {
            fn summarize(&self) -> String; // Each type implementing this trait must provide its own custom behavior for the body of the method.
        }
    }

    // 10-13 Implementing the Summary trait on the NewsArticle and SocialPost types ----
    {
        pub trait Summary {
            fn summarize(&self) -> String; // Each type implementing this trait must provide its own custom behavior for the body of the method.
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct SocialPost {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub repost: bool,
        }

        impl Summary for SocialPost {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let post = SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            repost: false,
        };

        println!("1 new post: {}", post.summarize());
    }

    // 10-14 Defining a Summary trait with a default implementation of the summarize method ----
    {

    }
}
