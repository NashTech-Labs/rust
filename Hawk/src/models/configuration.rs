use std::env::args;

pub struct Configuration {
    pub bucket_name: String,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub local_file_path: String,
    pub bucket_file_name: String,
}

impl Configuration {
    ///environment variables allocation into structure
    #[cfg_attr(tarpaulin, skip)]
    pub fn config() -> Configuration {
        let bucket_name: String = args().nth(1).unwrap();
        let access_key: Option<String> = args().nth(2);
        let secret_key: Option<String> = args().nth(3);
        let local_file_path: String = args().nth(4).unwrap();
        let bucket_file_name: String = args().nth(5).unwrap();
        Configuration {
            bucket_name,
            access_key,
            secret_key,
            local_file_path,
            bucket_file_name,
        }
    }
}
