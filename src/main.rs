use std::env;
use pt_collector::Collector;

use pt_collector::storage::local_storage::LocalPhotoStorage;
use pt_collector::provider::flickr_photos_provider::FlickrCollector;

fn main() {
    let mut collector = create_collector();
    collector.collect();
}


fn create_collector() -> Collector {
    let photo_storage = LocalPhotoStorage::new(&String::from("tmp")).unwrap();

    let api_key = env::var("FLICKR_API_KEY")
        .expect("FLICKR_API_KEY env variable is not set!");
    let photo_provider = FlickrCollector::new(&api_key);

    Collector::new(Box::new(photo_provider), Box::new(photo_storage))
}
