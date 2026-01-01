use std::thread;
use std::time::Duration;
use std::collections::HashMap;



// struct Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     calculation: T,
//     value: Option<u32>
// }

// impl<T> Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             },
//         }
//     }
// }

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value_map: HashMap<u32, Option<u32>>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value_map.get(&arg) {
            Some(v) => v.unwrap(),
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, Some(v));
                v
            },
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


fn generate_workout(intensity: u32, random_number: u32) {

    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // let expensive_result = simulated_expensive_calculation(intensity);

    let mut expensive_result = Cacher::new( |num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {

        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break tody! Remember to stay gydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 300;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
    // simulated_expensive_calculation(12);
}
