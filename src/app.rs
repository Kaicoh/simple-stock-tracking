use clap::{crate_authors, crate_description, crate_version, App, Arg};

pub fn build<'a, 'b>() -> App<'a, 'b> {
    App::new("Simple Stock Tracking")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("symbols")
                .help("Sets stock symbols")
                .short("s")
                .long("symbols")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("from")
                .help("Sets from date yyyy-mm-dd")
                .short("f")
                .long("from")
                .required(true)
                .takes_value(true),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_symbols() {
        let m = build().get_matches_from(vec!["test", "--symbols", "MSFT", "--from", "2020-03-01"]);
        let vals: Vec<&str> = m.values_of("symbols").unwrap().collect();
        assert_eq!(vals, ["MSFT"])
    }

    #[test]
    fn args_symbols_short() {
        let m = build().get_matches_from(vec!["test", "-s", "MSFT", "--from", "2020-03-01"]);
        let vals: Vec<&str> = m.values_of("symbols").unwrap().collect();
        assert_eq!(vals, ["MSFT"])
    }

    #[test]
    fn args_symbols_has_multiple_values() {
        let m = build().get_matches_from(vec![
            "test",
            "--symbols",
            "MSFT",
            "GOOG",
            "AAPL",
            "--from",
            "2020-03-01",
        ]);
        let vals: Vec<&str> = m.values_of("symbols").unwrap().collect();
        assert_eq!(vals, ["MSFT", "GOOG", "AAPL"])
    }

    #[test]
    fn args_from() {
        let m = build().get_matches_from(vec!["test", "--symbols", "MSFT", "--from", "2020-03-01"]);
        let val = m.value_of("from").unwrap();
        assert_eq!(val, "2020-03-01");
    }

    #[test]
    fn args_from_short() {
        let m = build().get_matches_from(vec!["test", "--symbols", "MSFT", "-f", "2020-03-01"]);
        let val = m.value_of("from").unwrap();
        assert_eq!(val, "2020-03-01");
    }
}
