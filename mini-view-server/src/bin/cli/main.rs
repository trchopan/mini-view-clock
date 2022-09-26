use chrono::Utc;
use clap::{Parser, Subcommand};
use mini_view_server::{domain::View, infrastructure::AuthRepo};

/// Client to send the command with given secret
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The endpoint to send request. Example: https://example.com:5001
    #[clap(long, value_parser)]
    endpoint: String,

    /// Secret string
    #[clap(long, value_parser)]
    secret: String,

    /// Secret string
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    View { view: View },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let auth_repo = AuthRepo {
        secret: args.secret,
    };

    match args.command {
        Command::View { view } => {
            let client = reqwest::Client::new();
            let timestamp = Utc::now().timestamp();
            let message = format!("{}/{}", view.to_string(), timestamp);
            let token = auth_repo.sign_message(&message);
            // /command/view/{view}/{timestamp}/{token}
            let url = format!(
                "{}/command/view/{}/{}/{}",
                args.endpoint,
                view.to_string(),
                timestamp,
                token
            );
            let res_text = client.put(url.clone()).send().await?.text().await?;
            println!("{:?}", res_text);
        }
    }

    Ok(())
}
