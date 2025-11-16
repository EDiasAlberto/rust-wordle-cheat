use serde::Deserialize;
use reqwest::blocking::get;
use std::error::Error;
use text_io::read;

const WORDLE_URL: &str =
    "https://www.nytimes.com/svc/wordle/v2/2025-11-15.json";
const CONNECTIONS_URL: &str =
    "https://www.nytimes.com/svc/connections/v2/2025-11-15.json";

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

// ---------- GAME LOGIC ---------- //

fn print_wordle() -> Result<(), Box<dyn Error>> {
    let data: WordleResp = fetch_json(WORDLE_URL)?;
    println!("Today's Wordle: {}", data.solution);
    Ok(())
}

fn print_connections() -> Result<(), Box<dyn Error>> {
    let data: ConnectionsResp = fetch_json(CONNECTIONS_URL)?;

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
