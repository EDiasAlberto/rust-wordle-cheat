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

#[derive(Debug)]
enum EitherResp<A, B> {
    Left(Result<A, reqwest::Error>),
    Right(Result<B, reqwest::Error>),
}

fn get_solution(game: char) -> Result<EitherResp<WordleResp, ConnectionsResp>, Box<dyn std::error::Error>> {
    let resp = match game {
        'w' => EitherResp::Left(reqwest::blocking::get(WORDLE_URL)?.json::<WordleResp>()),
        'c' => EitherResp::Right(reqwest::blocking::get(CONNECTIONS_URL)?.json::<ConnectionsResp>()),
        _ => panic!("Non-valid game character"),
    };
    Ok(resp) 

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let site_resp = get_solution('w');
    match site_resp {
        Ok(EitherResp::Left(conn_resp)) => println!("Got solution: {:#?}", conn_resp),
        Ok(EitherResp::Right(wordle_resp)) => println!("Got solution: {:#?}", wordle_resp),
        Err(_) => println!("Error!!"),
    }
    Ok(())
}
