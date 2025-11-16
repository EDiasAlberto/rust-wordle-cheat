use serde::Deserialize;
use chrono::offset::Utc;
use reqwest::blocking::get;
use std::error::Error;
use text_io::{read, scan};

/*
const WORDLE_URL: &str =
    "https://www.nytimes.com/svc/wordle/v2/2025-11-15.json";
const CONNECTIONS_URL: &str =
    "https://www.nytimes.com/svc/connections/v2/2025-11-15.json";
*/

const BASE_URL: &str = 
    "https://www.nytimes.com/svc";

// ---------- MODELS ---------- //

#[derive(Deserialize, Debug)]
struct WordleResp {
    id: u32,
    solution: String,
    print_date: String,
    days_since_launch: u32,
    editor: String,
}

#[derive(Deserialize, Debug)]
struct WordChoices {
    content: String,
    position: u8,
}

#[derive(Deserialize, Debug)]
struct ConnectionsCategories {
    title: String,
    cards: Vec<WordChoices>,
}

#[derive(Deserialize, Debug)]
struct ConnectionsResp {
    id: u32,
    print_date: String,
    editor: String,
    categories: Vec<ConnectionsCategories>,
}

// ---------- HELPERS ---------- //

fn fetch_json<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, Box<dyn Error>> {
    Ok(get(url)?.json::<T>()?)
}

fn get_api_url(game: &str, date: &str) -> String {
    let full_url: String = format!("{BASE_URL}/{game}/v2/{date}.json");
    full_url
}

fn validate_date(year: u16, month: u16, day: u16) -> String {
    let valid_year = (year > 999);
    let valid_month = (month > 0) & (month < 13);
    let valid_day = (day > 0) & (day < 32);
    let valid_date = valid_year & valid_month & valid_day;
    match valid_date {
        true => format!("{}-{}-{}", year, month, day),
        false => Utc::now().date_naive().to_string(),
    }
}

fn get_desired_date() -> String {
    println!("Please input a date in the format YYYY-MM-DD: ");
    println!("Note: anything invalid will cause the default (today) to be used");

    let year: u16;
    let month: u16;
    let day: u16;
    scan!("{}-{}-{}", year, month, day);
    validate_date(year, month, day)
}

// ---------- GAME LOGIC ---------- //

fn print_wordle() -> Result<(), Box<dyn Error>> {
    let validated_date = get_desired_date();
    let data: WordleResp = fetch_json(&get_api_url("wordle", &validated_date))?;
    println!("Wordle for {}: {}",validated_date, data.solution);
    Ok(())
}

fn print_connections() -> Result<(), Box<dyn Error>> {
    let validated_date = get_desired_date();
    let data: ConnectionsResp = fetch_json(&get_api_url("connections", &validated_date))?;

    println!("Connections for {}: ", validated_date);

    for category in &data.categories {
        println!("Category: {}", category.title);
        for word in &category.cards {
            print!("{} ", word.content);
        }
        println!("\n");
    }
    Ok(())
}

enum Game {
    Wordle,
    Connections,
}

impl Game {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'w' => Some(Game::Wordle),
            'c' => Some(Game::Connections),
            _ => None,
        }
    }
}

fn run_game(game: Game) -> Result<(), Box<dyn Error>> {
    match game {
        Game::Wordle => print_wordle(),
        Game::Connections => print_connections(),
    }
}

// ---------- MAIN ---------- //

fn main() -> Result<(), Box<dyn Error>> {
    println!("Pick a game:");
    println!("  w -> wordle");
    println!("  c -> connections");

    let user_choice: char = read!();

    match Game::from_char(user_choice) {
        Some(game) => run_game(game)?,
        None => println!("Invalid choice."),
    }

    Ok(())
}
