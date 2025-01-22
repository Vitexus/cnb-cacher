// src/utils/fetch.rs

use reqwest::blocking::get;
use reqwest::Error;

pub fn fetch_exchange_rates(url: &str) -> Result<String, Error> {
    let response = get(url)?.text()?;
    Ok(response)
}

pub fn parse_csv_data(csv_data: &str) -> Vec<(String, String, String, String, f64)> {
    let mut rates = Vec::new();
    let lines = csv_data.lines().skip(2); // Skip the header lines

    for line in lines {
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() == 5 {
            let country = fields[0].to_string();
            let currency = fields[1].to_string();
            let amount = fields[2].to_string();
            let code = fields[3].to_string();
            let rate = fields[4].replace(',', ".").parse::<f64>().unwrap_or(0.0);
            rates.push((country, currency, amount, code, rate));
        }
    }
    rates
}