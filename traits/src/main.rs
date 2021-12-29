pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more..)")
    }

    fn summarize_author(&self) -> String;

    fn attribute_to(&self) -> String {
        format!("(Read mroe from {}...)", self.summarize_author())
    }
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

    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct DM {
    pub user: String,
    pub message: String,
}

impl Summary for DM {
    fn summarize_author(&self) -> String {
        format!("from {}", self.user)
    }
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}


fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let dm = DM {
        user: String::from("bnbnb"),
        message: String::from("hello, nice to meet you"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("tweet: {}", tweet.attribute_to());
    println!("1 new dm: {}", dm.summarize());


    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['f', 'B', 'z', 's'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    } 
    largest
}
