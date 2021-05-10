use std::fmt::Display;

fn main() {
    // # Motivation

    // - Last time, we had a problem...
    fn largest<T: PartialOrd>(list : &[T]) -> &T{
        let mut largest = &list[0];
        for i in 0..list.len(){
            if &list[i] > largest{
                largest = &list[i];
            }
        }
        largest        
    }
    let list = &vec![1, 2, 3];
    largest(list);

    // - This is fixed with traits, so...

    // # Defining a trait (& Default Implementations)

    trait Summary{
        fn summarize(&self) -> String{
            "Read more...".to_string()
        }
    }

















    // # Implementing a trait

    struct Tweet{
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Summary for Tweet{
        fn summarize(&self) -> String{
            format!("{}: {}", self.username, self.content)
        }
    }

    // Note: We can only implement traits to local types!















    // # Traits w/ Functions

    // ## Traits as parameters

    fn notify(item: impl Summary) -> String {
        format!("{}", item.summarize())
    }

    fn notify2<T: Summary> (item: T) -> String {
        format!("{}", item.summarize())
    }

    fn notify3<T>(item: T) -> String 
        where T: Summary{
        format!("{}", item.summarize())
    }

















    // ## Returning traits

    struct NewsArticle{
        title: String,
        content: String,
    }

    impl Summary for NewsArticle{}

    fn return_summarizable() -> impl Summary{
        // if switch{
            NewsArticle{
                title: "ok".to_string(),
                content: "ok".to_string(), 
            }
        // }else{
        //     Tweet{
        //         username: "ok",
        //         content: "ok",
        //         reply: false,
        //         retweet: false,
        //     }
        // }
    }

    // We can only return ONE type for now, we'll find a fix for this later.

    // This is useful in terms of closure of iterators 
    // (~ also, more on this later ¯\_(ツ)_/¯). 














    // # Traits to conditionally implement methods

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
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

    // Let's go back to our motivation now!

}
