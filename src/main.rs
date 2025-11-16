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

fn handle_wordle_cheat() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(WORDLE_URL)?.json::<WordleResp>();
    match resp {
        Ok(ansr) => println!("Answer for today's wordle! {:#?}", ansr.solution),
        Err(e) => println!("Got error!"),
    }

    Ok(())
}

fn render_connections_ansr(answer: ConnectionsResp) {
    for category in answer.categories {
        println!("Category Title: {:#?}", category.title);
        for word in category.cards {
            print!("{:#?} ", word.content);
        }
        println!("\n");
    }
}

fn handle_connections_cheat() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(CONNECTIONS_URL)?.json::<ConnectionsResp>();

    match resp {
        Ok(ansr) => render_connections_ansr(ansr),
        Err(e) => println!("Got error!"),
    }

    Ok(())
}

fn get_solution(game: char) -> Result<(), Box<dyn std::error::Error>> {
    let resp = match game {
        'w' => handle_wordle_cheat(),
        'c' => handle_connections_cheat(),
        _ => panic!("Non-valid game character"),
    };
    Ok(())

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _ = get_solution('c');
    Ok(())
}
