use super::config::ReaderConfig;
use feed_rs::parser;

pub struct Store {}

impl Store {
    pub fn fetch(&self, cfg: ReaderConfig) {
        for f in cfg.feeds {
            let data = reqwest::blocking::get(f.url).unwrap();
            let feed = parser::parse(data).unwrap();
            println!("Feed: {:?}", feed);
        }
    }
}
