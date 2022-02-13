use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use dirs::home_dir;

fn config_dir() -> PathBuf {
    home_dir().unwrap().join(".rssrs")
}

fn config_file_path() -> PathBuf {
    config_dir().join(Path::new("config.yml"))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReaderConfig {
    pub feeds: Vec<FeedConfig>,
}

impl ReaderConfig {
    pub fn new() -> ReaderConfig {
        ReaderConfig {
            feeds: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedConfig {
    url: String,
}

pub fn load_config() -> ReaderConfig {
    let s = fs::read_to_string(config_file_path());
    match s.ok() {
        Some(s) => serde_yaml::from_str(s.as_str()).unwrap(),
        None => {
            // Create a config file
            let cfg = ReaderConfig::new(); 
            let s = serde_yaml::to_string(&cfg).unwrap();
            fs::create_dir_all(config_dir()).unwrap();
            println!("Writing new config to {}",
                     config_file_path().to_str().unwrap());
            fs::write(config_file_path(), s).unwrap();
            cfg
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_config() {
        let s = "
feeds:
    - url: http://example.com
        ";
        let cfg: ReaderConfig = serde_yaml::from_str(s).unwrap();        
        assert_eq!(cfg.feeds.get(0).unwrap().url, "http://example.com");
    }
}
