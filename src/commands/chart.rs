use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use plotters::prelude::*;
use reqwest::header;
use serde_json::Value;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use tokio::spawn;

pub async fn chart_command(bot: Bot, msg: Message, currency: String) {
    // let res = send_chart(bot.clone(), msg.clone(), currency.clone()).await;
    //
    // // print message if res return error which contain text fetching price for
    // if res.is_err() {
    //     let err = res.unwrap_err();
    //     let err_str = err.to_string();
    //     if err_str.contains("Error fetching price for") {
    //         spawn(async move {
    //             // Дожидаемся завершения отправки фото
    //             if let Err(err) = bot
    //                 .send_message(msg.chat.id, format!("price for {} not found", currency))
    //                 .await
    //             {
    //                 println!("Error sending photo: {}", err);
    //             }
    //         });
    //     }

    spawn(async move {
        // Дожидаемся завершения отправки фото
        if let Err(err) = send_chart(bot.clone(), msg.clone(), currency.clone()).await {
            println!("Error sending photo: {}", err);
        }
        if let Err(err) = bot
            .send_message(msg.chat.id, format!("price for {} not found", currency))
            .await
        {
            println!("Error sending photo: {}", err);
        }
    });
}

async fn get_chart(currency: &String) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
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

    let mut data = vec![];
    for kline in response_json {
        let close_value = kline[4].as_str().unwrap();
        let close = close_value.parse::<f64>().unwrap();
        data.push(close);
    }

    Ok(data.iter().map(|&d| d as u32).collect())
}

async fn send_chart(
    bot: Bot,
    msg: Message,
    currency: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_all = get_chart(&currency).await?;
    let data: Vec<u32> = data_all.iter().rev().take(24).copied().rev().collect(); // get last 30 days
                                                                                  //println!("{:?}", data);
    let filename = format!("chart_{}.png", chrono::Utc::now().timestamp());
    let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let timezone: Tz = "Europe/Kiev".parse().unwrap();

    let last_day = chrono::Utc::now() - chrono::Duration::days(1);

    let mut x_labels = vec![];

    for (i, _) in data.iter().enumerate() {
        if i % 2 == 0 {
            let timestamp = last_day.timestamp() as u32 + i as u32 * 60 * 60;
            let naive_dt = chrono::NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap();
            let localized_dt: DateTime<Tz> = timezone.from_utc_datetime(&naive_dt);
            let hour = localized_dt.format("%H").to_string();
            x_labels.push((i as u32, hour + ":00"));
        } else {
            x_labels.push((i as u32, "".to_string()));
        }
    }

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
            data.iter().min().unwrap() - 10..data.iter().max().unwrap() + 10,
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

    let filename_clone = filename.clone();

    spawn(async move {
        // Дожидаемся завершения отправки фото
        if let Err(err) = send_chart_file(&bot, &msg, &filename_clone).await {
            println!("Error sending photo: {}", err);
        }
    });

    Ok(())
}

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
