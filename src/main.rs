use std::{env::current_dir, fs::read_to_string};

fn main() {
    // ----------- Get the inputs
    let mut cwd = current_dir().expect("Error when finding current working directory");
    cwd.push("inputs.txt");
    let text_inputs = read_to_string(cwd.into_os_string()).expect("Error when reading file");

    // ----------- Turn input into array
    let mut rucksacks = text_inputs.split("\n").map(|rucksack| {
        let rucksack_content = rucksack.split_at(rucksack.len()/2);
        (String::from(rucksack),common_letter(rucksack_content))
    }).collect::<Vec<(String,char)>>();

    // ------------- Calculate all priorites
    let priorities_list = rucksacks.iter().map(|rucksack| {
        get_priority(rucksack.1)
    }).collect::<Vec<i32>>();

    // -------------- Get prorities sum
    let priorities:i32 = priorities_list.iter().sum();
    println!("Step1: {}",priorities);

    // -------------- Group rucksacks by 3
    let mut grouped_rucksacks:Vec<Vec<(String,char)>>=Vec::new();
    loop {
        grouped_rucksacks.push(rucksacks.drain(..3).collect::<Vec<(String,char)>>());
        if rucksacks.len() <= 0 {
            break;
        }
    }

    // -------------- Calculate all priorites
    let priorities_list = grouped_rucksacks.iter().map(|rucksacks| {
        let priority_letter = common_item(rucksacks);
        get_priority(priority_letter)
    }).collect::<Vec<i32>>();

    // -------------- Get prorities sum
    let priorities:i32 = priorities_list.iter().sum();
    println!("Step1: {}",priorities);
}

fn common_letter(chars:(&str,&str))->char {
    for chr_1 in chars.0.chars().enumerate() {
        if chars.1.contains(chr_1.1) {
            return chr_1.1
        }
    }
    panic!("No Match on items");
}

fn common_item(rucksacks:&Vec<(String,char)>)->char {
    for chr in rucksacks[0].0.chars().enumerate() {
        if rucksacks[1].0.contains(chr.1) && rucksacks[2].0.contains(chr.1) {
            return  chr.1;
        }
    }
    panic!("No common items");
}

fn get_priority(item:char)->i32{
    let priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    (priorities.find(item).expect("Priority error ") + 1) as i32
}