use std::{io::{self, Write}, process::exit};
use rand::Rng;
use std::process::Command;

fn generate_binaries() -> Vec<i32>
{
    let mut rng = rand::thread_rng();
    let mut binaries: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0];

    // Replace each element in binaries with a generated 0 or 1
    for i in binaries.iter_mut()
    {
        let temp_binary: i32 = rng.gen_range(0..2);
        *i = temp_binary;
    }

    return binaries;
}

fn get_index_position(binaries: Vec<i32>) -> Vec<i32>
{
    let mut position: Vec<i32> = vec![];
    let mut index_counter: i32 = 0;

    // Iterate through the binaries
    for i in binaries.iter()
    {
        // If binary is 1, save the position
        if *i != 0 {
            position.push(index_counter);
        }

        // Print each binary
        print!("  ");
        print!("{} ", *i);
        io::stdout().flush().unwrap();

        index_counter += 1;
    }

    return position;
}

fn get_result(index_pos: Vec<i32>) -> i32
{
    let mut result: i32 = 0;

    // Iterate through the positions of binaries that are 1
    for i in index_pos.iter()
    {
        // Assign values to the binaries that are 1
        match *i
        {
            0 => result += 128,
            1 => result += 64,
            2 => result += 32,
            3 => result += 16,
            4 => result += 8,
            5 => result += 4,
            6 => result += 2,
            7 => result += 1,
            
            _ => {
                println!("[ERROR] Index Position");
                exit(0);
            }
        };
    };

    return result
}

fn read_number() -> i32
{
    loop
    {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input)
                   .expect("Can't read user input");

        // Try to parse the input as an i32
        match user_input.trim().parse::<i32>()
        {
            Ok(num) => return num,
            Err(_) => println!("Please enter a number"),
        }
    };
}

fn main() 
{
    println!("Read the byte and enter the decimal value");
    println!();

     // Generate a vector of random binary values (0 or 1)
    let binaries: Vec<i32> = generate_binaries();

    // Get the positions of the binaries that are set to 1
    let index_pos: Vec<i32> = get_index_position(binaries);

    println!();
    println!("128  64  32  16   8   4   2   1");
    println!();

    // Calculate the result based on the indices of the binaries that are 1
    let result: i32 = get_result(index_pos);

    // Read the user input
    let user_input: i32 = read_number();
	
    // Check if the user's input matches the calculated result
    if user_input == result {
        println!("Correct!");
    }
    else {
        println!("Wrong answer");
    }

    // Pause the command line window until the user presses a key
    Command::new("cmd").args(&["/C", "pause"]).status()
                       .expect("Error on executing pause");
}