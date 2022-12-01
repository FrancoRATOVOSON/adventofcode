use std::{env::current_dir, fs::read_to_string};

fn main() {
    // ----------- Get the inputs
    let mut cwd = current_dir().expect("Error when finding current working directory");
    cwd.push("inputs.txt");
    let text_inputs = read_to_string(cwd.into_os_string()).expect("Error when reading file");

    // ----------- Turn input into array
    let input_array = text_inputs.split("\n\n");
    let mut elves_foods=Vec::new();
    for input in input_array {
        let mut elve_foods=Vec::new();
        for food in input.split("\n") {
            elve_foods.push(food.parse::<i32>().unwrap());
        }
        elves_foods.push(elve_foods);
    };

    // ------------ Reduce to list of numbers
    let mut elves_calories=Vec::new();
    for foods in elves_foods {
        let mut sum=0;
        for food in foods {
            sum = sum + food
        }
        elves_calories.push(sum)
    }
    println!("List: {:?}",elves_calories);

    // ------------ Find the maximum
    let mut max_elf=0;
    let mut i=0;
    let mut j=elves_calories.len() / 2;
    loop {
        max_elf = if elves_calories[i] > max_elf {
            elves_calories[i]
        } else if elves_calories[j] > max_elf {
            elves_calories[j]
        } else {
            max_elf
        };
        if j<elves_calories.len() {
            j = j + 1
        }
        if i<(elves_calories.len() / 2)+1 {
            i = i + 1
        }
        if i>=elves_calories.len() / 2 && j>=elves_calories.len() {
            break;
        }
    }

    // --------------- Finding the top 3
    elves_calories.sort();
    let top_3:&i32 = &elves_calories[elves_calories.len()-3..].iter().sum();

    // ---------------- Print the result
    println!("Max: {}",max_elf);
    println!("Top3: {}",top_3);
}
