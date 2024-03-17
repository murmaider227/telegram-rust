use crate::commands::price_all::price_all_command;
use crate::db::DatabaseManager;
use chrono::{Local, TimeZone};
use log::{debug, error};
use mongodb::bson::doc;
use teloxide::prelude::*;
use tokio::time::{self, Duration};

async fn send_all_currency(cfg: DatabaseManager, bot: Bot, _text: String) {
    let filter = Some(doc! {
        "notification": true,
        "currency.0": { "$exists": true }
    });
    let users_query = cfg.get_all_users(filter).await;

    let users = match users_query {
        Ok(users) => users,
        Err(err) => {
            debug!("Error getting users: {}", err);
            return;
        }
    };

    tokio::spawn(async move {
        for user in users {
            let currency_text = price_all_command(user.clone()).await;

            if let Err(err) = bot
                .send_message(UserId(user.user_id as u64), currency_text)
                .await
            {
                debug!("Error sending photo: {}", err);
            }
            let mess = "\n Для отключения уведомлений напишите /notify";
            if let Err(err) = bot.send_message(UserId(user.user_id as u64), mess).await {
                debug!("Error sending photo: {}", err);
            }
        }
    });
}

/// Send all users a message
pub async fn send_all_command(cfg: DatabaseManager, bot: Bot, text: String) {
    let users = cfg.get_all_users(None).await.unwrap();

    for user in users {
        if let Err(err) = bot
            .send_message(UserId(user.user_id as u64), text.clone())
            .await
        {
            error!("Error sending photo: {}", err);
        }
    }
}

pub async fn send_message_at_specific_time(bot: Bot, cfg: DatabaseManager) {
    tokio::spawn(async move {
        // Установите время, в которое необходимо отправить сообщение (например, 00:39 AM)
        let target_hour = 11;
        let target_minute = 00;

        loop {
            let now = Local::now();

            if let Some(naive_target_time) =
                now.date_naive().and_hms_opt(target_hour, target_minute, 0)
            {
                let target_time = Local
                    .from_local_datetime(&naive_target_time)
                    .single()
                    .unwrap();

                // Вычислите продолжительность до наступления target_time
                let duration_until_target = if now < target_time {
                    target_time - now
                } else {
                    // Если target_time уже прошло, установите его на следующий день
                    target_time + chrono::Duration::try_days(1).unwrap() - now
                };

                // Ждите, пока не наступит нужное время
                time::sleep(duration_until_target.to_std().unwrap()).await;

                send_all_currency(cfg.clone(), bot.clone(), "Hello".to_string()).await;

                // Подождите некоторое время, чтобы избежать повторной отправки сообщения, если цикл выполняется слишком быстро
                time::sleep(Duration::from_secs(60)).await;
            } else {
                log::error!("Некорректное время для отправки сообщений.");
                break;
            }
        }
    });
}
