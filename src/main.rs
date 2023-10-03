use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang",
        repo = "rust"
    );

    let client = reqwest::Client::new();

    let response = client
        .get(request_url)
        .header(USER_AGENT, "Hello Rust")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
