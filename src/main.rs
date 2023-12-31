use text_io::read;
use rand::Rng;
use std::time::Instant;

fn main() {
    let mut num_tests = 10;
    let mut min_value: i32 = 30;
    let mut max_value: i32 = 100;
    let mut rng = rand::thread_rng();
    println!("Hello, world!");
    println!("Enter the number of tests (hit enter for {}):", num_tests);
    let mut line: String = read!("{}\n"); 
    match line.parse::<i32>() {
        Ok(result) => num_tests = result,
        Err(_e) => println!("Using default value for number of tests: {}", num_tests),
    }
    println!("Enter the minvalue (hit enter for default {}): ", min_value);
    line = read!("{}\n");
    match line.parse::<i32>() {
        Ok(result) =>  min_value = result,
        Err(_e) => println!("Using default value for minvalue: {}", min_value),
    }
    println!("Enter the maxvalue (hit enter for default {}): ", max_value);
    line = read!("{}\n");
    match line.parse::<i32>() {
        Ok(result) => max_value = result,
        Err(_e) => println!("Using default value for mixvalue: {}", max_value),
    }
    println!("You wanted to do {} tests ranging from {} to {}", num_tests,min_value,max_value);
    
    let mut correct = 0;
    let mut errors = 0;
    let mut points = 0;

    let mut n1: i32 = rng.gen_range(min_value..=max_value);
    let mut n2: i32 = rng.gen_range(min_value..=max_value);
    let start = Instant::now();
    while correct < num_tests {
        println!("Question {}: ", correct+1);
        println!("{} * {} ??", n1, n2);
        let result: i32 = read!();
        if result == n1 * n2 {
            correct += 1;
            println!("Correct!");
            n1 = rng.gen_range(min_value..=max_value);
            n2 = rng.gen_range(min_value..=max_value);
            points += n1 * n2;
        } else {
            errors +=1;
        }

    }
    let duration = start.elapsed();

    println!("END OF GAME! POINTS: {}, ERRORS: {}. See you next time!", points, errors);
    println!("It took {:?}s. Congrats!",(duration.as_millis() as f64)/1000.0);
}
