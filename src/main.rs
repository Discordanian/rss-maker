use clap::Parser;
use std::fs;

use chrono::prelude::*;
use chrono::{Duration, Utc};
use glob::glob;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL to prefix with
    #[arg(short, long)]
    url: String,
}

fn dirlist() -> Vec<String> {
    let mut retval = Vec::new();
    for entry in glob("./*mp3").expect("Failed to find mp3 files in current directory") {
        match entry {
            Ok(path) => retval.push(path.display().to_string()),
            Err(e) => println!("{:?}", e),
        }
    }
    retval.sort();
    retval
}

fn end() -> String {
    let e = r#"
  </channel>
</rss>
"#;
    e.to_string()
}
fn start(url: String) -> String {
    let s = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0"
    xmlns:googleplay="http://www.google.com/schemas/play-podcasts/1.0"
    xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
  <channel>
    <title>Kurt Audio Book</title>
    <googleplay:owner>kurt@tangentialcold.com</googleplay:owner>
    <googleplay:author>Tangential Cold</googleplay:author>
    <description>An Audio Book Work Around</description>
    <googleplay:image href="http://dev.tangentialcold.com/TCS.jpg"/>
    <language>en-us</language>
    <link>LINK</link>
"#;
    s.replace("LINK", &url)
}
/*
    <item>
      <title>Top 10 myths about caring for a zebra</title>
      <description>Here are the top 10 misunderstandings about the care, feeding, and breeding of these lovable striped animals.</description>
      <pubDate>Tue, 14 Mar 2017 12:00:00 GMT</pubDate>
      <enclosure url="https://www.example.com/podcasts/dafnas-zebras/audio/toptenmyths.mp3"
                 type="audio/mpeg" length="34216300"/>
      <itunes:duration>30:00</itunes:duration>
      <guid isPermaLink="false">dzpodtop10</guid>
    </item>
*/
/* pubDate is RFC 2822.  utc.to_rfc2822() */
fn item(url: String, offset: usize, entry: String) -> String {
    let mut dt = Utc.with_ymd_and_hms(1988, 5, 1, 12, 1, 2).unwrap();
    dt += Duration::days(offset as i64);

    let item = r#"
    <item>
      <title>TITLE</title>
      <description>DESCRIPTION</description>
      <pubDate>PUBDATE</pubDate>
      <enclosure url="URL/ENTRY"
                 type="audio/mpeg"/>
      <guid isPermaLink="false">GUID</guid>
    </item>
    "#;
    item.replace("TITLE", &entry)
        .replace("DESCRIPTION", &entry)
        .replace("GUID", &format!("{}", dt.timestamp()))
        .replace("URL", &url)
        .replace("PUBDATE", &dt.to_rfc2822())
        .replace("ENTRY", &entry)
}

fn items(url: String, entries: Vec<String>) -> String {
    let mut retval = String::new();
    for (i, entry) in entries.iter().enumerate() {
        retval += &item(url.clone(), i, entry.to_string()).to_string();
    }
    retval
}

fn main() {
    let args = Args::parse();

    println!("ARGS URL : {}", args.url);
    let x = dirlist();
    let rssfile = "rss.xml";

    let mut rssdata = String::new();

    rssdata += &start(args.url.clone());
    rssdata += &items(args.url, x);
    rssdata += &end();

    println!("RSS DATA \n{}", rssdata);
    fs::write(rssfile, rssdata).unwrap_or_else(|_| panic!("Error writing to {}", rssfile));
}
