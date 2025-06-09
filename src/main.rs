use clap::{ArgGroup, CommandFactory, Parser};

use crate::{tafsir::tafsir_html_generator, translations::translations_html_generator};
mod env;
mod quran_com;
mod tafsir;
mod translations;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args([ "login", "translations", "tafsir"])
))]

struct Args {
    /// Login to quran.com API and get AccessToken
    #[arg(long)]
    login: bool,

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
        eprintln!("Error: .env file not found. refer `sampleenv`for format.");
        std::process::exit(1)
    }
    if env::check_envs().is_none() {
        eprintln!("Error: ENV variable expected.");
        std::process::exit(1)
    }

    // Show help if no args passed
    if std::env::args().len() == 1 {
        Args::command().print_help()?;
        println!();
        std::process::exit(0);
    }

    // Parse Args
    let args = Args::parse();

    // Login flow
    if args.login {
        let _ = quran_com::oauth2_token::handler().await;
    }

    // Translations flow
    if args.translations {
        if env::access_token().unwrap().is_empty() {
            // safe to use unwrap
            eprintln!("Error: Access Token missing in .env");
            std::process::exit(1)
        }

        if args.start_surah.is_none() {
            eprintln!("Error: --translations requires --start-surah <START_SURAH>");
            std::process::exit(1);
        }

        if let (Some(start), Some(end)) = (args.start_surah, args.end_surah) {
            if start > end {
                eprintln!("Error: --start-surah must be less than or equal to --end-surah");
                std::process::exit(1);
            }
        }

        let start_surah = args.start_surah.unwrap(); // safe to use unwrap
        let end_surah = args.end_surah.unwrap_or(start_surah);
        let _ = translations_html_generator::handler(start_surah, end_surah).await;
    }
    // Tafsir flow
    if args.tafsir {
        if env::access_token().unwrap().is_empty() {
            // safe to use unwrap
            eprintln!("Error: Access Token missing in .env");
            std::process::exit(1)
        }

        if args.start_surah.is_none() {
            eprintln!("Error: --tafsir requires --start-surah <START_SURAH>");
            std::process::exit(1);
        }

        if let (Some(start), Some(end)) = (args.start_surah, args.end_surah) {
            if start > end {
                eprintln!("Error: --start-surah must be less than or equal to --end-surah");
                std::process::exit(1);
            }
        }

        let start_surah = args.start_surah.unwrap(); // safe to use unwrap
        let end_surah = args.end_surah.unwrap_or(start_surah);
        let _ = tafsir_html_generator::handler(start_surah, end_surah).await;
    }

    tracing::info!("Success");
    Ok(())
}
