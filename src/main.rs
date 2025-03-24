#[cfg(feature = "cli")]
use futures_util::StreamExt;
use poe_api::models::FileInput;
#[cfg(feature = "cli")]
use poe_api::{
    api::PoeApi,
    models::{SendMessageData, Token},
};

#[cfg(feature = "cli")]
use clap::Parser;

#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
#[clap(
    name = "poe-cli",
    about = "A CLI for interacting with Poe",
    long_about = "This command-line interface allows you to send queries to the Poe.com."
)]
struct Args {
    /// The query to send to the bot.
    #[clap(value_name = "QUERY")]
    query: String,

    /// Content value of cookie p-b.
    #[clap(long, value_name = "P_B", env = "POE_P_B", hide_env_values = true)]
    p_b: String,

    /// Content value of cookie p-lat.
    #[clap(long, value_name = "P_LAT", env = "POE_P_LAT", hide_env_values = true)]
    p_lat: String,

    /// Unique code for each poe.com account.
    #[clap(long, value_name = "FROMKEY")]
    fromkey: Option<String>,

    /// Specify one of the bot names on poe.com.
    #[clap(short = 'b', long, value_name = "BOT_NAME", default_value = "")]
    bot_handle: String,

    /// Names of the media files to upload.
    #[clap(short, long, value_name = "FILE")]
    files: Vec<String>,

    /// Existing chat thread.
    #[clap(short, long, value_name = "ID")]
    chat_id: Option<i64>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "cli")]
    {
        stderrlog::StdErrLog::new()
            .show_level(true)
            .show_module_names(true)
            .verbosity(log::LevelFilter::Info)
            .init()?;

        let opt = Args::parse();

        let mut api = PoeApi::new(Token {
            p_b: &opt.p_b,
            p_lat: &opt.p_lat,
            formkey: opt.fromkey.as_deref(),
        })
        .await?;

        let mut message = api
            .send_message(SendMessageData {
                bot_handle: &opt.bot_handle,
                message: &opt.query,
                chat_id: opt.chat_id,
                files: opt
                    .files
                    .iter()
                    .map(|v| {
                        if v.starts_with("http") {
                            FileInput::Url(v)
                        } else {
                            FileInput::Local(v.into())
                        }
                    })
                    .collect(),
            })
            .await?;

        while let Some(chunk) = message.next().await {
            chunk.print()?;
        }

        if !message.text().await.is_empty() {
            eprintln!("{}", "-".repeat(25));
            eprintln!("- Chat title: {}", message.title());
            eprintln!("- Chat id: {}", message.chat().inner.chat_id);
            eprintln!("- Share url: {}", message.share().await?);
            let preview_apps = message.get_list_preview_app().await?;
            if !preview_apps.is_empty() {
                eprintln!("- Preview App List:");
                for (index, url) in preview_apps.iter().enumerate() {
                    eprintln!("  - App-{}: {}", index, url);
                }
            }
            eprintln!(
                "- Total message cost point: {}",
                message.total_cost_points().await?
            );
            eprintln!(
                "- Remaining point: {}",
                api.get_settings().await?.message_point_balance()
            );
        }
    }
    Ok(())
}
