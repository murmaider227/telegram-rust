use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use plotters::prelude::*;
use reqwest::header;
use serde_json::Value;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use tokio::spawn;

/// /chart command handler
/// Sends a chart of the specified currency
pub async fn chart_command(bot: Bot, msg: Message, currency: String) {
    spawn(async move {
        // Дожидаемся завершения отправки фото
        if let Err(err) = send_chart(bot.clone(), msg.clone(), currency.clone()).await {
            log::error!("Error sending photo: {}", err);
        }
    });
}

/// get info about the specified currency from binance api
async fn get_chart(currency: &String) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let symbol = currency.to_uppercase() + "USDT";
    let interval = "1h";
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}",
        symbol, interval
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(header::USER_AGENT, "rust")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!(
            "Error fetching price for {}: Status code {}",
            currency,
            response.status().as_u16()
        )
        .into());
    }

    let response_json = response.json::<Vec<Vec<Value>>>().await?;

    let mut data = Vec::with_capacity(200000);
    for kline in response_json {
        let close_value = kline[4].as_str().ok_or("Failed to convert to str")?;
        let close = close_value.parse::<f64>()?;
        data.push(close);
    }

    //Ok(data.iter().map(|&d| d as u32).collect())
    Ok(data)
}

#[tokio::test]
async fn test_get_chart() {
    let data = get_chart(&"btc".to_string()).await.unwrap();
    assert!(data.len() > 1);
}

/// Send a chart of the specified currency
async fn send_chart(
    bot: Bot,
    msg: Message,
    currency: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_all = get_chart(&currency).await?;
    let data: Vec<f64> = data_all.iter().rev().take(24).copied().rev().collect(); // get last 30 days
    let filename = format!("chart_{}.png", chrono::Utc::now().timestamp());

    let timezone: Tz = "Europe/Kiev".parse()?;

    let last_day = chrono::Utc::now() - chrono::Duration::days(1);

    let mut x_labels: Vec<(u32, String)> = vec![];

    for (i, _) in data.iter().enumerate() {
        if i % 2 == 0 {
            let timestamp = last_day.timestamp() as u32 + i as u32 * 60 * 60;
            let naive_dt = chrono::NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
                .ok_or("Failed to convert timestamp")?;
            let localized_dt: DateTime<Tz> = timezone.from_utc_datetime(&naive_dt);
            let hour = localized_dt.format("%H").to_string();
            x_labels.push((i as u32, hour + ":00"));
        } else {
            x_labels.push((i as u32, "".to_string()));
        }
    }

    build_chart(x_labels, currency, data, filename.clone()).await?;

    let filename_clone = filename.clone();

    spawn(async move {
        // Дожидаемся завершения отправки фото
        if let Err(err) = send_chart_file(&bot, &msg, &filename_clone).await {
            println!("Error sending photo: {}", err);
        }
    });

    Ok(())
}

/// Builds a chart and saves it to a file
async fn build_chart(
    x_labels: Vec<(u32, String)>,
    currency: String,
    data: Vec<f64>,
    filename: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Price Chart for {} in 24 hours", currency),
            ("sans-serif", 30).into_font(),
        )
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .build_cartesian_2d(
            0..data.len() as u32,
            (*data
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .ok_or("Failed to find the minimum value")?)
                ..(*data
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .ok_or("Failed to find the maximum value")?),
        )?;

    chart
        .configure_mesh()
        .x_labels(30)
        .x_label_formatter(&|x| {
            x_labels
                .get(*x as usize)
                .map(|(_, label)| label.clone())
                .unwrap_or_default()
        })
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..).zip(data.iter()).map(|(x, y)| (x, *y)),
        &BLUE,
    ))?;
    Ok(())
}

/// Sends a chart file to the user
async fn send_chart_file(
    bot: &Bot,
    msg: &Message,
    filename: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let chart_file = InputFile::file(filename);

    let send_photo_future = bot.send_photo(msg.chat.id, chart_file);

    // let filename_clone = filename.clone();

    send_photo_future.await?;
    std::fs::remove_file(filename)?;

    Ok(())
}
