#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate url;

use reqwest::Error;
use serde_json::Value;
use url::{Url, ParseError};


static FLICKR_ENDPOINT: &str = "https://api.flickr.com/";
static FLICKR_PATH: &str = "/services/rest";
static FLICKR_QUERY_METHOD: &str = "flickr.photos.search";

pub struct FlickrCollector {
    api_key: String
}

#[derive(Deserialize, Debug)]
struct Photo {
    id: String,
    owner: String,
    secret: String,
    server: String,
    farm: i32,
    title: String,
    ispublic: i8,
    isfriend: i8,
    isfamily: i8,
    tags: String
}

#[derive(Deserialize, Debug)]
struct Photos {
    page: i32,
    pages: i32,
    perpage: i32,
    total: String,
    photo: Vec<Photo>
}

#[derive(Deserialize, Debug)]
struct FlickrResponseJson {
    photos: Photos,
    stat: String
}

impl FlickrCollector {
    pub fn new(api_key: &str) -> FlickrCollector {
        let api_key: String = String::from(api_key);
        FlickrCollector { api_key }
    }

    /// Fetches a list of images that contain only ID and other metadata.
    /// No actual image data is downloaded.
    ///
    /// To get the image data use
    /// ```
    /// fetch_image()
    ///
    /// ```
    pub fn request_images(self) -> Vec<String> {
        let mut image_ids: Vec<String> = vec![];

        let url = self.build_search_url().expect("Failed to build search url");

//        println!("{:#?}", url);

        let mut response = reqwest::get(url.as_str()).unwrap();
        let text = response.text().unwrap();
        let json: Value = serde_json::from_str(&text).unwrap();
        let flickr_response_json: FlickrResponseJson = serde_json::from_value(json).unwrap();
//        println!("Response: {:?}", flickr_response_json);
        println!("Got response \"{}\". Found {} images.",
                 flickr_response_json.stat,
                 flickr_response_json.photos.total);

        let photos: Vec<Photo> = flickr_response_json.photos.photo;

        image_ids
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
        query.push_str(&format!("&{}", &make_query_param("extras", "tags")));


        query
    }
}

fn make_query_param(key: &str, value: &str) -> String {
    format!("{}={}", key, value)
}


#[cfg(test)]
mod tests {
    use crate::{FlickrCollector, FLICKR_PATH, FLICKR_QUERY_METHOD};
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
        let collector = FlickrCollector::new(API_KEY);
        let url = collector.build_search_url();

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
