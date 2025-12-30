
struct Point<T, U> {
    x: T, y: U
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item >largest {
            largest = item;
        }
    }
    largest
}

// sec10.1時点の知識ではエラーになってしまう・
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn sec10_1() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    // let result = largest(&number_list);
    println!("The largest number is {}.", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    // let result = largest(&char_list);
    println!("The largest char is {}.", result);


    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y : 'C'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn sec10_2() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        // アメリカ、ペンシルベニア州、ピッツバーグ
        location: String::from("Pittsburgh, PA, USA"),
        // アイスバーグ
        author: String::from("Iceburgh"),
        // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("1 new article: {}", article.summarize());


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn longest<'a>(item1: &'a str, item2: &'a str) -> &'a str {
    if item1.len() > item2.len() {
        item1
    } else {
        item2
    }
}


fn sec10_3() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);


    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }

    let string1 = String::from("long string is long");
    let mut result: String;
    {
        let string2 = String::from("xyz");
        let result = longest(string2.as_str(), string1.as_str());
        println!("The longest string is '{}'", result);
    }

}

fn main() {
    sec10_1();

    sec10_2();

    sec10_3();
}
