mod config;
mod store;
mod ui;

use config::*;
use store::*;

struct App {
    config: ReaderConfig,
    store: Store,
}

impl App {
    fn new() -> App {
        App {
            config: ReaderConfig::new(),
            store: Store{},
        }
    }

    fn init(&mut self) {
        self.config = load_config();
    }
}

fn main() {
    let mut app = App::new();

    app.init();
}
