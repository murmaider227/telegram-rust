use crate::commands::send_all::{send_all_command, send_message_at_specific_time};
use crate::commands::{
    chart::chart_command,
    currency::{add_currency_command, remove_currency_command},
    price::price_command,
    price_all::price_all_command,
    start::start_command,
};
use crate::db::DatabaseManager;
use teloxide::{prelude::*, types::Update, utils::command::BotCommands};

pub async fn register_currency_handlers(bot: Bot, db: DatabaseManager) {
    let handler = Update::filter_message()
        // You can use branching to define multiple ways in which an update will be handled. If the
        // first branch fails, an update will be passed to the second branch, and so on.
        .branch(
            dptree::entry()
                // Filter commands: the next handlers will receive a parsed `SimpleCommand`.
                .filter_command::<SimpleCommand>()
                // If a command parsing fails, this handler will not be executed.
                .endpoint(simple_commands_handler),
        )
        .branch(
            dptree::entry()
                .filter_command::<AdminCommand>()
                .endpoint(admin_commands_handler),
        );

    bot.set_my_commands(SimpleCommand::bot_commands())
        .await
        .expect("failed setting commands");

    send_message_at_specific_time(bot.clone(), db.clone()).await;

    Dispatcher::builder(bot, handler)
        // Here you specify initial dependencies that all handlers will receive; they can be
        // database connections, configurations, and other auxiliary arguments. It is similar to
        // `actix_web::Extensions`.
        .dependencies(dptree::deps![db])
        // If no handler succeeded to handle an update, this closure will be called.
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If the dispatcher fails for some reason, execute this handler.
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Simple commands")]
enum SimpleCommand {
    #[command(description = "shows this message.")]
    Help,
    #[command(description = "register user")]
    Start,
    #[command(description = "shows your ID.")]
    MyId,
    #[command(description = "get chart")]
    Chart(String),
    #[command(description = "handle a price", parse_with = "split")]
    Price { value: f64, currency: String },
    #[command(description = "shows your data.")]
    Me,
    #[command(description = "add currency")]
    AddCurrency(String),
    #[command(description = "remove currency")]
    RemoveCurrency(String),
    #[command(description = "print all user currencies")]
    PriceAll,
}

async fn simple_commands_handler(
    cfg: DatabaseManager,
    bot: Bot,
    // me: teloxide::types::Me,
    msg: Message,
    cmd: SimpleCommand,
) -> Result<(), teloxide::RequestError> {
    match cmd {
        SimpleCommand::Help => {
            bot.send_message(msg.chat.id, SimpleCommand::descriptions().to_string())
                .await?;
        }
        SimpleCommand::MyId => {
            bot.send_message(msg.chat.id, format!("{}", msg.from().unwrap().id))
                .await?;
        }
        SimpleCommand::Chart(currency) => {
            chart_command(bot.clone(), msg.clone(), currency).await;
        }
        SimpleCommand::Price { value, currency } => {
            let result = price_command(value, currency).await;
            bot.send_message(msg.chat.id, result).await?;
        }
        SimpleCommand::Start => {
            let result = start_command(msg.clone(), cfg.clone()).await;
            bot.send_message(msg.chat.id, result).await?;
        }
        SimpleCommand::Me => {
            let result = cfg
                .get_user(msg.from().unwrap().id.0 as i64)
                .await
                .expect("Error get user");
            bot.send_message(msg.chat.id, format!("{:?}", result))
                .await?;
        }
        SimpleCommand::AddCurrency(currency) => {
            let res =
                add_currency_command(msg.from().unwrap().id.0 as i64, currency, cfg.clone()).await;
            bot.send_message(msg.chat.id, res).await?;
        }
        SimpleCommand::RemoveCurrency(currency) => {
            let res =
                remove_currency_command(msg.from().unwrap().id.0 as i64, currency, cfg.clone())
                    .await;
            bot.send_message(msg.chat.id, res).await?;
        }
        SimpleCommand::PriceAll => {
            let user = cfg
                .get_user(msg.from().unwrap().id.0 as i64)
                .await
                .expect("Error get user");
            let result = price_all_command(user).await;
            bot.send_message(msg.chat.id, result).await?;
        }
    };

    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Admin commands")]
enum AdminCommand {
    #[command(description = "shows this message.")]
    Sendall(String),
}

async fn admin_commands_handler(
    cfg: DatabaseManager,
    bot: Bot,
    // me: teloxide::types::Me,
    // msg: Message,
    cmd: AdminCommand,
) -> Result<(), teloxide::RequestError> {
    match cmd {
        AdminCommand::Sendall(text) => {
            // let users = cfg.get_all_users().await.unwrap();
            // for user in users {
            //     bot.send_message(UserId(user.user_id as u64), text.clone())
            //         .await?;
            // }
            send_all_command(cfg, bot.clone(), text).await;
        }
    }
    Ok(())
}
