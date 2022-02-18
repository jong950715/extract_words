extern crate csv;
use std::thread;
use reqwest;
use std::io;
use std::fs;
use std::path::Path;
use tinyjson::{JsonValue};
use regex::Regex;

static PAPER_NAME: &str = "./src/paper.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut words = Vec::new();
    get_words(&mut words);

    println!("{:#?}", &words);

    // let word = String::from("apple");
    // let _word = word.clone();

    let mut tasks = Vec::new();

    for word in words.iter(){
        let _word = word.clone();
        
        tasks.push((word, tokio::spawn(async move {
            get_frq(&_word).await
        })));
    }

    

    let mut res = Vec::new();
    for (w, t) in tasks {
        let _t = t.await.unwrap().to_string();

        res.push((w, _t));
    }

    res.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());


    println!("{:#?}", res);
    //wrap_with_frq(&String::from("apple")).await;
    //tasks.push(tokio::spawn(async move { sleep_then_print(i).await }));

    let mut wtr = csv::Writer::from_writer(io::stdout());
    res.iter().for_each(|s| {wtr.serialize(&s);} );
    //wtr.serialize(&res);
    wtr.flush().unwrap();
    
    Ok(())
}
fn my_parse(j: &JsonValue) -> f64{
    let j = &j[0]["timeseries"];
    let numbers;

    if let JsonValue::Array(v) = j {
        numbers = v;
    } else {
        panic!();
    }
    
    let numbers = &numbers[&numbers.len()-1];

    let number;
    if let JsonValue::Number(n) = numbers {
        number = n.clone();
    } else {
        panic!();
    }

    number
}
//async fn wrap_with_frq(word: &String) -> (&String, f64);
async fn get_frq(word: &String) -> f64 {
    thread::sleep_ms(200);
    let url_pre = "https://books.google.com/ngrams/json?content=";
    let url_suf = "&year_start=2000&corpus=26&smoothing=10&case_insensitive=true";
    let url = format!("{}{}{}", url_pre, word, url_suf);

    let resp: JsonValue = reqwest::get(&url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .parse()
        .unwrap();

    let frq = my_parse(&resp);
    
    //println!("{:#?}", &frq);

    frq
}

fn get_words(words: &mut Vec<String>) -> &Vec<String> {
    let _path = std::env::current_dir().unwrap();
    let _path = _path.to_str().unwrap();

    println!("{}", _path);

    let paper_name = Path::new(PAPER_NAME);
    let paper = fs::read_to_string(paper_name).unwrap();
    
    let re = Regex::new(r"[^\w\s]").unwrap();
    let paper = re.replace_all(&paper, "");
    
    //     let re = Regex::new(r"[A-Za-z]").unwrap();
    // let result = re.replace_all("Hello World!", "x");
    // println!("{}", result); // => "xxxxx xxxxx!"

    println!("{}", paper);

    for word in paper.split_whitespace() {
        words.push(String::from(word));
    }

    words
}
