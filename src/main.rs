use std::{env, fs, path::Path};

use anyhow::{Context, Result};
use clap::{arg, command, value_parser};

use sgpt::openai::call_gpt;

const TOKEN_PATH: &str = ".sgpt/token";

#[tokio::main]
async fn main() -> Result<()> {
    // configure the cli
    let matches = command!()
        .arg(
            arg!(query: [QUERY])
                .num_args(1..)
                .last(true)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    // parse the text from the command line, everything after `--`
    let query = matches
        .get_many::<String>("query")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default()
        .iter()
        .map(|val| val.as_str().to_owned())
        .collect::<Vec<String>>()
        .join(" ");

    if query.is_empty() {
        println!("Please provide a query");
        return Ok(());
    }

    Ok(println!("{}", call_gpt(&get_token()?, &query).await?))
}

// fetch the token from the filesystem
fn get_token() -> Result<String> {
    let home_dir = env::var("HOME")?;
    let token_path = Path::new(&home_dir).join(TOKEN_PATH);
    let token = fs::read_to_string(&token_path).context(format!(
        "Failed to read token from {}",
        token_path.to_string_lossy()
    ))?;
    let token = token.strip_suffix('\n').unwrap_or(&token);
    Ok(token.to_owned())
}
