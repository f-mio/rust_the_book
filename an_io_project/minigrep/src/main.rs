extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;


fn main() {
    // let args: Vec<String> = env::args().collect();

    // 12.1. コマンドライン引数を受け付ける
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // 13.3 入出力プロジェクトを改善する。
    let config = Config::new(env::args()).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // // {}を探しています。
    // println!("Searching for {}", config.query);
    // // {}というファイルの中。
    // println!("In file {}", config.filename);

    // 12.2 ファイルを読み込む
    // run(config);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {

//         if args.len() < 3 {
//             // panic!("not enough arguments.")
//             return Err("not enough arguments.")
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config {query, filename})
//     }
// }

// // fn parse_config(args: &[String]) -> Config {
// //     let query = args[1].clone();
// //     let filename = args[2].clone();

// //     Config { query, filename }
// // }

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let mut f = File::open(config.filename)?;// .expect("file not found.");

//     let mut contents = String::new();
//     f.read_to_string(&mut contents)?;
//         // // ファイルの読み込み中に問題がありました
//         // .expect("something went wrong the file");

//     println!("With text:\n{}", contents);

//     Ok(())
// }
