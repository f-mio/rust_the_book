use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;


fn sec9_1() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}

fn read_username_from_file() -> Result<String, io::Error> {

    let f = File::open("hello3.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {

    let mut f = File::open("hello3.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello3.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


fn sec9_2() {
    // let f: u32 = File::open("hello.txt");
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 // ファイルを作成しようとしましたが、問題がありました。
    //                 // panic!("There was a problem opening the file: {:?}.", error);
    //                 panic!("Tried to create file but there was a problem: {:?}.", e)
    //             }
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // };

    // // unwrapを用いたエラーハンドリング
    let f = File::open("hello.txt").unwrap();

    // expectを用いたエラーハンドリング
    let f = File::open("hello2.txt").expect("Failtd to open hello2.txt");

    let user_name = read_username_from_file().unwrap();
    println!("User name is {:?}", user_name);

    let user_name = read_username_from_file2().unwrap();
    println!("User name is {:?}", user_name);

    let user_name = read_username_from_file3().unwrap();
    println!("User name is {:?}", user_name);
}

fn main() {

    // sec9_1();
    sec9_2();

}
