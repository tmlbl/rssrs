mod config;

use config::*;

fn main() {
   let cfg = load_config();
   println!("Config: {:?}", cfg);
}
