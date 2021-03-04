use chrono::{Utc, DateTime, NaiveDateTime};
use simple_stock_tracking as sst;
use yahoo_finance_api as yahoo;

fn main() {
    // 1. Get command line parameters
    let matches = sst::app::build().get_matches();

    // 2. Validate parameters
    let symbols: Vec<&str> = match matches.values_of("symbols") {
        Some(data) => data.collect(),
        None => exit("Cannot get symbols"),
    };

    let start_date = match matches.value_of("from") {
        Some(str_date) => match sst::date::from_string(str_date) {
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
    let provider = yahoo::YahooConnector::new();
    let start = start_date.and_hms_milli(0, 0, 0, 0);
    let end = end_date.and_hms_milli(23, 59, 59, 999);
    let indicators: Vec<Indicator> = symbols
        .iter()
        .map(
            |&symbol| match provider.get_quote_history(symbol, start, end) {
                Ok(res) => match (res.last_quote(), res.quotes()) {
                    (Ok(last_quote), Ok(quotes)) => Indicator::new(symbol, last_quote, quotes),
                    (Err(err), _) => {
                        let msg = format!("failed to get last quote: {:?}", err);
                        exit(&msg);
                    }
                    (_, Err(err)) => {
                        let msg = format!("failed to get quotes: {:?}", err);
                        exit(&msg);
                    }
                },
                Err(err) => {
                    let msg = format!("failed to get quote history: {:?}", err);
                    exit(&msg);
                }
            },
        )
        .collect();

    println!("period start,symbol,price,change %,min,max");
    for i in indicators {
        // 4. Calculate performance indicators
        // 5. print CSV
        i.print();
    }
}

fn exit(message: &str) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

fn fmt_opt_f64(val: Option<f64>) -> String {
    val.map(|v| format!("{:.2}", v)).unwrap_or_default()
}

struct Indicator<'a> {
    symbol: &'a str,
    timestamp: u64, // last quote timestamp
    price: f64, // last quote price
    adjcloses: Vec<f64>,
}

impl<'a> Indicator<'a> {
    fn new(symbol: &'a str, last_quote: yahoo::Quote, quotes: Vec<yahoo::Quote>) -> Self {
        Self {
            symbol,
            timestamp: last_quote.timestamp,
            price: last_quote.adjclose,
            adjcloses: quotes.iter().map(|q| q.adjclose).collect(),
        }
    }

    fn min(&self) -> Option<f64> {
        sst::min(&self.adjcloses)
    }

    fn max(&self) -> Option<f64> {
        sst::max(&self.adjcloses)
    }

    fn percentage(&self) -> Option<f64> {
        sst::price_diff(&self.adjcloses).map(|(v, _)| v)
    }

    fn print(&self) {
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.timestamp as i64, 0), Utc);
        println!(
            "{},{},${:.2},{}%,${},${}",
            dt.to_rfc3339(),
            self.symbol,
            self.price,
            fmt_opt_f64(self.percentage()),
            fmt_opt_f64(self.min()),
            fmt_opt_f64(self.max()),
        );
    }
}
