use dotenv::dotenv;
use reqwest::{self, header};
use select::document::Document;
use select::predicate::Name;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
// want usage to be like aoc [day] [year] and aoc submit [day] year
// maybe day can be inferred from cwd, maybe year somehow too
// ideally some sort of config file?
// Problem Input
fn write_input_to_fs(client: &reqwest::blocking::Client, base_url: String) {
    let resp = client.get(format!("{}/input", base_url)).send().unwrap();
    let text = resp.text().unwrap();
    let mut file = File::create("input.in").unwrap();
    file.write_all(text.as_bytes()).unwrap();
}
// Test Input
// First code block seems to usually be the example.
// Sometimes multiple examples given as well as visual examples.
// I'll just write all code blocks to files.
// and figure out which ones are valid as I'm working on it.
fn write_test_input_to_fs(client: &reqwest::blocking::Client, base_url: String) {
    let resp = client.get(base_url).send().unwrap();
    let text = resp.text().unwrap();
    let document = Document::from(text.as_str());
    for (i, node) in document.find(Name("pre")).enumerate() {
        let file_name = format!("test_input{}.in", i);
        let mut file = File::create(file_name).unwrap();
        file.write_all(node.text().as_bytes()).unwrap();
    }
}
fn main() {
    // TODO: do some testing on how this works when I build it.
    // Think about doing some sort of .config type thing
    dotenv().ok();
    let session = env::var("SESSION").expect("No session env variable set");
    let session_cookie = "session=".to_string() + &session;
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let year = &args[2];
    let base_url = format!("https://adventofcode.com/{}/day/{}", year, day);
    println!("\x1B[32mAdvent of Code Year {} - Day {}\x1B[0m", year, day);
    println!("{}", base_url);
    // Dont care if it doesn't open :)
    let _ = Command::new("xdg-open").arg(base_url.clone()).output();
    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", session_cookie.parse().unwrap());
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    write_input_to_fs(&client, base_url.clone());
    write_test_input_to_fs(&client, base_url.clone())
}
