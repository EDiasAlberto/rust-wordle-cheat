use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct WordleResp {
    id: u32,
    solution: String,
    print_date: String,
    days_since_launch: u32,
    editor: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.nytimes.com/svc/wordle/v2/2025-11-15.json")?.json::<WordleResp>();
    match resp {
        Ok(response) => println!("{:#?}", response),
        Err(_) => println!("ERROR"),
    }
    Ok(())
}
