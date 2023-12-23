use dotenv::dotenv;
use reqwest::{Error, StatusCode};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Data,
    meta: MetaData,
}

#[derive(Deserialize, Debug)]
struct Data {
    #[serde(flatten)]
    currencies: std::collections::HashMap<String, CurrencyData>,
}

#[derive(Deserialize, Debug)]
pub struct CurrencyData {
    pub code: String,
    pub value: f64,
}

#[derive(Deserialize, Debug)]
struct MetaData {
    last_updated_at: String,
}

impl CurrencyData {
    pub async fn get_conversion(
        currency: String,
        base_currency: String,
    ) -> Result<CurrencyData, Error> {
        dotenv().ok();
        let request_url = format!(
          "https://api.currencyapi.com/v3/latest?apikey=cur_live_3WA0zah3Lbz6WkhyClY8S1hUqbhlDPmVxKX2m6NZ&currencies={currency}&base_currency={base_currency}",
          currency = currency,
          base_currency = base_currency
      );
        let response = reqwest::get(&request_url).await?;
        if response.status().is_success() {
            let api_response: ApiResponse = response.json().await?;
            if let Some(currency_data) = api_response.data.currencies.get(&currency) {
                // Return the new CurrencyData instance
                return Ok(CurrencyData {
                    code: currency_data.code.clone(),
                    value: currency_data.value,
                })
            } else {
                println!("Currency data not found for {}", currency);
            }
            println!("Last Updated At: {}", api_response.meta.last_updated_at);
        } else {
            println!("Request failed with status: {}", response.status());
        }
        Ok(CurrencyData {
            code: String::new(),
            value: 0.0,
        }) // You might want to adjust the default values based on your needs
    }
}
