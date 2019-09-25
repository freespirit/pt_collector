pub mod local_storage {
    use std::fs::{self, File};
    use std::io::{Result, Write, Error, ErrorKind};

    use super::super::PhotoStorage;
    use super::super::Photo;

    static mut FILE_SEQ_NUMBER: i32 = 1;

    pub struct LocalPhotoStorage {
        dir: String
    }

    impl LocalPhotoStorage {
        pub fn new(path: &String) -> Result<LocalPhotoStorage> {
            let dir = fs::create_dir_all(path)?;
            let lps = LocalPhotoStorage{dir: path.clone() };
            Ok(lps)
        }
    }

    impl PhotoStorage for LocalPhotoStorage {
        fn save_photo(&self, photo: &Photo) {
            unsafe {
                let id = FILE_SEQ_NUMBER;
                let file_name = format!("{}/{}.jpg", self.dir, id);

                print!("Saving {} to {:?}...", id, &file_name);

                let result = File::create(&file_name)
                    .and_then(|mut tmp_file: File| {
                        let photo_bytes_opt = &photo.bytes;
                        match photo_bytes_opt {
                            None => {Err(Error::new(ErrorKind::Other, "No bytes in photo!"))}
                            Some(bytes) => { tmp_file.write_all(bytes) }
                        }
                    });

                match result {
                    Err(e) => print!(" error {}!", e),
                    Ok(_) => print!(" success!")
                }

                println!();

                FILE_SEQ_NUMBER += 1;
            }
        }
    }
}