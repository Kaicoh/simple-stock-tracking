use chrono::{Utc, DateTime, NaiveDateTime};
use simple_stock_tracking as sst;
use simple_stock_tracking::yahoo::Indicator;

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
    let indicators: Vec<Indicator> = symbols
        .iter()
        .map(|&symbol| {
            match Indicator::new(symbol, start, end) {
                Ok(indicator) => indicator,
                Err(err) => {
                    let msg = format!("failed to get quotes: {:?}", err);
                    exit(&msg);
                },
            }
        })
        .collect();

    print_csv_header();
    for i in indicators {
        // 4. Calculate performance indicators
        // 5. print CSV
        print_csv_row(i);
    }
}

fn exit(message: &str) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

fn print_csv_header() {
    println!("period start,symbol,price,change %,min,max,30d avg");
}

fn print_csv_row(indicator: Indicator) {
    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(indicator.timestamp() as i64, 0), Utc);
    let percentage = sst::price_diff(indicator.prices_in_period()).map(|(v, _)| v);
    let min = sst::min(indicator.prices_in_period());
    let max = sst::max(indicator.prices_in_period());
    let avg = sst::n_window_sma(30, indicator.prices_last_30_days())
        .unwrap()
        .last()
        .cloned();

    println!(
        "{},{},${:.2},{}%,${},${},${}",
        dt.to_rfc3339(),
        indicator.symbol,
        indicator.price(),
        fmt_opt_f64(percentage),
        fmt_opt_f64(min),
        fmt_opt_f64(max),
        fmt_opt_f64(avg),
    );
}

fn fmt_opt_f64(val: Option<f64>) -> String {
    val.map(|v| format!("{:.2}", v)).unwrap_or_default()
}
