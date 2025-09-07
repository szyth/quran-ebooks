use clap::{ArgGroup, CommandFactory, Parser};

use crate::utils::http::{ACCESS_TOKEN, HTTP_CLIENT};
mod env;
mod quran_com;
mod tafsir;
mod translations;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args([ "translations", "tafsir"])
))]

struct Args {
    /// Get Arabic and Translations
    #[arg(long)]
    translations: bool,

    /// Get Tafsir
    #[arg(long)]
    tafsir: bool,

    /// Start surah (required with --translations), 1 to 114
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=114))]
    start_surah: Option<u8>,

    /// End surah (optional with --translations), 1 to 114
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=114))]
    end_surah: Option<u8>,
}

async fn logs() {
    let level: tracing::Level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_owned())
        .parse()
        .unwrap_or(tracing::Level::INFO);

    tracing_subscriber::fmt().with_max_level(level).init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    logs().await;
    if dotenv::from_filename(".env").is_err() {
        tracing::error!("Error: .env file not found. refer `sampleenv`for format.");
        std::process::exit(1)
    }
    if env::check_envs().is_none() {
        tracing::error!("Error: ENV variable expected.");
        std::process::exit(1)
    }

    // Show help if no args passed
    if std::env::args().len() == 1 {
        Args::command().print_help()?;
        println!();
        std::process::exit(0);
    }

    // build global reqWest HTTP client
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    HTTP_CLIENT
        .set(client)
        .expect("ERROR: failed to create http client");

    // Login and store token globally
    let token = quran_com::apis::oauth2_token::handler().await;
    if token.is_err() {
        tracing::error!("Error: Failed to login.");
        std::process::exit(1)
    }
    let token = token.unwrap(); // safe to use unwrap
    ACCESS_TOKEN
        .set(token.clone())
        .expect("ERROR: failed to store access token");

    // Parse Args
    let args = Args::parse();

    // Translations flow
    if args.translations {
        if args.start_surah.is_none() {
            tracing::error!("Error: --translations requires --start-surah <START_SURAH>");
            std::process::exit(1);
        }

        if let (Some(start), Some(end)) = (args.start_surah, args.end_surah) {
            if start > end {
                tracing::error!("Error: --start-surah must be less than or equal to --end-surah");
                std::process::exit(1);
            }
        }

        let start_surah = args.start_surah.unwrap(); // safe to use unwrap
        let end_surah = args.end_surah.unwrap_or(start_surah);
        if let Err(e) = translations::generate_html::handler(start_surah, end_surah).await {
            tracing::error!("Error: {e}");
            std::process::exit(1);
        }
    }
    // Tafsir flow
    if args.tafsir {
        if args.start_surah.is_none() {
            tracing::error!("Error: --tafsir requires --start-surah <START_SURAH>");
            std::process::exit(1);
        }

        if let (Some(start), Some(end)) = (args.start_surah, args.end_surah) {
            if start > end {
                tracing::error!("Error: --start-surah must be less than or equal to --end-surah");
                std::process::exit(1);
            }
        }

        let start_surah = args.start_surah.unwrap(); // safe to use unwrap
        let end_surah = args.end_surah.unwrap_or(start_surah);
        if let Err(e) = tafsir::generate_html::handler(start_surah, end_surah).await {
            tracing::error!("Error: {e}");
            std::process::exit(1);
        }
    }

    tracing::info!("Success");
    Ok(())
}
