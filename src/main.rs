use std::{env::current_dir, fs::read_to_string};

fn main() {
    // ----------- Get the inputs
    let mut cwd = current_dir().expect("Error when finding current working directory");
    cwd.push("inputs.txt");
    let text_inputs = read_to_string(cwd.into_os_string()).expect("Error when reading file");

    // ------------ Game rules
    let me=1;
    let opponent = 0;

    // ----------- Turn input into array
    let game_rounds = text_inputs.split("\n").map(|round| {
        let round_splited = round.split(" ");
        round_splited.collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>();

    // ------------- Calculate all game scores
    let games_scores = game_rounds.iter().map(|game| {
        let game_shape_points = (shape_to_point(game[opponent]),shape_to_point(game[me]));
        let game_match_points=shape_match(game[opponent], game[me]);
        (game_shape_points.0+game_match_points.0,game_shape_points.1+game_match_points.1)
    }).collect::<Vec<(i32,i32)>>();

    // -------------- Get my scores
    let my_scores=games_scores.iter().map(|game| game.1).collect::<Vec<i32>>();

    // -------------- Get my total points
    let my_points:i32 = my_scores.iter().sum();
    println!("Step1: {}",my_points);

    // ------------ Calculate step 2 game score
    let new_games_scores = game_rounds.iter().map(|game| {
        let my_move = shape_to_move(game[opponent], game[me]);
        let game_shape_points = (shape_to_point(game[opponent]),shape_to_point(my_move));
        let game_match_points=shape_match(game[opponent], my_move);
        (game_shape_points.0+game_match_points.0,game_shape_points.1+game_match_points.1)
    }).collect::<Vec<(i32,i32)>>();

    // -------------- Get my scores for step 2
    let my_new_scores=new_games_scores.iter().map(|game| game.1).collect::<Vec<i32>>();

    // -------------- Get my total points
    let my_new_points:i32 = my_new_scores.iter().sum();
    println!("Step2: {}",my_new_points);
}

fn shape_to_point(shape:&str)->i32{
    match shape {
        "A" | "X"=>1,
        "B" | "Y"=>2,
        "C" | "Z"=>3,
        _=>panic!("Wrong shape")
    }
}

fn shape_match(op:&str,me:&str)->(i32,i32) {
    match (op,me) {
        ("A","X")=>(3,3),
        ("A","Y")=>(0,6),
        ("A","Z")=>(6,0),
        ("B","X")=>(6,0),
        ("B","Y")=>(3,3),
        ("B","Z")=>(0,6),
        ("C","X")=>(0,6),
        ("C","Y")=>(6,0),
        ("C","Z")=>(3,3),
        _=>panic!("Wrong game, OP:{}, ME:{}",op,me)
    }
}

fn shape_to_move<'a>(op:&str,me:&str)->&'a str{
    match (op,me) {
        ("A","X")=>"Z",
        ("B","X")=>"X",
        ("C","X")=>"Y",
        ("A","Y")=>"X",
        ("B","Y")=>"Y",
        ("C","Y")=>"Z",
        ("A","Z")=>"Y",
        ("B","Z")=>"Z",
        ("C","Z")=>"X",
        _=>panic!("Wrong game II, OP:{}, ME:{}",op,me)
    }
}