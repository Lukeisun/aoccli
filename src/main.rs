use reqwest;
fn main() {
    let resp = reqwest::blocking::get("https://google.com").unwrap();
    println!("{:#?}", resp.text());
}
