fn main() {
    let number = 10;

    if number < 6 {
        println!("condition was true!");
    } else {
        println!("condition was false.");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number %2 ==0 {
        println!("number is divisible by 2");
    } else {
        println!("数値は2,3,4で割り切れません");
    }

    let condition = false;
    let new_num = if condition {5} else {6};
    println!("The value of number is: {}", new_num);

    // loop {
    //     println!("again!")
    // }


    // LOOP USING LOOP
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 66;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    // LOOP USING WHILE
    let mut number3 = 3;
    while number3 != 0 {
        println!("{}!", number3);

        number3 -= 1;
    }

    println!("LIFTOFF!!");


    // LOOP USING FOR
    let a = [ 10, 20, 30, 40, 50];
    let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }
    for elem in a {
        println!("the value is: {}", elem);
    }

    for number4 in (1..4).rev() {
        println!("{}!", number4);
    }
    println!("LIFT OFF!!!!");


}
