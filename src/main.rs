use std::fmt::format;

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
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }

        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }

        impl Summary for NewsArticle {} // empty impl, so it will use the default behavior

        let article = NewsArticle {
            headline: String::from("Flyers win the Stanley Cup Championship!"),
            location: String::from("Philadelphia, PA, USA"),
            author: String::from("Philadelphia Inquirer"),
            content: String::from(
                "The Philadelphia Flyers are once again the best \
                hockey team in the NHL."
            ),
        };

        // Prints "New article available! (Read more...)"
        println!("New article available! {}", article.summarize());


    }

    //// Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. ----
    {
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                format!("Read more from {}...", self.summarize_author())
            }
        }

        pub struct SocialPost {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub repost: bool,
        }

        // To use this version of Summary, we only need to define summarize_author when we implement the trait on a type:
        impl Summary for SocialPost {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }

        // After we define summarize_author, we can call summarize on instances of the SocialPost struct,
        // and the default implementation of summarize will call the definition of summarize_author that we’ve provided.
        let post = SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        };

        // Prints: 1 new post: (Read more from @horse_ebooks...)
        println!("1 new post: {}", post.summarize());

        // Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.

        //// Using traits as parameters using "impl Trait" syntax ----
        // We can call notify and pass in any instance of NewsArticle or SocialPost.
        // Code that calls the function with any other type, such as a String or an i32, won’t compile,
        // because those types don’t implement Summary.
        pub fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }

        //// Trait Bound Syntax ----
        // The impl Trait syntax works for straightforward cases
        // but is actually syntax sugar for a longer form known as a trait bound; it looks like this:
        {
            // This longer form is equivalent to the example in the previous section but is more verbose.
            pub fn notify<T: Summary>(item: &T) {
                println!("Breaking news! {}", item.summarize());
            }

            // The impl Trait syntax is convenient and makes for more concise code in simple cases,
            // while the fuller trait bound syntax can express more complexity in other cases.
            // For example, we can have two parameters that implement Summary

            // Doing so with the impl Trait syntax looks like this:
            pub fn notify(item1: &impl Summary, item2: &impl Summary) { }
            // Using impl Trait is appropriate if we want this function to allow
            // item1 and item2 to have different types (as long as both types implement Summary).

            // If we want to force both parameters to have the same type, however,
            // we must use a trait bound, like this:
            pub fn notify<T: Summary>(item1: &T, item2: &T) { }
            // The generic type T specified as the type of the item1 and item2 parameters constrains the function
            // such that the concrete type of the value passed as an argument for item1 and item2 must be the same.
        }

        //// Multiple Trait Bounds with the + syntax ----
        {
            // We can also specify more than one trait bound. Say we wanted notify to use display formatting
            // as well as summarize on item: We specify in the notify definition that item must implement
            // both Display and Summary. We can do so using the + syntax:
            pub fn notify(item: &(impl Summary + Display)) { }

            // The + syntax is also valid with trait bounds on generic types:
            pub fn notify<T: Summary + Display>(item: &T) { }

            // With the two trait bounds specified, the body of notify can call summarize and use {} to format item.
        }

        //// Clearer Trait Bounds with where clauses ----
        {
            // Using too many trait bounds has its downsides.
            // Each generic has its own trait bounds, so functions with multiple generic type parameters can contain
            // lots of trait bound information between the function’s name and its parameter list, making the function
            // signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside
            // a where clause after the function signature.

            // So instead of writing this:
            {
                fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { }
            }

            // we can use a where clause, like this:
            {
                fn some_function<T, U>(t: &T, u: &U) -> i32
                where
                    T: Display + Clone,
                    U: Clone + Debug,
                    {

                    }

                // This function’s signature is less cluttered: The function name, parameter list,
                // and return type are close together, similar to a function without lots of trait bounds.
            }
        }

        //// Returning Types that implement Traits ----
        {
            // We can also use the impl Trait syntax in the return position to return a value
            // of some type that implements a trait, as shown here:
            fn returns_summarizable() -> impl Summary {
                SocialPost {
                    username: String::from("horse_ebooks"),
                    content: String::from(
                        "of course, as you probably already know, people"
                    ),
                    reply: false,
                    repost: false,
                }
            }
            // By using impl Summary for the return type, we specify that the returns_summarizable function returns
            // some type that implements the Summary trait without naming the concrete type. In this case,
            // returns_summarizable returns a SocialPost, but the code calling this function doesn’t need to know that.
        }

        // The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators,
        // which we cover in Chapter 13. Closures and iterators create types that only the compiler knows or types that are very long to specify.
        // The impl Trait syntax lets you concisely specify that a function returns some type that implements the Iterator trait without needing to write out a very long type.

        // However, you can only use impl Trait if you’re returning a single type.
        // For example, this code that returns either a NewsArticle or a SocialPost
        // with the return type specified as impl Summary wouldn’t work:
        {
            fn returns_summarizable(switch: bool) -> impl Summary {
                if switch {
                     NewsArticle {
                        headline: String::from(
                            "Penguins win the Stanley Cup Championship!",
                        ),
                        location: String::from("Pittsburgh, PA, USA"),
                        author: String::from("Iceburgh"),
                        content: String::from(
                            "The Pittsburgh Penguins once again are the best \
                            hockey team in the NHL.",
                        ),
                    }
                } else {
                    SocialPost {
                        username: String::from("horse_ebooks"),
                        content: String::from(
                            "of course, as you probably already know, people",
                        ),
                        reply: false,
                        repost: false,
                    }
                }
            }

            // Returning either a NewsArticle or a SocialPost isn’t allowed due to restrictions
            // around how the impl Trait syntax is implemented in the compiler.
        }

        // 10-15 Conditionally implementing methods on a generic type depending on trait bounds ----
        // Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait
        // that enables comparison and the Display trait that enables printing.
        {
            use std::fmt::Display;

            struct Pair<T> {
                x: T,
                y: T,
            }

            impl<T> Pair<T> {
                fn new(x: T, y: T) -> Self {
                    Self { x, y }
                }
            }

            impl<T: Display + PartialOrd> Pair<T> {
                fn cmp_display(&self) {
                    if self.x >= self.y {
                        println!("The largest member is x = {}", self.x);
                    } else {
                        println!("The largest member is y = {}", self.y);
                    }
                }
            }
        }
    }

}
