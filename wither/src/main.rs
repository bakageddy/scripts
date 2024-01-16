use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct WeatherApiResponse {
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";
    const STD: &str = "metric";
    const LANG: &str = "en";
    const CITY: &str = "Chennai";
    let apikey = match std::env::var("OPENWEATHER_KEY") {
        Ok(key) => key,
        Err(_) => {eprintln!("Failed to fetch key"); std::process::exit(1)},
    };

    let mut query_params = HashMap::new();
    query_params.insert("lang", LANG);
    query_params.insert("units", STD);
    query_params.insert("appid", apikey);
    query_params.insert("q", CITY);

    let res = Client::new()
        .get(BASE_URL)
        .query(&query_params)
        .send()
        .await;

    if let Ok(res) = res {
        if let Ok(body) = res.json::<WeatherApiResponse>().await {
        }
    }

    let body :serde_json::Value = match res.unwrap().json().await {
        Ok(body) => body,
        Err(_) => {eprintln!("Failed to parse to json"); std::process::exit(1);},
    };

    let body = serde_json::json!(body);

    let degrees = body["main"]["temp"].as_f64().unwrap() as i32;

    println!("{}C", degrees);
    Ok(())
}
