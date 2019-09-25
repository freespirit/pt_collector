pub mod storage;
pub mod provider;

pub trait PhotoStorage {
    fn save_photo(&self, photo: &Photo);
}

pub trait PhotoProvider {
    fn build_photos_metadata_list(&self) -> Vec<Photo>;
    fn get_photo_data(&self, photo: &mut Photo);
}

pub struct Photo {
    pub original_url: String,
    pub tags: Vec<String>,
    pub bytes: Option<Vec<u8>>,
}

pub struct Collector {
    photo_provider: Box<dyn PhotoProvider>,
    photo_storage: Box<dyn PhotoStorage>,
}

impl Collector {
    pub fn new(photo_provider: Box<dyn PhotoProvider>,
               photo_storage: Box<dyn PhotoStorage>) -> Collector {
        Collector { photo_provider, photo_storage }
    }

    pub fn collect(&mut self) {
        let mut photos = self.build_images_list();

        let filtered_photos = photos.iter_mut()
            .filter(|photo| self.filter_photo(photo));

        for photo in filtered_photos {
            self.get_photo_data(photo);
            self.save_photo(photo);
        }
    }

    fn build_images_list(&self) -> Vec<Photo> {
        self.photo_provider.build_photos_metadata_list()
    }

    fn get_photo_data(&self, photo: &mut Photo) {
        self.photo_provider.get_photo_data(photo);
    }

    fn save_photo(&self, photo: &Photo) {
        self.photo_storage.save_photo(photo);
    }

    fn filter_photo(&self, photo: &Photo) -> bool {
        if photo.tags.is_empty() {
            println!("No tags");
        }
        !photo.tags.is_empty()
    }
}
