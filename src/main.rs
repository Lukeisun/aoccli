use chrono::{self, Datelike};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::{self, header};
use select::document::Document;
use select::node::Node;
use select::predicate::Name;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, global = true)]
    day: Option<u32>,
    #[arg(long, global = true)]
    year: Option<i32>,
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    Submit {
        #[arg(long, short)]
        part: Option<u8>,
    },
    S {
        #[arg(long, short)]
        part: Option<u8>,
    },
}
// Getting inputs
async fn run(base_url: String, session_cookie: String) {
    // dont care if it doesnt open
    let _ = Command::new("xdg-open").arg(&base_url).output();
    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", session_cookie.parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    println!("Writing files to cwd");
    let _ = write_test_input_to_fs(&client, &base_url).await;
    let _ = write_input_to_fs(&client, &base_url).await;
    println!("Goodbye and good luck!");
}
// Problem Input
async fn write_input_to_fs(client: &reqwest::Client, base_url: &str) {
    let resp = client
        .get(format!("{}/input", base_url))
        .send()
        .await
        .unwrap();
    let text = resp.text().await.unwrap();
    let mut file = File::create("input.in").unwrap();
    file.write_all(text.as_bytes()).unwrap();
    println!("\tWrote input.in");
}
// Test Input
// First code block seems to usually be the example.
// Sometimes multiple examples given as well as visual examples.
// I'll just write all code blocks to files.
// and figure out which ones are valid as I'm working on it.
async fn write_test_input_to_fs(client: &reqwest::Client, base_url: &str) {
    let resp = client.get(base_url).send().await.unwrap();
    let text = resp.text().await.unwrap();
    let document = Document::from(text.as_str());
    for (i, node) in document.find(Name("pre")).enumerate() {
        let file_name = format!("test_input{}.in", i);
        let mut file = File::create(file_name.clone()).unwrap();
        file.write_all(node.text().as_bytes()).unwrap();
        println!("\tWrote {file_name}");
    }
}
// Submitting
async fn submit(base_url: String, session_cookie: String, part: u8) {
    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", session_cookie.parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let url = format!("{}/answer", base_url);
    let mut input = String::new();
    let stdin = std::io::stdin();
    let _input_size = stdin.read_line(&mut input);
    let is_successful = post_answer(&url, &client, part.to_string(), &input).await;
    if !is_successful && part == 1 {
        println!("Seems like you might have already submitted part 1\nRetrying...");
        let retry = post_answer(&url, &client, (part + 1).to_string(), &input).await;
        if retry == false {
            println!("Looks like you did both parts? Or something else is wrong :)");
        }
    }
    println!("Finished Submitting!");
}
async fn post_answer(
    url: &String,
    client: &reqwest::Client,
    part: String,
    answer: &String,
) -> bool {
    let params = [("level", part.as_str()), ("answer", answer.trim())];
    println!("Submitting...");
    let resp = client
        .post(url)
        .form(&params)
        .send()
        .await
        .expect("error submitting");
    let text = resp.text().await.expect("error getting text");
    let document = Document::from(text.as_str());
    let main: Vec<Node> = document.find(Name("main")).collect();
    let submission_text = main[0].text();
    if submission_text.contains("That's the right answer!") {
        tree();
        println!("Nice you got it right!");
    } else if submission_text.contains("You don't seem") {
        return false;
    } else {
        println!("{submission_text}");
    }
    true
}
#[tokio::main]
async fn main() {
    let date = chrono::offset::Local::now();
    let cli = Cli::parse();
    let day = cli.day.unwrap_or(date.day());
    let year = cli.year.unwrap_or(date.year());
    // Think about doing some sort of .config type thing
    dotenv().ok();
    let session = env::var("SESSION").expect("No session env variable set");
    let session_cookie = "session=".to_string() + &session;
    let base_url = format!("https://adventofcode.com/{}/day/{}", year, day);
    println!("\x1B[32mAdvent of Code Year {} - Day {}\x1B[0m", year, day);
    println!("{}", base_url);
    match &cli.command {
        Some(Commands::Submit { part }) | Some(Commands::S { part }) => {
            let part = part.unwrap_or(1);
            let _ = submit(base_url, session_cookie, part).await;
        }
        None => run(base_url, session_cookie).await,
    }
}
fn tree() {
    let art = r"
                     /\
                    <  >
                     \/
                     /\
                    /  \
                   /++++\
                  /  ()  \
                  /      \
                 /~`~`~`~`\
                /  ()  ()  \
                /          \
               /*&*&*&*&*&*&\
              /  ()  ()  ()  \
              /              \
             /++++++++++++++++\
            /  ()  ()  ()  ()  \
            /                  \
           /~`~`~`~`~`~`~`~`~`~`\
          /  ()  ()  ()  ()  ()  \
          /*&*&*&*&*&*&*&*&*&*&*&\
         /                        \
        /,.,.,.,.,.,.,.,.,.,.,.,.,.\
                   |   |
                  |`````|
                  \_____/
";
    println!("{}", art);
}
