// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}",x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

fn main() {
    println!("Hello, world!");

    another_function(11111111);
    print_labeled_measurement(1024, 's')
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);  // 別の関数
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}   {}", value, unit_label)
}
