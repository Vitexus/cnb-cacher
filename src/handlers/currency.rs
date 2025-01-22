
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use actix_web::{web, HttpResponse};

pub async fn get_today_currency(path: web::Path<String>) -> HttpResponse {
    let currency_code = path.into_inner();
    // Fetch and return today's currency data as JSON
    HttpResponse::Ok().json(format!("Today's data for {}", currency_code))
}

pub async fn get_yesterday_currency(path: web::Path<String>) -> HttpResponse {
    let currency_code = path.into_inner();
    // Fetch and return yesterday's currency data as JSON
    HttpResponse::Ok().json(format!("Yesterday's data for {}", currency_code))
}

async fn fetch_currency_data() -> Result<HashMap<String, f64>, Box<dyn Error>> {
    // Logic to fetch today's currency data
    // This function should call the utility function to get the data from the URL
}

async fn fetch_currency_data_yesterday() -> Result<HashMap<String, f64>, Box<dyn Error>> {
    // Logic to fetch yesterday's currency data
    // This function should call the utility function to get the data from the URL
}

fn format_currency_data(data: &HashMap<String, f64>, currency_code: &str) -> Result<String, Box<dyn Error>> {
    if let Some(&rate) = data.get(currency_code) {
        let json_response = json!({
            "currency": currency_code,
            "rate": rate,
        });
        Ok(json_response.to_string())
    } else {
        Err("Currency code not found".into())
    }
}