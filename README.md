# NYT Puzzle Fetcher

A small Rust command-line tool for retrieving **Wordle** and **Connections** answers from the New York Times puzzle API for any specified date.

## Features
- Fetches Wordle solutions by date
- Fetches Connections categories and cards by date
- Validates user-provided dates (defaults to today if invalid)
- Simple, synchronous implementation using `reqwest` and `serde`

## Usage
Run the program with:

```bash
cargo run
```

You will be prompted to choose a game:

```
w -> wordle
c -> connections
```

Then enter a date in `YYYY-MM-DD` format.  
If the input is invalid, the current date is used automatically.

## Dependencies
- `serde`
- `reqwest` (blocking)
- `chrono`
- `text_io`

## Notes
This project interacts with public NYT puzzle endpoints.  
Some dates may not have available data depending on publication history.

## License
MIT License
