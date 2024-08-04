use reqwest::get;
// use rodio::{source::Source, Decoder, OutputStream, Sink};
use dataglass::app::App;
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::error::Error;
use std::io::BufReader;
const MFP_FEED: &str = "https://musicforprogramming.net/rss.xml";

pub async fn music_for_programming() -> Result<(), Box<dyn Error>> {
    let url = "https://musicforprogramming.net/rss.xml";
    let response = get(url).await.unwrap();

    if response.status().is_success() {
        let content = response.text().await?;
        let channel = Channel::read_from(content.as_bytes())?;

        println!("Title: {}", channel.title());
        println!("Description: {}", channel.description());

        for item in channel.items() {
            // let episode =
            println!("Item: {}", item.title().unwrap_or_default());
            println!("\n");
            println!("Item Categories: {:?}", item.categories());
            println!("\n");
            println!("Item Comments: {:?}", item.comments().unwrap());
            println!("\n");
            // println!("Item Content: {:?}", item.content().unwrap());
            println!("\n");
            println!("Item ItunesExt {:?}", item.itunes_ext().unwrap());
            println!("\n");
            println!("Item Pub date: {:?}", item.pub_date());
            println!("\n");
            println!("Link: {}", item.link().unwrap_or_default());
            println!("\n");
            println!("Description: {}", item.description().unwrap_or_default());
            println!("------");
        }
    } else {
        println!("Failed to fetch the RSS feed: HTTP {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let _ = music_for_programming().await.unwrap();

    Ok(())
}
