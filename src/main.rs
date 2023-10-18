use text_io::read;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Hello, world!");
    println!("Enter the amount of tests:");
    let num_tests: i32 = read!();
    println!("Enter the minvalue: ");
    let min_value: i32 = read!();
    println!("Enter the maxvalue: ");
    let max_value: i32 = read!();

    println!("You wanted to do {} tests ranging from {} to {}", num_tests,min_value,max_value);
    
    let mut correct = 0;
    let mut errors = 0;
    let mut points = 0;

    let mut n1: i32 = rng.gen_range(min_value..max_value);
    let mut n2: i32 = rng.gen_range(min_value..max_value);
    while correct < num_tests {
        println!("Question {}: ", correct+1);
        println!("{} * {} ??", n1, n2);
        let result: i32 = read!();
        if result == n1 * n2 {
            correct += 1;
            println!("Correct!");
            n1 = rng.gen_range(min_value..max_value);
            n2 = rng.gen_range(min_value..max_value);
            points += n1 * n2;
        } else {
            errors +=1;
        }

    }

    println!("END OF GAME! POINTS: {}, ERRORS: {}. See you next time!", points, errors);
}
