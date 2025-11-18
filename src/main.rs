use crate::utils::http::{ACCESS_TOKEN, HTTP_CLIENT};
use clap::Parser;

mod env;
mod quran_com;
mod tafsir;
mod translations;
mod utils;

const TRANSLATION_CONFIG_FILE: &str = "translation_config.json";
const TAFSIR_CONFIG_FILE: &str = "tafsir_config.json";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Generate translations HTML (required if both configs exist)
    #[arg(long)]
    translations: bool,

    /// Generate tafsir HTML (required if both configs exist)
    #[arg(long)]
    tafsir: bool,
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

    // Parse CLI arguments
    let args = Args::parse();

    // User must specify either --translations or --tafsir
    if !args.translations && !args.tafsir {
        tracing::error!("Error: Must specify which config to run");
        tracing::info!("Usage:");
        tracing::info!("  cargo run -- --translations   (to generate translations)");
        tracing::info!("  cargo run -- --tafsir        (to generate tafsir)");
        std::process::exit(1);
    }

    // Check if both flags are provided
    if args.translations && args.tafsir {
        tracing::error!("Error: Cannot specify both --translations and --tafsir");
        tracing::info!("Please choose one: either --translations OR --tafsir");
        std::process::exit(1);
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

    // Generate based on which flag was provided
    if args.translations {
        let translation_config = match translations::config::TranslationConfig::from_file(TRANSLATION_CONFIG_FILE) {
            Ok(cfg) => cfg,
            Err(e) => {
                tracing::error!("Error loading {}: {}", TRANSLATION_CONFIG_FILE, e);
                tracing::info!("Please ensure {} exists and is valid", TRANSLATION_CONFIG_FILE);
                std::process::exit(1);
            }
        };

        tracing::info!("Generating translations HTML...");
        if let Err(e) = translations::generate_html::handler(translation_config).await {
            tracing::error!("Error generating translations: {e}");
            std::process::exit(1);
        }
    } else if args.tafsir {
        let tafsir_config = match tafsir::config::TafsirConfig::from_file(TAFSIR_CONFIG_FILE) {
            Ok(cfg) => cfg,
            Err(e) => {
                tracing::error!("Error loading {}: {}", TAFSIR_CONFIG_FILE, e);
                tracing::info!("Please ensure {} exists and is valid", TAFSIR_CONFIG_FILE);
                std::process::exit(1);
            }
        };

        tracing::info!("Generating tafsir HTML...");
        if let Err(e) = tafsir::generate_html::handler(tafsir_config).await {
            tracing::error!("Error generating tafsir: {e}");
            std::process::exit(1);
        }
    }

    tracing::info!("Success");
    Ok(())
}
