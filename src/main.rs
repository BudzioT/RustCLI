use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    loop {
        println!("Programs:\n\t1 - median and mode,\n\t2 - pig latin,\n\t3 - employee simulation");
        println!("\nChoose:");
        let mut answer: String = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read decision");
        let answer = answer.trim().to_string();

        println!();
        match answer.to_lowercase().as_str() {
            "1" => median_and_mode(),
            "2" => pig_latin(),
            "3" => company_employees(),
            "quit" => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Unknown choice, quitting...");
                break;
            }
        }
    }

    println!("Thanks for using these Rust programs");
}


fn median_and_mode() {
    // Read input as a line
    println!("Write a vector of integers seperated by spaces:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error in reading input");

    // Convert it to Vector of ints
    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Error while parsing"))
        .collect();

    let median = median(&mut numbers);
    let mode = mode(&mut numbers);

    println!("\nMedian: {}\nMode: {}", median, mode);


    // Calculate median of provided vector
    fn median(numbers: &mut Vec<i32>) -> f32 {
        // Sort the vector
        numbers.sort();

        let median: f32;
        let middle: i32 = (numbers.len() / 2) as i32;

        // If length is odd, return the middle number
        if numbers.len() % 2 != 0 {
             median = numbers[middle as usize] as f32;
        } else {
            // Otherwise return average of the near-middle numbers
            median = (numbers[middle as usize] + numbers[(middle - 1) as usize]) as f32 / 2.0;
        }

        median
    }

    // Calculate mode of provided vector
    fn mode(numbers: &mut Vec<i32>) -> i32 {
        // Create hashmap and get a first number from the vector
        let mut occurrences = HashMap::new();
        let mut highest_number = match numbers.get(0) {
            None => panic!("Vector is empty!"),
            Some(num) => num.clone()
        };

        // Go through each of the numbers, increasing count of occurrences
        for num in numbers.clone() {
            let count = occurrences.entry(num).or_insert(0);
            *count += 1;

            // If current number appears more time than the highest one, change it
            if *count > occurrences[&highest_number] {
                highest_number = num;
            }
        }

        highest_number
    }
}

// Convert words to pig latin
fn pig_latin() {
    println!("Enter a string:");

    // Read the input
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Error while reading input string");

    let mut words = String::new();
    // Go through each word and check the first letter
    for word in string.split_whitespace() {
        let first_letter: char = word.chars().collect::<Vec<char>>()[0];
        match first_letter.to_ascii_lowercase() {
            // If it's a vowel, add hay to the word
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => words.push_str(format!("{word}{} ", "hay").as_str()),
            // Otherwise, put the first letter at the end and add ay to the word
            _ => {
                let mut result_word: String = String::new();
                let length: usize = word.len();

                // Safeguard against one-letter word
                if length > 1 {
                    // Get the first letter
                    let mut iter = word.chars().into_iter();
                    iter.next();
                    // Push it to the end of word
                    result_word = iter.collect();
                    result_word.push(first_letter);
                }

                // Add ay to the end
                words += format!("{}ay ", result_word).as_str();
            }
        }
    }

    // Trim the result and show it
    let result = String::from(words.trim());
    println!("{string} in pig latin is: {result}");
}

// Simulate database with employees from companies
fn company_employees() {
    // Create a list of companies, with a list of employees based on their department
    let mut companies: HashMap<String, HashMap<String, String>> = HashMap::new();

    let mut query: String = String::new();
    // Command-line loop
    while query != "quit" {
        query = String::new();
        let mut company: String = String::new();
        // Get company from the input
        print!("Company name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut company).expect("Failed to read company name");
        let company: String = company.trim().to_string().to_lowercase();

        let mut employees: HashMap<String, String> = companies.entry(company.clone())
            .or_insert(HashMap::new()).clone();

        // Get query from input
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut query).expect("Failed to read the command");
        let query: String = query.trim().to_string().to_lowercase();

        // Variables to manage commands
        let mut add: bool = false;
        let mut show: bool = false;

        // Split query into tokens
        let mut words = query.split_whitespace();
        // Set the correct flags based on the first token
        match words.next().unwrap() {
            "add" => add = true,
            "show" => show = true,
            "quit" => break,
            _ => {
                println!("Unknown query token!");
                continue;
            }
        }

        // Handle adding user
        if add {
            // Check for proper syntax
            if !query.contains("to") {
                println!("Add query doesn't contain the target department");
                continue;
            }

            // Variables to store name and department
            let mut name: String = String::new();
            let mut department: String = String::new();
            // Flag to indicate that department name starts
            let mut dep_start: bool = false;

            // Go through each word
            for (_i, word) in words.enumerate() {
                match word {
                    // If it is "to" keyword, then set the next tokens as department
                    "to" => dep_start = true,
                    // Update department or name
                    other => {
                        if dep_start {
                            department.push_str(other);
                            department.push(' ');
                        } else {
                            name.push_str(other);
                            name.push(' ');
                        }
                    }
                }
            }
            let name: String = name.trim().to_string();
            let department: String = department.trim().to_string();
            println!("Added {name} to {department}");
            // Add the employee
            employees.insert(name, department);
            companies.insert(company, employees);
        }

        // Handle showing data
        else if show {
            // If user wants to see specific department, get it from them
            match words.next() {
                None => {
                    // Show all employees
                    println!("Employees:\n{{");
                    for (name, department) in employees {
                        println!("{name} in {department}");
                    }
                    println!("}}");
                }
                Some(word) => {
                    if word == "from" {
                        // Get department from the rest of input
                        let mut department: String = String::new();
                        for word in words {
                            department.push_str(word);
                            department.push(' ');
                        }
                        let department = department.trim();

                        println!("Employees in {department}:\n{{");
                        for (name, dep) in employees {
                            if dep == department {
                                println!("{name}");
                            }
                        }
                        println!("}}");
                    }
                    else {
                        // Otherwise, just show all employees
                        println!("Employees:\n{{");
                        for (name, department) in employees {
                            println!("{name} in {department}");
                        }
                        println!("}}");
                    }
                }
            }
        }
    }

    println!("Thanks for using the system!");
}