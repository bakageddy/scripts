use reqwest::Client;

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

    let res = Client::new()
        .get(format!(
            "{}?q={}&lang={}&units={}&appid={}",
            BASE_URL, CITY, LANG, STD, apikey
        ))
        .send()
        .await;

    let body :serde_json::Value = match res.unwrap().json().await {
        Ok(body) => body,
        Err(_) => {eprintln!("Failed to parse to json"); std::process::exit(1);},
    };

    let body = serde_json::json!(body);

    let degrees = body["main"]["temp"].as_f64().unwrap() as i32;

    println!("{}C", degrees);
    Ok(())
}
