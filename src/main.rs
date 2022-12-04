use std::{env::current_dir, fs::read_to_string};

type SectionRange = Vec<i32>;
type AssignementGroup = Vec<SectionRange>;

fn main() {
    // ----------- Get the inputs
    let mut cwd = current_dir().expect("Error when finding current working directory");
    cwd.push("inputs.txt");
    let text_inputs = read_to_string(cwd.into_os_string()).expect("Error when reading file");

    // ----------- Turn input into array
    let section_pairs = text_inputs.split("\n").map(|pair| {
        let assignements = pair.split(",");
        assignements.map(|section| {
            section.split("-").map(|number| number.parse::<i32>().unwrap()).collect::<SectionRange>()
        }).collect::<AssignementGroup>()
    }).collect::<Vec<AssignementGroup>>();

    // ------------- Get all assignement pair
    let assignement_pair = section_pairs.iter().map(|pair|{
        is_contained(pair)
    }).collect::<Vec<bool>>();

    // ------------- Get all assignement pair with one range fully contain the other
    let assignement_with_range = assignement_pair.iter().filter(|pair| {
        **pair == true
    }).collect::<Vec<&bool>>();

    // println!("With range: {:?}",assignement_pair);
    println!("Step1: {}",assignement_with_range.len());

    // ------------- Get all assignement pair with ranges overlapping
    let assignement_overlaps = section_pairs.iter().map(|pair|{
        is_overlapping(pair)
    }).filter(|pair| {
        *pair == true
    }).collect::<Vec<bool>>();
    
    // println!("With range: {:?}",assignement_pair);
    println!("Step1: {}",assignement_overlaps.len());
}

fn is_contained(group:&AssignementGroup)->bool {
    let assignement_1 = &group[0];
    let assignement_2 = &group[1];
    let assignement2_in_assignement1 = assignement_2[0] >= assignement_1[0] && assignement_2[1] <= assignement_1[1];
    let assignement1_in_assignement2 = assignement_1[0] >= assignement_2[0] && assignement_1[1] <= assignement_2[1];
    assignement1_in_assignement2 || assignement2_in_assignement1
}

fn is_overlapping(group:&AssignementGroup)->bool {
    let assignement_1 = &group[0];
    let range_1 = assignement_1[0]..assignement_1[1]+1;
    let assignement_2 = &group[1];
    let range_2 = assignement_2[0]..assignement_2[1]+1;
    let assignement2_in_assignement1 = range_1.contains(&assignement_2[0]) ||range_1.contains(&assignement_2[1]);
    let assignement1_in_assignement2 = range_2.contains(&assignement_1[0]) ||range_2.contains(&assignement_1[1]);
    assignement2_in_assignement1 || assignement1_in_assignement2
}