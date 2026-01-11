use std::collections::{HashMap, btree_map::Keys};
use std::io;

fn main() {
    //hm_test1();
    exercises();
}

fn exercises () {
    //mean_median_mode();
    //let pgltn = parse_to_pig_latin(&String::from("first apple of the bunch."));
    //println!("{pgltn}");
    company_db();
}

fn company_db () {
    println!("useage:\n
    add user to deptartment -> add -n Sally -d Engineering\n
    list all in department | list all by dept. sorted by alpha -> list -d Engineering|-a\n
    exit the cli -> exit");

    let add_flags = ["-n", "-d"];
    let list_flags = ["-d", "-a"];

    let mut flag_pairs: HashMap<String, String> = HashMap::new();
    let mut database = HashMap::<String, Vec<String>>::new();

    let mut running = true;
    while running {
        // Get user input
        let mut user_in = String::new();
        io::stdin().read_line(&mut user_in).expect("Failed to read line");

        let input_parts: Vec<&str> = user_in
                        .split_whitespace()
                        .collect();

        //println!("{input_parts:#?}");

        let flags: Vec<&&str> = 
                    input_parts
                        .iter()
                        .filter(|x| x.contains("-"))
                        .collect();
        //println!("{flags:#?}");

        let command = user_in
                        .split_whitespace()
                        .enumerate()
                        .next()
                        .unwrap_or_default()
                        .1;

        if flags.len() == 0 && command != "exit" {
            println!("Flags must be passed.");
            continue;
        }
        else if command == "exit" {
            running = false;
            continue;
        }

        for value in &input_parts {
            if (value.contains("-")) {
                let current_pos = &input_parts
                    .iter()
                    .position(|x| x == value);

                let next_value = &input_parts.get(current_pos.unwrap() + 1);

                if Option::is_some(next_value) {
                    flag_pairs.insert(value.to_string().clone(), 
                        next_value.unwrap().to_string().clone());
                }
            }
        }
        println!("Flag Pairs:\n{flag_pairs:#?}");

        // Parse user input
        match command {
            "add" => {
                // collect unknown flags
                let unknown_flags: Vec<&&str> = 
                    flags
                        .clone()
                        .into_iter()
                        .filter(|x| !add_flags.contains(x))
                        .collect();
                
                // report them to user
                if unknown_flags.len() > 0 {
                    println!("{unknown_flags:?} flag(s) not recognized.")
                }

                // get name
                let name = &flag_pairs["-n"];
                
                // get department
                let dept = &flag_pairs["-d"];

                database
                    .entry(dept.to_string()).or_insert(Vec::new()).push(name.to_string());
            },
            "list" => {
                let unknown_flags: Vec<&&str> = 
                    flags
                        .clone()
                        .into_iter()
                        .filter(|x| !list_flags.contains(x))
                        .collect();
                
                // report them to user
                if unknown_flags.len() > 0 {
                    println!("{unknown_flags:?} flag(s) not recognized.");
                }
                else if flags.len() > 1 {
                    println!("Too many flags for command 'list'.\n
                    i.e. list -d");
                }

                if flags[0] == &"-d" {
                    // get department
                    if (!&flag_pairs.contains_key("-d")){
                        println!("No such flag pair '-d'");
                        continue;
                    }
                    let dept = &flag_pairs["-d"];
                    if Option::is_none(&database.get_key_value(dept)) {
                        println!("{dept} yielded no value in database.");
                        continue;
                    }
                    let filtered_hm = &database.get_key_value(dept).unwrap();
                    let mut names = filtered_hm.1.clone();
                    let dept = filtered_hm.0;
                    println!("Department: {dept}");
                    names.sort_by(|x, y| x.cmp(y));
                    for name in names {
                        println!("{name}")
                    }
                }
                else {
                    for entry in &database {
                        let mut names = entry.1.clone();
                        let dept = entry.0;
                        println!("Department: {dept}");
                        names.sort_by(|x, y| x.cmp(y));
                        for name in names {
                            println!("{name}")
                        }
                    }
                }
            },
            "exit" => running = false,
            _ => println!("{command} command not recognized.")
        }
    }
}

fn parse_to_pig_latin (text: &String) -> String {
    let str_collection = text.split_whitespace().enumerate();
    let mut pl_str_collection = Vec::<String>::new();
    let vouls = ['a', 'e', 'i', 'o', 'u', 'y'];
    for (_, word) in str_collection {
        let first_letter = word.chars().next().unwrap();
        if vouls.contains(&first_letter) {
            let ltn = format!("{word}hay");
            pl_str_collection.push(ltn);
        }
        else if word.contains(".") {
            let core = word.split_at(1).1;
            let core = &core.replace(".", "");
            let ltn = format!("{core}-{first_letter}ay.");
            pl_str_collection.push(ltn);
        }
        else {
            let core = word.split_at(1).1;
            let ltn = format!("{core}-{first_letter}ay");
            pl_str_collection.push(ltn);
        }
    }

    let mut new_prase: String = String::new();
    for nword in &pl_str_collection {
        if *nword != *pl_str_collection.last().unwrap() {
            new_prase += (nword.to_owned() + " ").as_str();
        }
        else {
            new_prase += nword.as_str();
        }
    }
    new_prase
}

fn mean_median_mode() {
    let integers_list = [0_u16,1,3,4,5,5,6,12,4546,23,7,112,4];

    // Mean
    let mut sum = 0;
    for number in integers_list {
        sum += number;
    }
    let mean = (sum as usize / integers_list.len()) as u16;

    // Median
    let center_cnt_rough = (integers_list.len() / 2) as f32;
    let center_cnt = center_cnt_rough.round() as usize;
    let median = integers_list.get(center_cnt).unwrap();

    // Mode
    let mut numbers: HashMap<u16, u8> = HashMap::<u16, u8>::new();
    let mut higest_cnt: (u16, u8) = (0, 0);
    for number in integers_list {
        let count = numbers.entry(number).or_insert(0);
        *count += 1;
        if *count > higest_cnt.1 {
            higest_cnt = (number, *count);
        }
    }
    let mode = higest_cnt.0;
    
    println!("Mean: {mean}, Median: {median}, Mode: {mode}.");
}

fn hm_test1(){
    let blue_team = String::from("Blue");
    let red_team = String::from("Red");
    let yello_team = String::from("Yello");

    let mut scores = HashMap::new();
    // w/o clone here field_name ownership moves to the scores HM.
    scores.insert(&blue_team, 10);
    scores.insert(&red_team, 50);

    print_score(&blue_team, &scores);

    scores.insert(&red_team, 42);

    print_score(&red_team, &scores);

    scores.entry(&blue_team).or_insert(100);
    scores.entry(&yello_team).or_insert(100);

    print_scores(&scores);
}

fn get_score<'a>(team: &'a String, scores: &HashMap<&String, i32>) -> 
    (&'a String, String) {
    (team, scores.get(team).unwrap_or(&0).to_string())
}

fn print_score(team: &String, scores: &HashMap<&String, i32>) {
    let score = get_score(team, &scores);
    let score_team = score.0;
    let score_value = score.1;
    println!("{score_team}: {score_value}");
}

fn print_scores(scores: &HashMap<&String, i32>) {
    println!("\nScores:");
    for (k, v) in scores {
        println!("{k}: {v}");
    }
}