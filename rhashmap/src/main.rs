use std::{collections::HashMap};

fn main() {
    
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);


    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("Score for {}: {}", team_name, score);

    // for (team, score) in &scores {
    //     println!("Score for {}: {}", team, score);
    // }

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // println!("Map contains: {:?}", map);   




    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);


    // println!("{scores:?}");


    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{scores:?}");


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let value = map.entry(word);
        let value = value.or_insert(0);
        *value += 1;
    }

    let mut items = map.into_iter().collect::<Vec<_>>();

    // items.sort_by_key(|(_, count)| *count);


    items.sort_by(|a, b| a.0.cmp(b.0));




    // items.reverse();




    println!("{items:?}");


}
