pub mod flickr_photos_provider {
    use reqwest;
    use serde::{self, Deserialize};
    use serde_json::Value;
    use std::io::Read;
    use std::thread::sleep;
    use std::time::Duration;
    use url::{Url, ParseError};

    pub use crate::PhotoProvider;
    pub use crate::Photo;

    static FLICKR_ENDPOINT: &str = "https://api.flickr.com/";
    static FLICKR_PATH: &str = "/services/rest";
    static FLICKR_QUERY_METHOD: &str = "flickr.photos.search";
    //https://www.flickr.com/services/api/explore/flickr.photos.search

    pub struct FlickrCollector {
        pub api_key: String
    }

    #[derive(Deserialize, Debug)]
    struct FlickrPhoto {
        id: String,
        server: String,
        farm: i32,
        ispublic: i8,
        tags: String,
        url_o: String,
    }

    #[derive(Deserialize, Debug)]
    struct Photos {
        page: i32,
        pages: i32,
        perpage: i32,
        total: String,
        photo: Vec<FlickrPhoto>,
    }

    #[derive(Deserialize, Debug)]
    struct FlickrResponseJson {
        photos: Photos,
        stat: String,
    }

    impl FlickrCollector {
        pub fn new(api_key: &str) -> FlickrCollector {
            let api_key: String = String::from(api_key);
            FlickrCollector { api_key }
        }
    }

    impl PhotoProvider for FlickrCollector {
        fn build_photos_metadata_list(&self) -> Vec<Photo> {
            let mut all_photos: Vec<FlickrPhoto> = vec![];

            let mut _page = 0;
            let mut has_next_page = true;
            let api_key = &self.api_key;

            while has_next_page {
                let request_builder = FlickrRequestBuilder::new(api_key.clone(), _page);
                let url = request_builder.build_search_url().expect("Failed to build search url");
                let mut response = reqwest::get(url.as_str()).unwrap();
                let text = response.text().unwrap();
                let json: Value = serde_json::from_str(&text).unwrap();
                let flickr_response_json: FlickrResponseJson = serde_json::from_value(json).unwrap();

                let photos_response = flickr_response_json.photos;
                println!("Got response \"{}\". Page: {}/{}.",
                         flickr_response_json.stat,
                         photos_response.page,
                         photos_response.pages);
//            println!("Got response: {:#?}", photos_response);

                let _photos: Vec<FlickrPhoto> = photos_response.photo;
                all_photos.extend(_photos);

                println!("\tphotos collections: {}/{}", all_photos.len(), all_photos.capacity());

                _page = photos_response.page + 1;
                has_next_page = _page < photos_response.pages;
                sleep(Duration::from_secs(1)); //Flickr only allow 3600 requests per hour...

                if all_photos.len() >= 1000 { //todo max photos count
                    break;
                }
            }

            all_photos.iter().map(|flickr_photo: &FlickrPhoto| {
                Photo {
                    original_url: flickr_photo.url_o.clone(),
                    bytes: None,
                    tags:flickr_photo.tags.split_whitespace()
                        .map(|str| String::from(str))
                        .collect()
                }
            }).collect()
        }

        fn get_photo_data(&self, photo: &mut Photo) {
            let target = &photo.original_url.clone();
            let mut response = reqwest::get(target).unwrap();
            let mut photo_bytes: Vec<u8> = vec![];

            let result = response.read_to_end(&mut photo_bytes);

            match result {
                Err(_) => println!("Failed to read response {:#?}", response),
                Ok(_) => {
                    photo.bytes = Some(photo_bytes);
                }
            }

            sleep(Duration::from_millis(1001));//Flickr only allow 3600 requests per hour... allow some buffer as well
        }
    }

    struct FlickrRequestBuilder {
        api_key: String,
        page: i32,
    }

    impl FlickrRequestBuilder {
        fn new(api_key: String, page: i32) -> FlickrRequestBuilder {
            FlickrRequestBuilder { api_key, page }
        }

        fn build_search_url(self) -> Result<Url, ParseError> {
            let mut url = Url::parse(FLICKR_ENDPOINT)?;
            url.set_path(FLICKR_PATH);
            url.set_query(Some(&self.build_query()));
            Ok(url)
        }

        fn build_query(self) -> String {
            let mut query = String::new();

            query.push_str(&format!("{}", &make_query_param("method", FLICKR_QUERY_METHOD)));
            query.push_str(&format!("&{}", &make_query_param("api_key", &self.api_key)));
            query.push_str(&format!("&{}", &make_query_param("license", "4")));
            query.push_str(&format!("&{}", &make_query_param("format", "json")));
            query.push_str(&format!("&{}", &make_query_param("nojsoncallback", "1")));
            query.push_str(&format!("&{}", &make_query_param("extras", "tags,url_o")));
            query.push_str(&format!("&{}", &make_query_param("per_page", "500")));
            query.push_str(&format!("&{}", &make_query_param("page", &format!("{}", self.page))));


            query
        }
    }

    fn make_query_param(key: &str, value: &str) -> String {
        format!("{}={}", key, value)
    }


    #[cfg(test)]
    mod tests {
        use crate::provider::flickr_photos_provider::{FlickrCollector, FlickrRequestBuilder,
                                                      FLICKR_PATH, FLICKR_QUERY_METHOD};
        use super::make_query_param;
        use std::borrow::Cow;

        const API_KEY: &'static str = "Test API Key";

        #[test]
        fn new_collector_uses_key() {
            let collector = FlickrCollector::new(API_KEY);

            assert_eq!(API_KEY, collector.api_key);
        }

        #[test]
        fn build_search_url() {
            let request_builder = FlickrRequestBuilder::new(String::from(API_KEY), 0);
            let url = request_builder.build_search_url();

            assert!(url.is_ok());
            let url = url.unwrap();

            assert_eq!(url.cannot_be_a_base(), false);

            assert_eq!(url.path(), FLICKR_PATH);

            let mut query_pairs = url.query_pairs();

            let param_method = Some((Cow::Borrowed("method"), Cow::Borrowed(FLICKR_QUERY_METHOD)));
            assert_eq!(query_pairs.next(), param_method);

            let param_api_key = Some((Cow::Borrowed("api_key"), Cow::Borrowed(API_KEY)));
            assert_eq!(query_pairs.next(), param_api_key);

            let param_license = Some((Cow::Borrowed("license"), Cow::Borrowed("4")));
            assert_eq!(query_pairs.next(), param_license);
        }

        #[test]
        fn make_query_param_makes() {
            assert_eq!("=", make_query_param("", ""));
            assert_eq!("k=v", make_query_param("k", "v"));
            assert_ne!("v=k", make_query_param("k", "v"));
        }
    }
}