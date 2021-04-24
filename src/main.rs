use chrono::{Date, Utc};
use simple_stock_tracking as sst;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::{YResponse, YahooError};

fn main() {
    // 1. Get command line parameters
    let matches = sst::app::build().get_matches();

    // 2. Validate parameters
    let symbols: Vec<&str> = match matches.values_of("symbols") {
        Some(data) => data.collect(),
        None => exit("Cannot get symbols"),
    };

    let start = match matches.value_of("from") {
        Some(str_date) => match sst::date::from_string(str_date) {
            Ok(date) => date,
            Err(_) => exit("Date parsing error. Follow pattern yyyy-mm-dd"),
        },
        None => exit("Cannot get from-date"),
    };

    let end = Utc::today();
    if start > end {
        exit("from-date should be a past date");
    }

    // 3. Ingest stock quote data from API
    let responses: Vec<YahooResponses> = symbols
        .iter()
        .map(|&symbol| match YahooResponses::new(symbol, start, end) {
            Ok(responses) => responses,
            Err(err) => {
                let msg = format!("failed to get quotes: {:?}", err);
                exit(&msg);
            }
        })
        .collect();

    print_csv_header();
    for r in responses {
        // 4. Calculate performance indicators
        // 5. print CSV
        if let Err(err) = print_csv_row(r) {
            exit(&err.to_string());
        }
    }
}

fn exit(message: &str) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

fn print_csv_header() {
    println!("period start,symbol,price,change %,min,max,30d avg");
}

fn print_csv_row(responses: YahooResponses) -> Result<(), YahooError> {
    let dt = sst::date_of_last_quote(&responses.period)?;
    let close = sst::close_price(&responses.period)?;
    let percentage = sst::change_rate(&responses.period)?;
    let min = sst::min_price(&responses.period)?;
    let max = sst::max_price(&responses.period)?;
    let avg = sst::average_price(&responses.thirty_days)?;

    println!(
        "{},{},${:.2},{}%,${},${},${:.2}",
        dt.to_rfc3339(),
        responses.symbol,
        close,
        fmt_opt_f64(percentage),
        fmt_opt_f64(min),
        fmt_opt_f64(max),
        avg,
    );

    Ok(())
}

fn fmt_opt_f64(val: Option<f64>) -> String {
    val.map(|v| format!("{:.2}", v)).unwrap_or_default()
}

struct YahooResponses<'a> {
    symbol: &'a str,
    period: YResponse,
    thirty_days: YResponse,
}

impl<'a> YahooResponses<'a> {
    fn new(symbol: &'a str, start: Date<Utc>, end: Date<Utc>) -> Result<Self, yahoo::YahooError> {
        let provider = yahoo::YahooConnector::new();
        let num_days = format!("{}d", end.signed_duration_since(start).num_days());
        let period = provider.get_quote_range(symbol, "1d", &num_days)?;
        let thirty_days = provider.get_quote_range(symbol, "1d", "30d")?;
        Ok(Self {
            symbol,
            period,
            thirty_days,
        })
    }
}
