use serde::Deserialize;

const WORDLE_URL: &str = "https://www.nytimes.com/svc/wordle/v2/2025-11-15.json";

const CONNECTIONS_URL: &str = "https://www.nytimes.com/svc/connections/v2/2025-11-15.json";

#[derive(Deserialize, Debug)]
struct WordleResp {
    id: u32,
    solution: String,
    print_date: String,
    days_since_launch: u32,
    editor: String
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

#[derive(Deserialize, Debug)]
struct WebResponse {
    connections: ConnectionsResp,
    wordle: WordleResp,
}

//fn get_solution(game: char) -> Result<WebResponse, Box<dyn std::error::Error>> {
fn get_solution(game: char) -> Result<(), Box<dyn std::error::Error>> {
    match game {
        'w' => println!("Wordle game!"),
        'c' => println!("Connections game!"),
        _ => panic!("Invalid game :("),
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    let resp = reqwest::blocking::get(CONNECTIONS_URL)?.json::<ConnectionsResp>();
    match resp {
        Ok(response) => println!("{:#?}", response),
        Err(e) => println!("ERROR: {:#?}", e),
    }
    */

    get_solution('a');
    Ok(())
}
