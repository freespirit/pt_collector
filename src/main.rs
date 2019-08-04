use std::env;

extern crate pt_collector;
use pt_collector::FlickrCollector;

fn main() {
    let config = create_configuration();

    let collector = FlickrCollector::new(&config.api_key);
    collector.request_images();
}

struct Config {
    api_key: String
}

fn create_configuration() -> Config {
    let api_key = env::var("FLICKR_API_KEY")
        .expect("FLICKR_API_KEY env variable is not set!");
    Config { api_key }
}
