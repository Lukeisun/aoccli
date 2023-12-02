use chrono::{self, Datelike};
use clap::Parser;
use dotenv::dotenv;
use reqwest::{self, header};
use select::document::Document;
use select::node::Node;
use select::predicate::Name;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::{exit, Command};
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(long)]
    day: Option<u32>,
    #[arg(long)]
    year: Option<i32>,
    #[arg(long, short)]
    submit: Option<bool>,
    #[arg(long, short)]
    part: Option<u8>,
}
// Problem Input
fn write_input_to_fs(client: &reqwest::blocking::Client, base_url: &str) {
    let resp = client.get(format!("{}/input", base_url)).send().unwrap();
    let text = resp.text().unwrap();
    let mut file = File::create("input.in").unwrap();
    file.write_all(text.as_bytes()).unwrap();
    println!("\tWrote input.in");
}
// Test Input
// First code block seems to usually be the example.
// Sometimes multiple examples given as well as visual examples.
// I'll just write all code blocks to files.
// and figure out which ones are valid as I'm working on it.
fn write_test_input_to_fs(client: &reqwest::blocking::Client, base_url: &str) {
    let resp = client.get(base_url).send().unwrap();
    let text = resp.text().unwrap();
    let document = Document::from(text.as_str());
    for (i, node) in document.find(Name("pre")).enumerate() {
        let file_name = format!("test_input{}.in", i);
        let mut file = File::create(file_name.clone()).unwrap();
        file.write_all(node.text().as_bytes()).unwrap();
        println!("\tWrote {file_name}");
    }
}
fn submit(base_url: String, session_cookie: String, part: String) {
    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", session_cookie.parse().unwrap());
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let url = format!("{}/answer", base_url);
    let mut input = String::new();
    let stdin = std::io::stdin();
    let _input_size = stdin.read_line(&mut input);
    let params = [("level", part.as_str()), ("answer", input.trim())];
    println!("Submitting...");
    let resp = client.post(url).form(&params).send().unwrap();
    let text = resp.text().unwrap();
    let document = Document::from(text.as_str());
    let main: Vec<Node> = document.find(Name("main")).collect();
    let submission_text = main[0].text();
    if submission_text.contains("That's the right answer!") {
        tree();
        println!("Nice you got it right!");
    } else {
        println!("Wrong :( Try again in a minute");
    }
}
fn run(base_url: String, session_cookie: String) {
    // dont care if it doesnt open
    let _ = Command::new("xdg-open").arg(&base_url).output();
    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", session_cookie.parse().unwrap());
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    println!("Writing files to cwd");
    write_input_to_fs(&client, &base_url);
    write_test_input_to_fs(&client, &base_url);
    println!("Goodbye and good luck!");
}
fn main() {
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
    if let Some(_) = cli.submit {
        let part = cli.part.expect("No part provided");
        if part != 1 && part != 2 {
            println!("Part must be 1 or 2");
            exit(1);
        }
        submit(base_url.clone(), session_cookie.clone(), part.to_string());
    } else {
        run(base_url.clone(), session_cookie.clone());
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
