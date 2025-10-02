fn default_implementation() {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

fn traits_as_parameters() {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    
    pub fn notify<T: Summary>(item1: T, item2: T) {
        println!("Breaking news! {}", item.summarize());
    }


    // pub fn notify(item: impl Summary + Display) {};  // syntax sugar
    // pub fn notify<T: Summary + Display>(item: T) {};  // same as above

    //  fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {} // multiple trait bounds
    // fn some_function<T, U>(t: T, u: U) -> i32 // where T: Display + Clone, U: Clone + Debug {} // same as above with where clause
    //     where T: Display + Clone,
    //     U: Clone + Debug
    // {
    // }

fn main() {
    default_implementation();
    traits_as_parameters();
}
