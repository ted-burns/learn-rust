use std::io::{self, Write};

fn main() {
    print!("Enter the n for which we will generate the nth fibonacci number: ");
    io::stdout().flush().expect("Unable to flush to std out");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    
    let n = input.trim().to_string().parse::<usize>().expect("Unable to parse input as integer");

    let fib: u128 = match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let mut nums: Vec<u128> = vec![1, 1];
            for i in 2..n {
                nums.push(nums.get(i - 1).expect("this number exists") + nums.get(i - 2).expect("this number exists too"));
            }
            *nums.last().expect("There is a last element")
        },
    };

    println!("{}", fib);


}
