use chrono::Utc;
use simple_stock_tracking::{app, date};

fn main() {
    // 1. Get command line parameters
    let matches = app::build().get_matches();

    // 2. Validate parameters
    let symbols: Vec<&str> = match matches.values_of("symbols") {
        Some(data) => data.collect(),
        None => exit("Cannot get symbols"),
    };

    let start_date = match matches.value_of("from") {
        Some(str_date) => match date::from_string(str_date) {
            Ok(d) => d,
            Err(_) => exit("Date parsing error. follow pattern yyyy-mm-dd"),
        },
        None => exit("Cannot get from-date"),
    };

    let end_date = Utc::today();
    if start_date > end_date {
        exit("from-date should be past date");
    }

    // 3. Ingest stock quote data from API
    // 4. Calculate performance indicators
    // 5. print CSV
}

fn exit(message: &str) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}
