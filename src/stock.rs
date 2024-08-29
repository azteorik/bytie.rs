use crate::context::{Context, Error};
use reqwest::{self, header};
use serde_json::Value;

struct StockInfo {
    price: f64,
    low: f64,
    high: f64,
}

/// Get stock information
#[poise::command(slash_command)]
pub async fn stock(
    ctx: Context<'_>,
    #[description = "Stock symbol"] symbol: String,
) -> Result<(), Error> {
    match fetch_stock_info(&symbol).await {
        Ok(info) => {
            ctx.say(format!(
                "{}:\nPrice: ${:.2}\nDaily Low: ${:.2}\nDaily High: ${:.2}",
                symbol, info.price, info.low, info.high
            ))
            .await?;
        }
        Err(err) => {
            ctx.say(format!("Failed to fetch stock information: {}", err))
                .await?;
        }
    }

    Ok(())
}

/// Fetch stock information from the Yahoo Finance API
async fn fetch_stock_info(symbol: &str) -> Result<StockInfo, Error> {
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}",
        symbol
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(header::USER_AGENT, "curl/7.68.0")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let response_text = response.text().await?;
    let data: Value = serde_json::from_str(&response_text)?;

    let quote = &data["chart"]["result"][0]["meta"];

    let price = quote["regularMarketPrice"].as_f64().unwrap_or(0.0);
    let low = quote["regularMarketDayLow"].as_f64().unwrap_or(0.0);
    let high = quote["regularMarketDayHigh"].as_f64().unwrap_or(0.0);

    Ok(StockInfo { price, low, high })
}
