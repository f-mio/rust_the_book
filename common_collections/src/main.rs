

fn sec8_1() {
    let mut v1 = Vec::new();

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    println!("first value of v1 is {}!", &v1[0]);

    let v2 = vec![1, 2,3,4,5];

    let third: &i32 = &v2[2];

    match v2.get(2) {
        Some(third) => println!("The third element is {}.", third),
        None => println!("Threr is no third element.")
    }

    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{}", i);
    }

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4{
        *i += 50;
    }
    for i in &v4 {
        println!("changed value --> {}", i);
    }

    enum SpreadsheetCell {
        Int(i32), Float(f64), Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn sec8_2() {
    // 新しい文字列型を定義
    let mut s0 = String::new();

    let data = "initial contents";
    
    let s = "initial contints".to_string();
    println!("initial string is \"{}\".", s);

    let s: String = s0 + &data.to_string();
    println!("initial string is \"{}\".", s);

    let s: String = String::from("initial string");
    println!("initial value is \"{}\"", s);

    let hello_japanese: String = String::from("にんにちは！");
    println!("日本語の挨拶は、\"{}\"です。", hello_japanese);


    //// 文字列を更新する

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}",s);


    // +演算子、またはformat!マクロで連結
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("Stringの足し算: {} (String + &String)", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);


    //// 文字列の添え字アクセスする。

    // 下記のコードはエラー
    // let s1 = String::from("hello");
    // let h = s1[0];

    let len = String::from("Здравствуйте").len();
    println!("Здравствуйтеの文字数は、{}", len);

    // 下記もエラーが返ってくる
    // let hello = "Здравствуйте";
    // let first_char = &hello[0];
    // println!("First Char of 'Здравствуйте' is {}.", first_char);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);


    //// 文字列を走査するメソッド群
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "日本語".chars() {
        println!("{}", c);
    }

    for c in String::from("日本語").chars() {
        println!("{}", c);
    }

    // バイトで返す
    println!("バイトで、'नमस्ते'を返した場合の出力");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    println!("バイトで、'日本語'を返した場合の出力");
    for b in String::from("日本語").bytes() {
        println!("{}", b);
    }
}

fn sec8_3() {
    //// 新規ハッシュマップを生成する。
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // another metho. (generation of hash map)
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scrores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


    //// ハッシュマップと所有権
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    println!("{}, {}", field_name, field_value);

    map.insert(field_name, field_value);

    // println!("{}, {}", field_name, field_value);

    //// ハッシュマップの値にアクセスする
    let team_name = String::from("Yellow");
    let score = scores.get(&team_name);
    println!("{} teams score is {}.", team_name, score.unwrap_or(&0));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //// ハッシュマップの更新
    ////// 値を上書きする
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);

    println!("{:?}", scores2);


    ////// 値がなかった時のみ値を挿入する
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);

    scores3.entry(String::from("Blue")).or_insert(30);
    scores3.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores3);

    ////// 古い値に基づいて値を更新する
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    println!("Hello, world!");

    sec8_1();

    sec8_2();

    sec8_3();
}
